use crate::errors::ResponseDataError;
use crate::inputs::Grab;
use base64::Engine;
use base64::engine::general_purpose;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

trait ToHTTPString<'a> {
    fn to_http_string(&'a self) -> String;
}

#[derive(Clone, Debug)]
struct StatusCodeAndHeaders<'a> {
    status_code: StatusCode,
    headers: &'a HashMap<String, String>,
}

impl<'a> From<&'a ResponseData> for StatusCodeAndHeaders<'a> {
    fn from(value: &'a ResponseData) -> Self {
        Self {
            status_code: value.status_code,
            headers: value.headers(),
        }
    }
}

impl<'a> ToHTTPString<'a> for StatusCodeAndHeaders<'a> {
    fn to_http_string(&'a self) -> String {
        format!(
            "{}\n{}",
            self.status_code,
            headers_to_http_string(self.headers)
        )
    }
}

#[derive(Clone, Debug)]
struct StatusCodeAndBody<'a> {
    status_code: StatusCode,
    body: &'a str,
}

impl<'a> From<&'a ResponseData> for StatusCodeAndBody<'a> {
    fn from(value: &'a ResponseData) -> Self {
        Self {
            status_code: value.status_code(),
            body: value.body(),
        }
    }
}

impl<'a> ToHTTPString<'a> for StatusCodeAndBody<'a> {
    fn to_http_string(&'a self) -> String {
        format!("{}\n\n{}", self.status_code, self.body)
    }
}

#[derive(Clone, Debug, Serialize)]
struct HeadersAndBody<'a> {
    headers: &'a HashMap<String, String>,
    body: &'a str,
}

impl<'a> From<&'a ResponseData> for HeadersAndBody<'a> {
    fn from(value: &'a ResponseData) -> Self {
        Self {
            headers: value.headers(),
            body: value.body(),
        }
    }
}

impl<'a> ToHTTPString<'a> for HeadersAndBody<'a> {
    fn to_http_string(&'a self) -> String {
        format!("{}\n\n{}", headers_to_http_string(self.headers), self.body)
    }
}

#[derive(Clone, Debug)]
pub struct ResponseData {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: String,
}

impl ResponseData {
    pub fn new(status_code: StatusCode, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn status_code_u16(&self) -> u16 {
        self.status_code.as_u16()
    }

    pub fn status_code_string(&self) -> String {
        self.status_code_u16().to_string()
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn new_from_borrowed(
        status_code: StatusCode,
        headers: HashMap<&str, &str>,
        body: &str,
    ) -> Self {
        let mut headers_owned: HashMap<String, String> = HashMap::new();
        for (k, v) in headers {
            headers_owned.insert(k.into(), v.into());
        }

        Self {
            status_code,
            headers: headers_owned,
            body: body.into(),
        }
    }

    pub async fn try_from_response(response: Response) -> Result<Self, ResponseDataError> {
        Ok(Self {
            status_code: response.status(),
            headers: build_response_headers(response.headers()),
            body: build_body(response).await?,
        })
    }

    fn to_full_http_string(&self) -> String {
        format!(
            "{}\n{}\n\n{}",
            self.status_code,
            headers_to_http_string(self.headers()),
            self.body(),
        )
    }

    // TODO - Dont convert to structs here just use methods
    pub fn to_http_string(&self, grab: Grab) -> String {
        let output = match grab {
            Grab::Status => self.status_code.to_string(),
            Grab::Headers => headers_to_http_string(self.headers()),
            Grab::Body => self.body.clone(),
            Grab::StatusCodeAndHeaders => StatusCodeAndHeaders::from(self).to_http_string(),
            Grab::StatusCodeAndBody => StatusCodeAndBody::from(self).to_http_string(),
            Grab::HeadersAndBody => HeadersAndBody::from(self).to_http_string(),
            Grab::Full => self.to_full_http_string(),
            Grab::StatusCode => self.status_code_string(),
        };

        output.trim().to_string()
    }

    // Note - printing body as part of a map/json may not work in a pretty fashion
    // Because we are alwasy printing a map (e.g. body is a string).
    // Body might actually have to be a serde_json::Value for a nice print
    pub fn to_json_string(&self, grab: Grab, pretty: bool) -> Result<String, ResponseDataError> {
        match grab {
            Grab::Status => self.status_code_to_json(pretty),
            Grab::Headers => self.headers_to_json(pretty),
            Grab::Body => self.body_to_json(pretty),
            Grab::StatusCodeAndHeaders => self.status_code_and_headers_as_json(pretty),
            Grab::StatusCodeAndBody => self.status_code_and_body_as_json(pretty),
            Grab::HeadersAndBody => self.headers_and_body_as_json(pretty),
            Grab::Full => self.to_json(pretty),
            Grab::StatusCode => Ok(self.status_code_string()),
        }
    }

    #[inline]
    fn status_code_to_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        to_json_string(&json!({"status_code": self.status_code_u16()}), pretty)
    }

    #[inline]
    fn headers_to_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        to_json_string(&json!({"headers": self.headers()}), pretty)
    }

    fn body_to_map<'a>(&'a self) -> BodyToMapReturn<'a> {
        match serde_json::from_str::<serde_json::Value>(self.body()) {
            Ok(as_json) => BodyToMapReturn::Json(as_json),
            Err(_) => BodyToMapReturn::Str(self.body()),
        }
    }

    fn body_to_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        let json = match self.body_to_map() {
            BodyToMapReturn::Json(body) => json!({"body": body}),
            BodyToMapReturn::Str(body) => json!({"body": body}),
        };
        to_json_string(&json, pretty)
    }

    #[inline]
    fn status_code_and_headers_as_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        to_json_string(
            &json!({"status_code": self.status_code_u16(), "headers": self.headers()}),
            pretty,
        )
    }

    fn status_code_and_body_as_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        let json = match self.body_to_map() {
            BodyToMapReturn::Json(body) => {
                json!({
                    "status_code": self.status_code_u16(),
                    "body": body,
                })
            }
            BodyToMapReturn::Str(body) => {
                json!({
                    "status_code": self.status_code_u16(),
                    "body": body,
                })
            }
        };
        to_json_string(&json, pretty)
    }

    fn headers_and_body_as_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        let json = match self.body_to_map() {
            BodyToMapReturn::Json(body) => {
                json!({
                    "headers": self.headers(),
                    "body": body
                })
            }
            BodyToMapReturn::Str(body) => {
                json!({
                    "headers": self.headers(),
                    "body": body
                })
            }
        };
        to_json_string(&json, pretty)
    }

    fn to_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        let json = match self.body_to_map() {
            BodyToMapReturn::Json(body) => {
                json!({
                    "status_code": self.status_code_u16(),
                    "headers": self.headers(),
                    "body": body
                })
            }
            BodyToMapReturn::Str(body) => {
                json!({
                    "status_code": self.status_code_u16(),
                    "headers": self.headers(),
                    "body": body
                })
            }
        };
        to_json_string(&json, pretty)
    }
}

enum BodyToMapReturn<'a> {
    Json(serde_json::Value),
    Str(&'a str),
}

fn build_response_headers(header_map: &HeaderMap) -> HashMap<String, String> {
    let mut as_hash_map: HashMap<String, String> = HashMap::new();

    header_map.iter().for_each(|(k, v)| {
        if let Some(header_override) =
            as_hash_map.insert(k.as_str().into(), header_value_to_string(v))
        {
            warn!("Overriding header {}", header_override)
        }
    });

    as_hash_map
}

async fn build_body(response: Response) -> Result<String, ResponseDataError> {
    let as_bytes = response
        .bytes()
        .await
        .map_err(|e| ResponseDataError::Build(e.to_string()))?
        .to_vec();

    match std::str::from_utf8(&as_bytes) {
        Ok(str_) => Ok(str_.to_string()),
        Err(_) => {
            warn_to_console("Could not convert response body to a String. Encoding as base64.");
            Ok(encode_as_base_64(&as_bytes))
        }
    }
}

fn header_value_to_string(header_value: &HeaderValue) -> String {
    match header_value.to_str() {
        Ok(str_) => str_.into(),
        Err(_) => {
            warn_to_console("Found invalid utf-8 header value. Encoding as base64.");
            encode_as_base_64(header_value.as_bytes())
        }
    }
}

fn encode_as_base_64(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

fn headers_to_http_string(headers: &HashMap<String, String>) -> String {
    let mut http_string = String::new();

    headers.iter().for_each(|(k, v)| {
        http_string.push_str(&format!("{}: {}\n", k, v));
    });

    match http_string.strip_suffix("\n") {
        Some(stripped) => stripped.to_string(),
        None => http_string,
    }
}

fn warn_to_console(warning: &str) {
    eprintln!("[WARNING] {warning}")
}

fn to_json_string<T: Serialize>(value: &T, pretty: bool) -> Result<String, ResponseDataError> {
    let result = match pretty {
        true => serde_json::to_string_pretty(value),
        false => serde_json::to_string(value),
    };
    result.map_err(|e| ResponseDataError::Serialize(e.to_string()))
}

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

    #[inline]
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    #[inline]
    pub fn status_code_u16(&self) -> u16 {
        self.status_code.as_u16()
    }

    #[inline]
    pub fn status_code_string(&self) -> String {
        self.status_code_u16().to_string()
    }

    #[inline]
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    #[inline]
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

    pub fn to_http_string(&self, grab: Grab) -> String {
        let output = match grab {
            Grab::Status => self.status_code.to_string(),
            Grab::Headers => self.headers_to_http_string(),
            Grab::Body => self.body.clone(),
            Grab::StatusCodeAndHeaders => self.status_code_and_headers_to_http_string(),
            Grab::StatusCodeAndBody => self.status_code_and_body_to_http_string(),
            Grab::HeadersAndBody => self.headers_and_body_to_http_string(),
            Grab::Full => self.to_full_http_string(),
            Grab::StatusCode => self.status_code_string(),
        };
        output.trim().to_string()
    }

    pub fn to_json_string(&self, grab: Grab, pretty: bool) -> Result<String, ResponseDataError> {
        match grab {
            Grab::Status => self.status_code_to_json(pretty),
            Grab::Headers => self.headers_to_json(pretty),
            Grab::Body => self.body_to_json(pretty),
            Grab::StatusCodeAndHeaders => self.status_code_and_headers_to_json(pretty),
            Grab::StatusCodeAndBody => self.status_code_and_body_to_json(pretty),
            Grab::HeadersAndBody => self.headers_and_body_as_json(pretty),
            Grab::Full => self.to_json(pretty),
            Grab::StatusCode => Ok(self.status_code_string()),
        }
    }

    #[inline]
    fn status_code_to_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        to_json_string(&json!({"status_code": self.status_code_u16()}), pretty)
    }

    fn headers_to_http_string(&self) -> String {
        let mut http_string = String::new();

        self.headers.iter().for_each(|(k, v)| {
            http_string.push_str(&format!("{}: {}\n", k, v));
        });

        match http_string.strip_suffix("\n") {
            Some(stripped) => stripped.to_string(),
            None => http_string,
        }
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
    fn status_code_and_headers_to_http_string(&self) -> String {
        format!("{}\n{}", self.status_code, self.headers_to_http_string())
    }

    #[inline]
    fn status_code_and_headers_to_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
        to_json_string(
            &json!({"status_code": self.status_code_u16(), "headers": self.headers()}),
            pretty,
        )
    }

    #[inline]
    fn status_code_and_body_to_http_string(&self) -> String {
        format!("{}\n\n{}", self.status_code, self.body)
    }

    fn status_code_and_body_to_json(&self, pretty: bool) -> Result<String, ResponseDataError> {
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

    #[inline]
    fn headers_and_body_to_http_string(&self) -> String {
        format!("{}\n\n{}", self.headers_to_http_string(), self.body)
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

    #[inline]
    fn to_full_http_string(&self) -> String {
        format!(
            "{}\n{}\n\n{}",
            self.status_code,
            self.headers_to_http_string(),
            self.body
        )
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

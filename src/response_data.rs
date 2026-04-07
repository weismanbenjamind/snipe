use crate::errors::ResponseDataError;
use crate::inputs::Grab;
use base64::Engine;
use base64::engine::general_purpose;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct StatusCodeAndHeaders<'a> {
    status_code: u16,
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

#[derive(Debug, Serialize)]
struct StatusCodeAndBody<'a> {
    status_code: u16,
    body: &'a str,
}

impl<'a> From<&'a ResponseData> for StatusCodeAndBody<'a> {
    fn from(value: &'a ResponseData) -> Self {
        Self {
            status_code: value.status_code,
            body: value.body(),
        }
    }
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct ResponseData {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl ResponseData {
    pub fn new(status_code: StatusCode, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            headers,
            body,
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
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
            status_code: status_code.as_u16(),
            headers: headers_owned,
            body: body.into(),
        }
    }

    pub async fn try_from_response(response: Response) -> Result<Self, ResponseDataError> {
        Ok(Self {
            status_code: response.status().as_u16(),
            headers: build_response_headers(response.headers()),
            body: response
                .text()
                .await
                .map_err(|e| ResponseDataError::Build(e.to_string()))?,
        })
    }

    pub fn to_json_string(&self, grab: Grab, pretty: bool) -> Result<String, ResponseDataError> {
        match grab {
            Grab::Full => to_json(&self, pretty),
            Grab::StatusCode => Ok(self.status_code.to_string()),
            Grab::Headers => to_json(&self.headers(), pretty),
            Grab::Body => to_json(&self.body, pretty),
            Grab::StatusCodeAndHeaders => to_json(&StatusCodeAndHeaders::from(self), pretty),
            Grab::StatusCodeAndBody => to_json(&StatusCodeAndBody::from(self), pretty),
            Grab::HeadersAndBody => to_json(&HeadersAndBody::from(self), pretty),
        }
    }
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

fn header_value_to_string(header_value: &HeaderValue) -> String {
    match header_value.to_str() {
        Ok(str_) => str_.into(),
        Err(_) => {
            warn!("Found invalid utf-8 header value. Encoding as base64.");
            general_purpose::STANDARD.encode(header_value.as_bytes())
        }
    }
}

fn to_json<T: Serialize>(value: &T, pretty: bool) -> Result<String, ResponseDataError> {
    let result = match pretty {
        true => serde_json::to_string_pretty(value),
        false => serde_json::to_string(value),
    };
    result.map_err(|e| ResponseDataError::Derserialize(e.to_string()))
}

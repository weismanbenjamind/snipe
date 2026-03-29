use crate::errors::ResponseWrapperError;
use base64::Engine;
use base64::engine::general_purpose;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ResponseWrapper {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl ResponseWrapper {
    pub fn new(status_code: StatusCode, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            headers,
            body,
        }
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

    pub async fn try_from_response(response: Response) -> Result<Self, ResponseWrapperError> {
        Ok(Self {
            status_code: response.status().as_u16(),
            headers: build_response_headers(response.headers()),
            body: response
                .text()
                .await
                .map_err(|e| ResponseWrapperError::Build(e.to_string()))?,
        })
    }

    pub fn to_json(&self) -> Result<String, ResponseWrapperError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ResponseWrapperError::Derserialize(e.to_string()))
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

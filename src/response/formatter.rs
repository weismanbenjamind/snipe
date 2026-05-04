use crate::errors::ResponseDataError;
use crate::response::formats::{HTTPFormat, JsonFormat};
use log::{info, warn};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ResponseFormatter {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: String,
}

impl ResponseFormatter {
    #[allow(dead_code)]
    pub fn new(status_code: StatusCode, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    #[inline]
    #[allow(dead_code)]
    pub fn status_code_u16(&self) -> u16 {
        self.status_code.as_u16()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn status_code_string(&self) -> String {
        self.status_code_u16().to_string()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    #[inline]
    #[allow(dead_code)]
    pub fn body(&self) -> &str {
        &self.body
    }

    pub async fn try_from_response(response: Response) -> Result<Self, ResponseDataError> {
        Ok(Self {
            status_code: response.status(),
            headers: build_response_headers(response.headers())?,
            body: build_body(response).await?,
        })
    }

    pub fn get_http_string(
        &self,
        status_code: bool,
        headers: bool,
        body: bool,
    ) -> Result<String, ResponseDataError> {
        HTTPFormat::new(
            status_code.then_some(self.status_code),
            headers.then_some(&self.headers),
            body.then_some(&self.body),
        )
        .into_http_string()
        .map_err(ResponseDataError::to_string_err_from_err)
    }

    pub fn get_json_string(
        &self,
        status_code: bool,
        headers: bool,
        body: bool,
        pretty: bool,
    ) -> Result<String, ResponseDataError> {
        JsonFormat::new_from_str_body(
            status_code.then_some(self.status_code.as_u16()),
            headers.then_some(&self.headers),
            body.then_some(&self.body),
        )
        .into_json_string(pretty)
        .map_err(ResponseDataError::to_string_err_from_err)
    }
}

fn build_response_headers(
    header_map: &HeaderMap,
) -> Result<HashMap<String, String>, ResponseDataError> {
    info!("Building response headers.");
    let mut as_hash_map: HashMap<String, String> = HashMap::new();

    header_map.iter().try_for_each(|(k, v)| {
        if let Some(header_override) =
            as_hash_map.insert(k.as_str().into(), header_value_to_string(v)?)
        {
            warn!("Overriding header {}", header_override)
        }
        Ok(()) // try_for_each each must return Result<(), Err>
    })?; // Need the ? here to propogate errors out of try_for_each

    info!("Response headers built.");
    Ok(as_hash_map)
}

#[inline]
fn header_value_to_string(header_value: &HeaderValue) -> Result<String, ResponseDataError> {
    match header_value.to_str() {
        Ok(str_) => Ok(str_.to_string()),
        Err(e) => Err(ResponseDataError::new_response_field_to_string(
            "headers", e,
        )),
    }
}

async fn build_body(response: Response) -> Result<String, ResponseDataError> {
    info!("Building response body.");
    let as_bytes = response
        .bytes()
        .await
        .map_err(|e| ResponseDataError::Build(e.to_string()))?
        .to_vec();

    match std::str::from_utf8(&as_bytes) {
        Ok(str_) => {
            info!("Response body built.");
            Ok(str_.to_string())
        }
        Err(e) => {
            info!("Failed to build response body with error: {e}.");
            Err(ResponseDataError::new_response_field_to_string("body", e))
        }
    }
}

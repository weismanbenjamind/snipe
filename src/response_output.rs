// TODO - might need rename here. Might actually want to make a module for this module and what ResponseData gets renamed to

use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Error as SerdeJsonError;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Error as FmtError;
use std::fmt::Write;

#[derive(Clone, Debug, Serialize)]
pub struct JsonResponseOutput<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<&'a HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Body<'a>>,
}

impl<'a> JsonResponseOutput<'a> {
    pub fn new(
        status_code: Option<u16>,
        headers: Option<&'a HashMap<String, String>>,
        body: Option<Body<'a>>,
    ) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    pub fn new_from_str_body(
        status_code: Option<u16>,
        headers: Option<&'a HashMap<String, String>>,
        body: Option<&'a str>,
    ) -> Self {
        match body {
            Some(value) => Self::new(status_code, headers, Some(value.into())),
            None => Self::new(status_code, headers, None),
        }
    }

    pub fn into_json_string(self, pretty: bool) -> Result<String, SerdeJsonError> {
        match pretty {
            true => serde_json::to_string_pretty(&self),
            false => serde_json::to_string(&self),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Body<'a> {
    Json(Value),
    RawStr(&'a str),
}

impl<'a> From<&'a str> for Body<'a> {
    fn from(value: &'a str) -> Self {
        match serde_json::from_str::<Value>(value) {
            Ok(json) => Self::Json(json),
            Err(_) => Self::RawStr(value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HTTPResponseOutput<'a> {
    status_code: Option<StatusCode>,
    headers: Option<&'a HashMap<String, String>>,
    body: Option<&'a str>,
}

impl<'a> HTTPResponseOutput<'a> {
    pub fn new(
        status_code: Option<StatusCode>,
        headers: Option<&'a HashMap<String, String>>,
        body: Option<&'a str>,
    ) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    pub fn into_http_string(self) -> Result<String, FmtError> {
        let mut buf = String::new();

        if let Some(status_code) = self.status_code {
            writeln!(buf, "{status_code}")?
        }
        if let Some(headers) = self.headers {
            headers
                .iter()
                .try_for_each(|(k, v)| writeln!(buf, "{}: {}", k, v))?
        }
        if let Some(body) = self.body {
            match buf.is_empty() {
                true => write!(buf, "{body}")?,
                false => write!(buf, "\n{body}")?,
            }
        }

        Ok(buf.trim().to_string())
    }
}

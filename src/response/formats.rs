use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Error as SerdeJsonError;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Error as FmtError;
use std::fmt::Write;

#[derive(Clone, Debug, Serialize)]
pub(super) struct JsonFormat<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<&'a HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Body<'a>>,
}

impl<'a> JsonFormat<'a> {
    pub(super) fn new(
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

    pub(super) fn new_from_str_body(
        status_code: Option<u16>,
        headers: Option<&'a HashMap<String, String>>,
        body: Option<&'a str>,
    ) -> Self {
        match body {
            Some(value) => Self::new(status_code, headers, Some(value.into())),
            None => Self::new(status_code, headers, None),
        }
    }

    pub(super) fn into_json_string(self, pretty: bool) -> Result<String, SerdeJsonError> {
        let serializer = JsonSerializer::from(self);
        match pretty {
            true => serializer.into_string_pretty(),
            false => serializer.into_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub(super) enum Body<'a> {
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

enum JsonSerializer<'a> {
    StatusCode(u16),
    Headers(&'a HashMap<String, String>),
    Body(Body<'a>),
    Multi(JsonFormat<'a>),
}

impl<'a> JsonSerializer<'a> {
    fn into_string_pretty(self) -> Result<String, SerdeJsonError> {
        match self {
            Self::StatusCode(status_code) => {
                let map = Self::build_status_code_map(status_code);
                serde_json::to_string_pretty(&map)
            }
            Self::Headers(headers) => serde_json::to_string_pretty(headers),
            Self::Body(body) => serde_json::to_string_pretty(&body),
            Self::Multi(json_format) => serde_json::to_string_pretty(&json_format),
        }
    }

    fn into_string(self) -> Result<String, SerdeJsonError> {
        match self {
            Self::StatusCode(status_code) => {
                let map = Self::build_status_code_map(status_code);
                serde_json::to_string(&map)
            }
            Self::Headers(headers) => serde_json::to_string(headers),
            Self::Body(body) => serde_json::to_string(&body),
            Self::Multi(json_format) => serde_json::to_string(&json_format),
        }
    }

    fn build_status_code_map(status_code: u16) -> HashMap<&'static str, u16> {
        let mut map: HashMap<&'static str, u16> = HashMap::with_capacity(1);
        map.insert("status_code", status_code);
        map
    }
}

impl<'a> From<JsonFormat<'a>> for JsonSerializer<'a> {
    fn from(value: JsonFormat<'a>) -> Self {
        let to_match = (value.status_code, value.headers, value.body);
        match to_match {
            (Some(status_code), None, None) => Self::StatusCode(status_code),
            (None, Some(headers), None) => Self::Headers(headers),
            (None, None, Some(body)) => Self::Body(body),
            (Some(_), Some(_), Some(_))
            | (Some(_), Some(_), None)
            | (Some(_), None, Some(_))
            | (None, Some(_), Some(_))
            | (None, None, None) => Self::Multi(JsonFormat {
                status_code: to_match.0,
                headers: to_match.1,
                body: to_match.2,
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct HTTPFormat<'a> {
    status_code: Option<StatusCode>,
    headers: Option<&'a HashMap<String, String>>,
    body: Option<&'a str>,
}

impl<'a> HTTPFormat<'a> {
    pub(super) fn new(
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

    pub(super) fn into_http_string(self) -> Result<String, FmtError> {
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

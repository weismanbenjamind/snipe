use crate::errors::ResponseFormatterError;
use crate::inputs::Grab;
use crate::response_data::ResponseData;
use serde::Serialize;
use serde_json::json;

#[derive(Clone, Debug)]
pub struct ResponseFormatter<'a> {
    response_data: &'a ResponseData,
}

impl<'a> ResponseFormatter<'a> {
    #[allow(dead_code)]
    pub fn new(response_data: &'a ResponseData) -> Self {
        Self { response_data }
    }

    #[allow(dead_code)]
    pub fn response_data(&'a self) -> &'a ResponseData {
        self.response_data
    }

    pub fn get_http_string(&self, grab: Grab) -> String {
        let output = match grab {
            Grab::Status => self.response_data.status_code().to_string(),
            Grab::Headers => self.headers_to_http_string(),
            Grab::Body => self.response_data.body().to_string(),
            Grab::StatusCodeAndHeaders => self.status_code_and_headers_to_http_string(),
            Grab::StatusCodeAndBody => self.status_code_and_body_to_http_string(),
            Grab::HeadersAndBody => self.headers_and_body_to_http_string(),
            Grab::Full => self.to_full_http_string(),
            Grab::StatusCode => self.response_data.status_code_string(),
        };
        output.trim().to_string()
    }

    pub fn get_json_string(
        &self,
        grab: Grab,
        pretty: bool,
    ) -> Result<String, ResponseFormatterError> {
        match grab {
            Grab::Status => self.status_code_to_json(pretty),
            Grab::Headers => self.headers_to_json(pretty),
            Grab::Body => self.body_to_json(pretty),
            Grab::StatusCodeAndHeaders => self.status_code_and_headers_to_json(pretty),
            Grab::StatusCodeAndBody => self.status_code_and_body_to_json(pretty),
            Grab::HeadersAndBody => self.headers_and_body_as_json(pretty),
            Grab::Full => self.to_json(pretty),
            Grab::StatusCode => Ok(self.response_data.status_code_string()),
        }
    }

    #[inline]
    fn status_code_to_json(&self, pretty: bool) -> Result<String, ResponseFormatterError> {
        to_json_string(
            &json!({"status_code": self.response_data.status_code_u16()}),
            pretty,
        )
    }

    fn headers_to_http_string(&self) -> String {
        let mut http_string = String::new();

        self.response_data.headers().iter().for_each(|(k, v)| {
            http_string.push_str(&format!("{}: {}\n", k, v));
        });

        match http_string.strip_suffix("\n") {
            Some(stripped) => stripped.to_string(),
            None => http_string,
        }
    }

    #[inline]
    fn headers_to_json(&self, pretty: bool) -> Result<String, ResponseFormatterError> {
        to_json_string(&json!({"headers": self.response_data.headers()}), pretty)
    }

    fn body_to_map(&'a self) -> BodyToMapReturn<'a> {
        match serde_json::from_str::<serde_json::Value>(self.response_data.body()) {
            Ok(as_json) => BodyToMapReturn::Json(as_json),
            Err(_) => BodyToMapReturn::Str(self.response_data.body()),
        }
    }

    fn body_to_json(&self, pretty: bool) -> Result<String, ResponseFormatterError> {
        let json = match self.body_to_map() {
            BodyToMapReturn::Json(body) => json!({"body": body}),
            BodyToMapReturn::Str(body) => json!({"body": body}),
        };
        to_json_string(&json, pretty)
    }

    #[inline]
    fn status_code_and_headers_to_http_string(&self) -> String {
        format!(
            "{}\n{}",
            self.response_data.status_code(),
            self.headers_to_http_string()
        )
    }

    #[inline]
    fn status_code_and_headers_to_json(
        &self,
        pretty: bool,
    ) -> Result<String, ResponseFormatterError> {
        to_json_string(
            &json!({"status_code": self.response_data.status_code_u16(), "headers": self.response_data.headers()}),
            pretty,
        )
    }

    #[inline]
    fn status_code_and_body_to_http_string(&self) -> String {
        format!(
            "{}\n\n{}",
            self.response_data.status_code(),
            self.response_data.body()
        )
    }

    fn status_code_and_body_to_json(&self, pretty: bool) -> Result<String, ResponseFormatterError> {
        let json = match self.body_to_map() {
            BodyToMapReturn::Json(body) => {
                json!({
                    "status_code": self.response_data.status_code_u16(),
                    "body": body,
                })
            }
            BodyToMapReturn::Str(body) => {
                json!({
                    "status_code": self.response_data.status_code_u16(),
                    "body": body,
                })
            }
        };
        to_json_string(&json, pretty)
    }

    #[inline]
    fn headers_and_body_to_http_string(&self) -> String {
        format!(
            "{}\n\n{}",
            self.headers_to_http_string(),
            self.response_data.body()
        )
    }

    fn headers_and_body_as_json(&self, pretty: bool) -> Result<String, ResponseFormatterError> {
        let json = match self.body_to_map() {
            BodyToMapReturn::Json(body) => {
                json!({
                    "headers": self.response_data.headers(),
                    "body": body
                })
            }
            BodyToMapReturn::Str(body) => {
                json!({
                    "headers": self.response_data.headers(),
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
            self.response_data.status_code(),
            self.headers_to_http_string(),
            self.response_data.body()
        )
    }

    fn to_json(&self, pretty: bool) -> Result<String, ResponseFormatterError> {
        let json = match self.body_to_map() {
            BodyToMapReturn::Json(body) => {
                json!({
                    "status_code": self.response_data.status_code_u16(),
                    "headers": self.response_data.headers(),
                    "body": body
                })
            }
            BodyToMapReturn::Str(body) => {
                json!({
                    "status_code": self.response_data.status_code_u16(),
                    "headers": self.response_data.headers(),
                    "body": body
                })
            }
        };
        to_json_string(&json, pretty)
    }
}

impl<'a> From<&'a ResponseData> for ResponseFormatter<'a> {
    fn from(value: &'a ResponseData) -> Self {
        Self {
            response_data: value,
        }
    }
}

fn to_json_string<T: Serialize>(value: &T, pretty: bool) -> Result<String, ResponseFormatterError> {
    let result = match pretty {
        true => serde_json::to_string_pretty(value),
        false => serde_json::to_string(value),
    };
    result.map_err(|e| ResponseFormatterError::Base(e.to_string()))
}

enum BodyToMapReturn<'a> {
    Json(serde_json::Value),
    Str(&'a str),
}

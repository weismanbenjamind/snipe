use crate::errors::ResponseFormatterError;
use crate::inputs::Grab;
use crate::response_data::ResponseData;
use crate::response_output::{HTTPResponseOutput, JsonResponseOutput};
use reqwest::StatusCode;
use std::collections::HashMap;

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

    pub fn get_http_string(&self, grab: Grab) -> Result<String, ResponseFormatterError> {
        let mut status_code: Option<StatusCode> = None;
        let mut headers: Option<&HashMap<String, String>> = None;
        let mut body: Option<&str> = None;

        // Note - might want to add flags for status_code, headers, body and set proper values to None when building response data
        match grab {
            Grab::Status => {
                status_code = Some(self.response_data.status_code());
            }
            Grab::Headers => {
                headers = Some(self.response_data.headers());
            }
            Grab::Body => {
                body = Some(self.response_data.body());
            }
            Grab::StatusCodeAndHeaders => {
                status_code = Some(self.response_data.status_code());
                headers = Some(self.response_data.headers());
            }
            Grab::StatusCodeAndBody => {
                status_code = Some(self.response_data.status_code());
                body = Some(self.response_data.body())
            }
            Grab::HeadersAndBody => {
                headers = Some(self.response_data.headers());
                body = Some(self.response_data.body())
            }
            Grab::Full => {
                status_code = Some(self.response_data.status_code());
                headers = Some(self.response_data.headers());
                body = Some(self.response_data.body())
            }
            Grab::StatusCode => return Ok(self.response_data.status_code_string()),
        }

        HTTPResponseOutput::new(status_code, headers, body)
            .into_http_string()
            .map_err(|e| {
                ResponseFormatterError::Base(format!(
                    "Failed to serialize reponse output to HTTP string. Error: {e}"
                ))
            })
    }

    pub fn get_json_string(
        &self,
        grab: Grab,
        pretty: bool,
    ) -> Result<String, ResponseFormatterError> {
        let mut status_code: Option<u16> = None;
        let mut headers: Option<&HashMap<String, String>> = None;
        let mut body: Option<&str> = None;

        // Note - might want to add flags for status_code, headers, body and set proper values to None when building response data
        match grab {
            Grab::Status => {
                status_code = Some(self.response_data.status_code_u16());
            }
            Grab::Headers => {
                headers = Some(self.response_data.headers());
            }
            Grab::Body => {
                body = Some(self.response_data.body());
            }
            Grab::StatusCodeAndHeaders => {
                status_code = Some(self.response_data.status_code_u16());
                headers = Some(self.response_data.headers());
            }
            Grab::StatusCodeAndBody => {
                status_code = Some(self.response_data.status_code_u16());
                body = Some(self.response_data.body())
            }
            Grab::HeadersAndBody => {
                headers = Some(self.response_data.headers());
                body = Some(self.response_data.body())
            }
            Grab::Full => {
                status_code = Some(self.response_data.status_code_u16());
                headers = Some(self.response_data.headers());
                body = Some(self.response_data.body())
            }
            Grab::StatusCode => return Ok(self.response_data.status_code_string()),
        };

        JsonResponseOutput::new_from_str_body(status_code, headers, body)
            .into_json_string(pretty)
            .map_err(|e| {
                ResponseFormatterError::Base(format!(
                    "Failed to serialize reponse output to JSON string. Error: {e}"
                ))
            })
    }
}

impl<'a> From<&'a ResponseData> for ResponseFormatter<'a> {
    fn from(value: &'a ResponseData) -> Self {
        Self {
            response_data: value,
        }
    }
}

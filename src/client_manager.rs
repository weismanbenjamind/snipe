use reqwest::{Client, RequestBuilder, Response};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

use crate::targets::{Method, Target, Auth};
use std::collections::HashMap;
use crate::errors::RequestManagerError;
use std::str::FromStr;
use log::warn;

pub struct RequestManager {
    client: Client
}

impl RequestManager {
    pub fn new() -> Self {
        Self {
            client: Client::new()
        }
    }

    pub async fn send_request(&self, target: &Target) -> Result<Response, RequestManagerError> {
        self.build_request(target)?.send().await.map_err(|e| RequestManagerError::SendRequestFailure(e.to_string()))
    }

    fn build_request(&self, target: &Target) -> Result<RequestBuilder, RequestManagerError> {
        let mut request_builder = self.init_request_builder(target);

        if let Some(headers) = target.headers() {
            let headers = build_headers(headers)?;
            request_builder = request_builder.headers(headers);
        }

        if let Some(auth) = target.auth() {
            request_builder = build_auth(request_builder, auth);
        }

        Ok(request_builder)
    }

    fn init_request_builder(&self, target: &Target) -> RequestBuilder {
        match target.method() {
            Method::DELETE => self.client.delete(target.url()),
            Method::GET => self.client.get(target.url()),
            Method::PATCH => self.client.patch(target.url()),
            Method::POST => self.client.post(target.url()),
            Method::PUT => self.client.put(target.url())
        }
}
}

fn build_headers(headers: &HashMap<String, String>) -> Result<HeaderMap, RequestManagerError> {
    let mut header_map = HeaderMap::new();
    let mut header_name: HeaderName;
    let mut header_value: HeaderValue;

    for (name, value) in headers {
        header_name = get_header_name(name)?;
        header_value = get_header_value(value)?;
        if let Some(prev_header_name) = header_map.insert(header_name, header_value) {
            warn!("Overriding header {:?}", prev_header_name);
        }
    }
    Ok(header_map)
}

fn get_header_name(header_name: &str) -> Result<HeaderName, RequestManagerError> {
    Ok(HeaderName::from_str(header_name).map_err(|err| RequestManagerError::RequestBuild(err.to_string()))?)
}

fn get_header_value(header_value: &str) -> Result<HeaderValue, RequestManagerError> {
    Ok(HeaderValue::from_str(header_value).map_err(|err| RequestManagerError::RequestBuild(err.to_string()))?)
}

fn build_auth(request_builder: RequestBuilder, auth: &Auth) -> RequestBuilder {
    match auth {
        Auth::Bearer(bearer_auth) => request_builder.bearer_auth(bearer_auth.token()),
        Auth::Basic(basic_auth) => request_builder.basic_auth(basic_auth.username(), Some(basic_auth.password()))
    }
}

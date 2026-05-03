use reqwest::Client as Client_;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{RequestBuilder, Response};

use crate::errors::ClientError;
use crate::targets::{Auth, Method, Target};
use log::{info, warn};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

pub struct Client {
    _client: Client_,
}

impl Client {
    pub fn new() -> Result<Self, ClientError> {
        info!("Building client");
        let _client = Client_::builder()
            .build()
            .map_err(|e| ClientError::ClientBuild(e.to_string()))?;
        info!("Client built.");
        Ok(Self { _client })
    }

    pub async fn send_request(&self, target: &Target) -> Result<Response, ClientError> {
        self.build_request(target)?
            .send()
            .await
            .map_err(|e| ClientError::SendRequestFailure(e.to_string()))
    }

    fn build_request(&self, target: &Target) -> Result<RequestBuilder, ClientError> {
        info!("Starting request build.");
        let mut request_builder = self.init_request_builder(target);

        if let Some(timeout) = target.timeout_seconds() {
            info!("Adding timeout of {timeout} seconds.");
            request_builder = request_builder.timeout(Duration::from_secs(timeout));
            info!("Timeout added.")
        }

        if let Some(headers) = target.headers() {
            info!("Adding headers");
            request_builder = request_builder.headers(build_headers(headers)?);
            info!("Headers added.")
        }

        if let Some(auth) = target.auth() {
            info!("Adding auth.");
            request_builder = build_auth(request_builder, auth);
            info!("Auth added.")
        }

        if let Some(payload) = target.payload() {
            info!("Adding payload.");
            request_builder = request_builder.json(payload);
            info!("Payload added.");
        }

        info!("Request built");
        Ok(request_builder)
    }

    fn init_request_builder(&self, target: &Target) -> RequestBuilder {
        match target.method() {
            Method::Delete => self._client.delete(target.url()),
            Method::Get => self._client.get(target.url()),
            Method::Patch => self._client.patch(target.url()),
            Method::Post => self._client.post(target.url()),
            Method::Put => self._client.put(target.url()),
        }
    }
}

fn build_headers(headers: &HashMap<String, String>) -> Result<HeaderMap, ClientError> {
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

fn get_header_name(header_name: &str) -> Result<HeaderName, ClientError> {
    HeaderName::from_str(header_name).map_err(|err| ClientError::RequestBuild(err.to_string()))
}

fn get_header_value(header_value: &str) -> Result<HeaderValue, ClientError> {
    HeaderValue::from_str(header_value).map_err(|err| ClientError::RequestBuild(err.to_string()))
}

fn build_auth(request_builder: RequestBuilder, auth: &Auth) -> RequestBuilder {
    match auth {
        Auth::Bearer(bearer_auth) => request_builder.bearer_auth(bearer_auth.token()),
        Auth::Basic(basic_auth) => {
            request_builder.basic_auth(basic_auth.username(), Some(basic_auth.password()))
        }
    }
}

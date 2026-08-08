use reqwest::Client as Client_;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{RequestBuilder, Response};

use crate::containers::{Auth, Headers, Method, Payload, Target};
use crate::errors::ClientError;
use log::{debug, info, warn};
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
            debug!("Adding timeout of {timeout} seconds.");
            request_builder = request_builder.timeout(Duration::from_secs(timeout));
            debug!("Timeout added.")
        }

        if let Some(headers) = target.headers() {
            debug!("Adding headers:\n{headers:#?}");
            request_builder = request_builder.headers(build_headers(headers)?);
            debug!("Headers added.")
        }

        if let Some(auth) = target.auth() {
            debug!("Adding auth:\n{auth:#?}");
            request_builder = build_auth(request_builder, auth);
            debug!("Auth added.")
        }

        if let Some(payload) = target.payload() {
            debug!("Adding payload:\n{payload:#?}");
            request_builder = build_payload(request_builder, payload)?;
            debug!("Payload added.");
        }

        info!("Request built.");
        Ok(request_builder)
    }

    fn init_request_builder(&self, target: &Target) -> RequestBuilder {
        let method = target.method();
        let url = target.url();
        debug!("Attempting to build '{:?}' request for url {url}", method);
        match method {
            Method::Delete => self._client.delete(url),
            Method::Get => self._client.get(url),
            Method::Patch => self._client.patch(url),
            Method::Post => self._client.post(url),
            Method::Put => self._client.put(url),
        }
    }
}

fn build_headers(headers: &Headers) -> Result<HeaderMap, ClientError> {
    let mut header_map = HeaderMap::new();
    let mut header_name: HeaderName;
    let mut header_value: HeaderValue;

    for (name, secret_string) in headers {
        header_name = get_header_name(name)?;
        header_value = get_header_value(secret_string.value())?;
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
        Auth::Bearer(bearer_auth) => request_builder.bearer_auth(bearer_auth.token().value()),
        Auth::Basic(basic_auth) => {
            request_builder.basic_auth(basic_auth.username(), Some(basic_auth.password().value()))
        }
    }
}

fn build_payload(
    request_builder: RequestBuilder,
    payload: &Payload,
) -> Result<RequestBuilder, ClientError> {
    match payload {
        Payload::Params(json) => Ok(request_builder.json(json)),
        Payload::File(path) => {
            debug!("Reading payload at path {} into bytes", path.display());
            let bytes = std::fs::read(path).map_err(|e| ClientError::BodyToBytes {
                path: path.into(),
                source: e,
            })?;
            Ok(request_builder.body(bytes))
        }
    }
}

use reqwest::{Client, RequestBuilder, header};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

use crate::targets::{Method, Target, Auth, AuthScheme};
use std::collections::HashMap;
use crate::errors::ClientManagerError;
use std::str::FromStr;
use log::warn;

pub struct ClientManager {
    client: Client
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            client: Client::new()
        }
    }

    pub fn send_request(&self, target: &Target) -> Result<(), ClientManagerError> {
        let request = init_request(target, &self.client);

        if let Some(headers) = target.headers() {
            let headers = build_headers(headers)?;
            request.headers(headers);
        }

        // TODO - Start here - match on the enum to do auth
        if let Some(auth) = target.auth() {
            let auth = build_auth(auth)?;
            
        }

        Ok(())
    }
}

fn init_request(target: &Target, client: &Client) -> RequestBuilder {
    match target.method() {
        Method::DELETE => client.delete(target.url()),
        Method::GET => client.get(target.url()),
        Method::PATCH => client.patch(target.url()),
        Method::POST => client.post(target.url()),
        Method::PUT => client.put(target.url())
    }
}

fn build_headers(headers: &HashMap<String, String>) -> Result<HeaderMap, ClientManagerError> {
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

fn get_header_name(header_name: &str) -> Result<HeaderName, ClientManagerError> {
    Ok(HeaderName::from_str(header_name).map_err(|err| ClientManagerError::RequestBuild(err.to_string()))?)
}

fn build_auth(auth: &Auth) -> Result<HeaderMap, ClientManagerError> {
    let auth_value = match auth.scheme() {
        AuthScheme::Basic => format!("Basic {}", auth.credentials()),
        AuthScheme::Bearer => format!("Bearer {}", auth.credentials())
    };

    let auth_value = get_header_value(&auth_value)?;
    let mut header_map = HeaderMap::new();
    header_map.insert(header::AUTHORIZATION, auth_value).unwrap();

    Ok(header_map)

}

fn get_auth_header_value(auth: &HashMap<String, String>) -> Result<HeaderValue, ClientManagerError> {
    let auth_name = get_only_val(auth.keys())?;
    let auth_value = get_only_val(auth.values())?;
    Ok(get_header_value(&format!("{auth_name}: {auth_value}")))?
}

fn get_header_value(header_value: &str) -> Result<HeaderValue, ClientManagerError> {
    Ok(HeaderValue::from_str(header_value).map_err(|err| ClientManagerError::RequestBuild(err.to_string()))?)
}

fn get_only_val<'a, I: Iterator<Item = &'a String>>(mut iterator: I) -> Result<&'a str, ClientManagerError> {
    match (iterator.next(), iterator.next()) {
        (Some(item), None) => Ok(item),
        (Some(_), Some(_)) => Err(ClientManagerError::RequestBuild(String::from("Found multiple items in iterator."))),
        (None, None) => Err(ClientManagerError::RequestBuild(String::from("Found no items in iterator."))),
        (None, Some(_)) => Err(ClientManagerError::RequestBuild(String::from("Iterator in invalid state."))),
    }
}
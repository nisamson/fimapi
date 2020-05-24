//! This module contains an implementation of an HTTP client for communicating with the FimFic servers

use crate::response::{Error, extract_api_response};

macro_rules! endpoint {
    () => {"https://www.fimfiction.net/api/v2"};
    ($name:literal) => {concat!(endpoint!(), $name)};
}

/// The URL for the fimfiction API
pub const BASE_URL: &str = endpoint!();


/// Client for making requests through FimFic API. This type will only support simple client credentials.
#[derive(Clone, Debug)]
pub struct Client {
    bearer_token: String,
    client: reqwest::Client,
}

impl Client {
    /// Creates a Client with default configuration.
    pub async fn new(client_id: impl AsRef<str>, client_secret: impl AsRef<str>) -> Result<Self, Error> {
        Self::with_client(client_id, client_secret, reqwest::Client::default()).await
    }

    /// Creates a client with the given [HTTP Client][reqwest::Client].
    pub async fn with_client(client_id: impl AsRef<str>, client_secret: impl AsRef<str>, http: reqwest::Client) -> Result<Self, Error> {
        let form = [
            ("client_id", client_id.as_ref()),
            ("client_secret", client_secret.as_ref()),
            ("grant_type", "client_credentials")
        ];

        let res = http.post(endpoint!("/token"))
            .form(&form)
            .send()
            .await?;

        let value: serde_json::Value = extract_api_response(res).await?;
        Ok(Client {
            bearer_token: format!("Bearer {}", value.get("access_token").unwrap().as_str().unwrap()),
            client: http,
        })
    }

    /// Creates a client from the given bearer token. This does not verify that this is a valid token,
    /// so if it's not valid, you will be receiving a lot of [APIErrors][crate::response::error::APIError]
    pub fn from_token(tok: impl Into<String>) -> Self {
        Client {
            bearer_token: tok.into(),
            client: reqwest::Client::default(),
        }
    }

    /// Accessor for the bearer token. You can save one that is generated and reuse it in the future.
    pub fn bearer_token(&self) -> &str {
        &self.bearer_token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::init_env;

    #[tokio::test]
    pub async fn grab_token() {
        init_env();
        let run_new_token = std::env::var("GET_NEW_TOKEN").is_ok();
        if !run_new_token {
            println!("Did not run test because GET_NEW_TOKEN did not exist.");
            return;
        }

        let client_id = std::env::var("FF_CLIENT_ID").unwrap();
        let client_secret = std::env::var("FF_CLIENT_SECRET").unwrap();

        let _ = Client::new(client_id, client_secret).await.unwrap();
    }
}
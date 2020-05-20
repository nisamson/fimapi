//! This module contains an implementation of an HTTP client for communicating with the FimFic servers

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client
}

impl From<reqwest::Client> for Client {
    fn from(c: reqwest::Client) -> Self {
        Client { client: c }
    }
}

impl Client {
    // pub fn new()
}
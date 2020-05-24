// Copyright 2020 Nick Samson -- See LICENSE for copyright info.

//! Contains types and functions related to responses we expect from the FimFic API.


pub mod error;

use crate::response::error::{InvalidErrorCode};
use std::borrow::Cow;

pub use error::APIError;
pub use error::Error;
use serde_json::Value;
use std::convert::TryFrom;

pub(crate) trait ExtractErrExt {
    fn extract_error(&self) -> Result<APIError, InvalidErrorCode>;
}

impl ExtractErrExt for serde_json::Value {
    fn extract_error(&self) -> Result<APIError, InvalidErrorCode> {
        self.get("errors")
            .and_then(|v| v.get(0))
            .ok_or_else(|| InvalidErrorCode::Invalid(Cow::Borrowed(self)))
            .and_then(|v| APIError::try_from(v.clone()))
    }
}

pub(crate) async fn extract_api_response<T: serde::de::DeserializeOwned>(s: reqwest::Response) -> Result<T, Error> {
    if s.status().is_client_error() {
        let v = s.json::<Value>().await?;
        Err(v.extract_error().unwrap())?
    } else if s.status().is_server_error() {
        Err(s.error_for_status().unwrap_err())?
    } else {
        let o = s.json::<T>().await?;
        Ok(o)
    }
}

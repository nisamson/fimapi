// Copyright 2020 Nick Samson -- See LICENSE for copyright info.

//! Contains types and functions related to errors received from the FimFic API.

use std::convert::TryFrom;
use std::ops::Rem;
use std::borrow::Cow;
use serde_json::Value;

/// Ideally, you should never see one of these. These happen when an error code is unrecognized or
/// malformed.
#[derive(Debug, thiserror::Error)]
pub enum InvalidErrorCode<'value> {
    /// Unrecognized code
    #[error("Invalid error code: {0}")]
    BadCode(u64),
    /// The error has changed so much we couldn't parse anything from it.
    #[error("Could not parse received value: {0}")]
    Invalid(Cow<'value, serde_json::Value>),
}

/// 400 errors
#[derive(thiserror::Error, Debug, Copy, Clone)]
pub enum Malformed {
    /// The body of the request was not valid. It should be valid JSON.
    #[error("The body of the request was not valid")]
    Body,
    /// The requested included resource was not valid.
    #[error("The requested included resource was not valid.")]
    Include,
    #[error("You should never see this.")]
    #[doc(hidden)]
    __Nonexhaustive,
}

impl TryFrom<u64> for Malformed {
    type Error = InvalidErrorCode<'static>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value / 10 != 400 {
            Err(InvalidErrorCode::BadCode(value))
        } else {
            let idx = value.rem(10);
            match idx {
                1 => Ok(Malformed::Body),
                2 => Ok(Malformed::Include),
                _ => Err(InvalidErrorCode::BadCode(value))
            }
        }
    }
}

/// 403 errors.
#[derive(thiserror::Error, Debug, Copy, Clone)]
pub enum Forbidden {
    /// Returned whenever you try to do something the authenticated user is not allowed to do.
    /// For example, trying to edit a story the user does not own will return this error.
    #[error("The authenticated user is not allowed to perform that action.")]
    InvalidPermission,
    /// The scope for this request was not set for the token being used.
    /// Returned, for example, if you tried to edit a story without `write_stories`.
    #[error("The client is missing the required scope to perform that action.")]
    MissingScope,
    /// The token used for the request was not valid.
    /// Either check you have the data correct or request a new token via the auth flow.
    #[error("The token used to the request was not valid.")]
    InvalidToken,
    #[error("You should never see this.")]
    #[doc(hidden)]
    __Nonexhaustive,
}

impl TryFrom<u64> for Forbidden {
    type Error = InvalidErrorCode<'static>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value / 10 != 403 {
            Err(InvalidErrorCode::BadCode(value))
        } else {
            let idx = value.rem(10);
            match idx {
                0 => Ok(Forbidden::InvalidPermission),
                1 => Ok(Forbidden::MissingScope),
                2 => Ok(Forbidden::InvalidToken),
                _ => Err(InvalidErrorCode::BadCode(value))
            }
        }
    }
}

/// 404 errors.
#[derive(thiserror::Error, Debug, Copy, Clone)]
pub enum NotFound {
    /// The requested resource was not found.
    /// Will return if the resource you're querying for a collection of does not exist either.
    #[error("The requested resource was not found.")]
    ResourceNotFound,
    /// The requested application does not exist.
    /// Returned whenever `client_id` is submitted and a corresponding application does not exist.
    #[error("The requested application does not exist.")]
    InvalidApplication,
    /// The requested endpoint does not exist.
    /// Check you have the correct HTTP method as this is frequently the cause.
    /// Also check you are not trying to using string values for variables that expect numeric inputs.
    #[error("The requested endpoint does not exist.")]
    EndpointMissing,
    #[error("You should never see this.")]
    #[doc(hidden)]
    __Nonexhaustive,
}

impl TryFrom<u64> for NotFound {
    type Error = InvalidErrorCode<'static>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value / 10 != 404 {
            Err(InvalidErrorCode::BadCode(value))
        } else {
            let idx = value.rem(10);
            match idx {
                0 => Ok(NotFound::ResourceNotFound),
                1 => Ok(NotFound::InvalidApplication),
                2 => Ok(NotFound::EndpointMissing),
                _ => Err(InvalidErrorCode::BadCode(value))
            }
        }
    }
}

/// 422 errors.
#[derive(thiserror::Error, Debug, Copy, Clone)]
pub enum Unprocessable {
    /// A parameter required for the request was not present.
    #[error("A parameter required for the request was not present.")]
    MissingParameter,
    /// Argument was invalid.
    #[error("An argument was invalid.")]
    InvalidArgument,
    /// The secret submitted as part of a token exchange was incorrect.
    /// Check you have the correct secret for the client ID you are using.
    #[error("The secret submitted for a token exchange was incorrect.")]
    IncorrectSecret,
    /// The grant type provided as part of a token exchange was not permitted.
    /// The error contains the grant types that are supported.
    #[error("The grant type provided as part of the token exchange was not permitted.")]
    InvalidGrantType,
    /// The authorization header was missing.
    /// Submit a `Authorization` header with your request in order to be authenticated.
    #[error("The authorization header was missing.")]
    MissingAuthHeader,
    /// Some or all of the submitted attributes as part of a PATCH/POST request were not valid.
    #[error("Some or all of the submitted attributes as part of a PATCH/POST request were not valid.")]
    InvalidAttributes,
    /// One of the attributes submitted is not supported.
    /// Check the resource documentation to find out what attributes are valid for each type.
    #[error("One of the attributes submitted is not supported.")]
    UnsupportedAttribute,
    /// The provided filter is not supported.
    /// Check the documentation for the endpoint to see what filters are supported, if any.
    #[error("The provided filter is not supported.")]
    InvalidFilter,
    /// One or more of the pagination properties provided was not valid.
    /// Check the documentation for the endpoint to see what pagination strategy is employed.
    #[error("One or more of the pagination properties provided was not valid.")]
    InvalidPagination,
    /// The HTTP Authorization header was malformed.
    /// It should be in the format of `Bearer access_token`.
    /// Check the OAuth login flow documentation for more help.
    #[error("The HTTP authorization header was malformed.")]
    MalformedAuthHeader,
    /// One or more of the attributes provided as part of a PATCH/POST request was not valid.
    #[error("One or more of the attributes of a PATCH/POST request were not valid.")]
    InvalidAttribute,
    /// The provided sort field is not valid.
    /// Check the endpoint documentation to find out what sort fields are available, if any.
    #[error("The provided sort field is not valid.")]
    InvalidSortField,
    /// The provided sort field was malformed.
    /// Check the {json:api} documentation to see what format sorts should be provided in.
    #[error("The provided sort field was malformed.")]
    MalformedSortField,
    #[error("You should never see this.")]
    #[doc(hidden)]
    __Nonexhaustive,
}

impl TryFrom<u64> for Unprocessable {
    type Error = InvalidErrorCode<'static>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        use Unprocessable::*;
        let idx = if value > 10000 {
            if value / 100 != 422 { return Err(InvalidErrorCode::BadCode(value)); }
            value % 100 // Knighty WHY
        } else {
            if value / 10 != 422 { return Err(InvalidErrorCode::BadCode(value)); }
            value % 10
        };

        let o = match idx {
            0 => MissingParameter,
            1 => InvalidArgument,
            2 => IncorrectSecret,
            3 => InvalidGrantType,
            4 => MissingAuthHeader,
            5 => InvalidAttributes,
            6 => UnsupportedAttribute,
            7 => InvalidFilter,
            8 => InvalidPagination,
            9 => MalformedAuthHeader,
            10 => InvalidAttribute,
            11 => InvalidSortField,
            12 => MalformedSortField,
            _ => return Err(InvalidErrorCode::BadCode(value))
        };

        Ok(o)
    }
}

/// The type of error received from FimFic.
#[derive(thiserror::Error, Debug, Copy, Clone)]
pub enum ErrorKind {
    /// 400 errors.
    #[error("{0}")]
    Malformed(#[from] Malformed),
    /// 403 errors.
    #[error("{0}")]
    Forbidden(#[from] Forbidden),
    /// 404 errors.
    #[error("{0}")]
    NotFound(#[from] NotFound),
    /// 422 errors.
    #[error("{0}")]
    Unprocessable(#[from] Unprocessable),
    /// 429 errors.
    #[error("You are being rate limited.")]
    RateLimited,
    #[error("You should never see this.")]
    #[doc(hidden)]
    __Nonexhaustive,
}

impl TryFrom<u64> for ErrorKind {
    type Error = InvalidErrorCode<'static>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let v = value;
        let o = match v / 10 {
            400 => ErrorKind::Malformed(Malformed::try_from(v)?),
            403 => ErrorKind::Forbidden(Forbidden::try_from(v)?),
            404 => ErrorKind::NotFound(NotFound::try_from(v)?),
            429 => ErrorKind::RateLimited,
            v if v == 422 || v / 10 == 422 => ErrorKind::Unprocessable(Unprocessable::try_from(value)?),
            v => return Err(InvalidErrorCode::BadCode(v))
        };
        Ok(o)
    }
}

/// Represents an error received from FimFic.
/// Contains the meta data necessary to understand what when wrong.
#[derive(Debug, thiserror::Error, Clone)]
#[error("Error from API: {kind}: {meta}")]
pub struct APIError {
    kind: ErrorKind,
    meta: serde_json::Value,
}

impl APIError {
    /// Retrieves the [ErrorKind] describing how the request failed.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Retrieves the metadata [Value][serde_json::Value] associated with the failure.
    pub fn meta(&self) -> &serde_json::Value {
        &self.meta
    }
}

impl TryFrom<serde_json::Value> for APIError {
    type Error = InvalidErrorCode<'static>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let code = value.get("code")
            .ok_or_else(|| InvalidErrorCode::Invalid(Cow::Owned(value.clone())))?
            .as_u64()
            .ok_or_else(|| InvalidErrorCode::Invalid(Cow::Owned(value.clone())))?;
        let kind = ErrorKind::try_from(code)?;
        let meta = value.get("meta").map(|x| x.clone()).unwrap_or_else(|| serde_json::Value::Null);
        Ok(APIError { kind, meta })
    }
}

/// Wrapper around the errors you may see while using this crate.
/// This will typically be either HTTP errors or FimFic API errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Wrapper around [reqwest] errors.
    #[error("Error occurred while processing request: {0}")]
    Request(#[from] reqwest::Error),
    /// Wrapper around [APIError]
    #[error("")]
    API(#[from] APIError),
}


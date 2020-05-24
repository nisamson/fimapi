#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![deny(unused_imports, missing_docs)]

//! The `fimapi` crate is a Rust wrapper around the [FimFiction](https://fimfiction.net) web API

use std::str::FromStr;

pub mod client;
pub mod response;
pub mod auth;
pub(crate) mod util;
#[cfg(test)]
pub(crate) mod test;

/// Returns a string representation of the fimapi library version
pub fn version_str() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns a [Version][semver::Version] representation of the `fimapi` library version.
pub fn version() -> semver::Version {
    semver::Version::from_str(version_str()).expect("fimapi was compiled with a bad CARGO_PKG_VERSION.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parse() {
        assert_eq!(version().to_string(), version_str())
    }
}
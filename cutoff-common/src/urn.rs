use std::fmt::{Display, Formatter};
use std::str::FromStr;

use derive_builder::Builder;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;
use url::Url;

static URN_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Za-z0-9\-._]+):([A-Za-z0-9.\-_:]+)(?:/([A-Za-z0-9/\-]*))?$")
        .expect("Cannot compile the URN regular expression")
});

#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
pub struct Urn {
    #[builder(setter(into))]
    nid: String,
    #[builder(setter(into))]
    nss: String,
    #[builder(setter(into, strip_option), default)]
    path: Option<String>,
    #[builder(setter(into, strip_option), default)]
    query: Option<String>,
    #[builder(setter(into, strip_option), default)]
    fragment: Option<String>,
}

impl FromStr for Urn {
    type Err = UrnFormatError;

    fn from_str(urn_string: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(urn_string)
            .map_err(|_| UrnFormatError::InvalidUrn)?;
        
        if url.scheme() == "urn" {
            if let Some(captures) = URN_PATTERN.captures(url.path()) {
                let urn = Urn {
                    nid: captures.get(1).map_or("", |m| m.as_str()).to_string(),
                    nss: captures.get(2).map_or("", |m| m.as_str()).to_string(),
                    path: captures.get(3).map(|m| m.as_str().to_string()),
                    query: url.query().map(|s| s.to_string()),
                    fragment: url.fragment().map(|s| s.to_string()),
                };
                Ok(urn)
            } else {
                Err(UrnFormatError::InvalidUrn)
            }
        } else {
            Err(UrnFormatError::UrnSchemeExpected)
        }
    }
}

impl Urn {
    pub fn builder() -> UrnBuilder {
        UrnBuilder::default()
    }

    /// The Namespace ID
    pub fn nid(&self) -> &str { &self.nid }
    /// The Namespace-Specific String
    pub fn nss(&self) -> &str { &self.nss }
    pub fn path(&self) -> Option<&str> { self.path.as_deref() }
    pub fn query(&self) -> Option<&str> { self.query.as_deref() }
    pub fn fragment(&self) -> Option<&str> { self.fragment.as_deref() }
}

impl Display for Urn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "urn:{}:{}", self.nid, self.nss)?;
        if let Some(path) = &self.path {
            write!(f, "/{}", path)?;
        }
        if let Some(query) = &self.query {
            write!(f, "?{}", query)?;
        }
        if let Some(fragment) = &self.fragment {
            write!(f, "#{}", fragment)?;
        }
        Ok(())
    }
}

impl Serialize for Urn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Urn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Urn::from_str(&s)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Error, Debug)]
pub enum UrnFormatError {
    #[error("Invalid URN: URN scheme expected, but not found")]
    UrnSchemeExpected,

    #[error("Invalid URN: unrecognizable URN format")]
    InvalidUrn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let urn = Urn::from_str("urn:some_nid:foo.bar/hello?foo=bar&flip=flop#world").unwrap();
        assert_eq!(urn.nid, "some_nid");
        assert_eq!(urn.nss, "foo.bar");
        assert_eq!(urn.path, Some("hello".to_string()));
        assert_eq!(urn.query, Some("foo=bar&flip=flop".to_string()));
        assert_eq!(urn.fragment, Some("world".to_string()));
    }

    #[test]
    fn test_no_scheme() {
        match Urn::from_str("some_nid:foo.bar/hello?foo=bar&flip=flop#world") {
            Err(UrnFormatError::InvalidUrn) => {}
            _ => panic!("Expected UriFormatError::InvalidUrnPattern"),
        }
    }

    #[test]
    fn test_display() {
        let urn = Urn::builder()
            .nid("some_nid")
            .nss("foo.bar")
            .path("hello")
            .query("foo=bar&flip=flop")
            .fragment("world")
            .build().unwrap();

        assert_eq!(
            "urn:some_nid:foo.bar/hello?foo=bar&flip=flop#world",
            urn.to_string()
        );
    }
}
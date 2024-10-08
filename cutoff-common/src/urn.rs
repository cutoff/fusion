use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::LazyLock;

use derive_builder::Builder;
use regex::Regex;
use thiserror::Error;
use url::Url;

/// A regular expression pattern for parsing URNs.
/// The pattern matches URNs in the format: urn:<nid>:<nss>[/<path>][?<query>][#<fragment>]
static URN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([A-Za-z0-9\-._]+):([A-Za-z0-9.\-_:]+)(?:/([A-Za-z0-9/\-]*))?$")
        .expect("Cannot compile the URN regular expression")
});

/// Represents a Uniform Resource Name (URN).
///
/// A URN is a URI that uses the "urn" scheme. It's designed to be globally unique and persistent
/// even when the resource it identifies no longer exists or becomes unavailable.
///
/// # Fields
///
/// * `nid`: Namespace Identifier
/// * `nss`: Namespace Specific String
/// * `path`: Optional path component
/// * `query`: Optional query component
/// * `fragment`: Optional fragment component
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

impl Urn {
    pub fn builder() -> UrnBuilder {
        UrnBuilder::default()
    }

    /// Returns the Namespace Identifier (NID) of the URN.
    pub fn nid(&self) -> &str { &self.nid }

    /// Returns the Namespace Specific String (NSS) of the URN.
    pub fn nss(&self) -> &str { &self.nss }

    /// Returns the optional path component of the URN, if present.
    pub fn path(&self) -> Option<&str> { self.path.as_deref() }

    /// Returns the optional query component of the URN, if present.
    pub fn query(&self) -> Option<&str> { self.query.as_deref() }

    /// Returns the optional fragment component of the URN, if present.
    pub fn fragment(&self) -> Option<&str> { self.fragment.as_deref() }

    /// Checks if the URN is valid according to RFC 8141.
    pub fn is_valid(&self) -> bool {
        // This is a simplified check. A full implementation would need to consider
        // all rules specified in RFC 8141.
        !self.nid.is_empty() && !self.nss.is_empty()
    }

    /// Converts the URN to a URL, if possible.
    pub fn to_url(&self) -> Option<Url> {
        Url::parse(&self.to_string()).ok()
    }

    /// Compares two URNs for equality, ignoring case sensitivity in the scheme and namespace identifier.
    pub fn equals(&self, other: &Self) -> bool {
        self.nid.to_lowercase() == other.nid.to_lowercase() &&
            self.nss == other.nss &&
            self.path == other.path &&
            self.query == other.query &&
            self.fragment == other.fragment
    }

    /// Normalizes the URN by converting the scheme and namespace identifier to lowercase.
    pub fn normalize(&self) -> Self {
        Urn {
            nid: self.nid.to_lowercase(),
            nss: self.nss.clone(),
            path: self.path.clone(),
            query: self.query.clone(),
            fragment: self.fragment.clone(),
        }
    }

    /// Creates a new URN with the given query string.
    pub fn with_query(&self, query: Option<&str>) -> Self {
        Urn {
            nid: self.nid.clone(),
            nss: self.nss.clone(),
            path: self.path.clone(),
            query: query.map(String::from),
            fragment: self.fragment.clone(),
        }
    }

    /// Creates a new URN with the given fragment.
    pub fn with_fragment(&self, fragment: Option<&str>) -> Self {
        Urn {
            nid: self.nid.clone(),
            nss: self.nss.clone(),
            path: self.path.clone(),
            query: self.query.clone(),
            fragment: fragment.map(String::from),
        }
    }

    /// Creates a new URN without the query component.
    pub fn without_query(&self) -> Self {
        self.with_query(None)
    }


    /// Creates a new URN without the fragment component.
    pub fn without_fragment(&self) -> Self {
        self.with_fragment(None)
    }

    /// Parses the query string into a key-value map.
    pub fn parse_query(&self) -> Option<std::collections::HashMap<String, String>> {
        self.query.as_ref().map(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .into_owned()
                .collect()
        })
    }

    /// Checks if two URNs are lexically equivalent according to RFC 8141.
    pub fn is_lexically_equivalent(&self, other: &Self) -> bool {
        let norm_self = self.normalize();
        let norm_other = other.normalize();

        norm_self.nid == norm_other.nid &&
            norm_self.nss == norm_other.nss &&
            norm_self.path == norm_other.path
        // Note: query and fragment are not considered for lexical equivalence
    }
}

impl FromStr for Urn {
    type Err = UrnFormatError;

    fn from_str(urn_string: &str) -> Result<Self, Self::Err> {
        // Check if the string starts with "urn:"
        if !urn_string.starts_with("urn:") {
            return Err(UrnFormatError::UrnSchemeExpected);
        }

        let url = Url::parse(urn_string)
            .map_err(|_| UrnFormatError::InvalidUrn)?;

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
    }
}

impl Display for Urn {
    /// Formats the URN as a string.
    ///
    /// # Returns
    ///
    /// A string representation of the URN in the format:
    /// urn:<nid>:<nss>[/<path>][?<query>][#<fragment>]
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

#[derive(Error, Debug)]
pub enum UrnFormatError {
    /// Returned when the input string doesn't start with the "urn:" scheme.
    #[error("Invalid URN: URN scheme expected, but not found")]
    UrnSchemeExpected,

    /// Returned when the input string doesn't match the expected URN format.
    #[error("Invalid URN: unrecognizable URN format")]
    InvalidUrn,
}

#[cfg(feature = "serde")]
mod serde {
    use crate::urn::Urn;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;

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
            Err(UrnFormatError::UrnSchemeExpected) => {}
            _ => panic!("Expected UrnFormatError::UrnSchemeExpected"),
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

    #[test]
    fn test_minimal_urn() {
        let urn = Urn::from_str("urn:example:simple").unwrap();
        assert_eq!(urn.nid(), "example");
        assert_eq!(urn.nss(), "simple");
        assert_eq!(urn.path(), None);
        assert_eq!(urn.query(), None);
        assert_eq!(urn.fragment(), None);
    }

    #[test]
    fn test_urn_with_path() {
        let urn = Urn::from_str("urn:example:complex/path/to/resource").unwrap();
        assert_eq!(urn.nid(), "example");
        assert_eq!(urn.nss(), "complex");
        assert_eq!(urn.path(), Some("path/to/resource"));
        assert_eq!(urn.query(), None);
        assert_eq!(urn.fragment(), None);
    }

    #[test]
    fn test_urn_with_query() {
        let urn = Urn::from_str("urn:example:resource?key1=value1&key2=value2").unwrap();
        assert_eq!(urn.nid(), "example");
        assert_eq!(urn.nss(), "resource");
        assert_eq!(urn.path(), None);
        assert_eq!(urn.query(), Some("key1=value1&key2=value2"));
        assert_eq!(urn.fragment(), None);
    }

    #[test]
    fn test_urn_with_fragment() {
        let urn = Urn::from_str("urn:example:resource#section1").unwrap();
        assert_eq!(urn.nid(), "example");
        assert_eq!(urn.nss(), "resource");
        assert_eq!(urn.path(), None);
        assert_eq!(urn.query(), None);
        assert_eq!(urn.fragment(), Some("section1"));
    }

    #[test]
    fn test_invalid_urn_scheme() {
        assert!(matches!(
            Urn::from_str("http:example:resource"),
            Err(UrnFormatError::UrnSchemeExpected)
        ));
    }

    // Add a new test for invalid URN format
    #[test]
    fn test_invalid_urn_format() {
        assert!(matches!(
            Urn::from_str("urn:invalid"),
            Err(UrnFormatError::InvalidUrn)
        ));
    }

    #[test]
    fn test_builder() {
        let urn = Urn::builder()
            .nid("example")
            .nss("resource")
            .path("path")
            .query("key=value")
            .fragment("section")
            .build()
            .unwrap();

        assert_eq!(urn.to_string(), "urn:example:resource/path?key=value#section");
    }

    #[test]
    fn test_is_valid() {
        let valid_urn = Urn::from_str("urn:example:valid").unwrap();
        assert!(valid_urn.is_valid());

        let invalid_urn = Urn::builder().nid("").nss("invalid").build().unwrap();
        assert!(!invalid_urn.is_valid());
    }

    #[test]
    fn test_to_url() {
        let urn = Urn::from_str("urn:example:resource?query=value#fragment").unwrap();
        let url = urn.to_url().unwrap();
        assert_eq!(url.as_str(), "urn:example:resource?query=value#fragment");
    }

    #[test]
    fn test_equals() {
        let urn1 = Urn::from_str("urn:EXAMPLE:resource").unwrap();
        let urn2 = Urn::from_str("urn:example:resource").unwrap();
        let urn3 = Urn::from_str("urn:example:different").unwrap();

        assert!(urn1.equals(&urn2));
        assert!(!urn1.equals(&urn3));
    }

    #[test]
    fn test_normalize() {
        let urn = Urn::from_str("urn:EXAMPLE:resource").unwrap();
        let normalized = urn.normalize();
        assert_eq!(normalized.nid(), "example");
    }

    #[test]
    fn test_with_query() {
        let urn = Urn::from_str("urn:example:resource").unwrap();
        let with_query = urn.with_query(Some("key=value"));
        assert_eq!(with_query.query(), Some("key=value"));
    }

    #[test]
    fn test_with_fragment() {
        let urn = Urn::from_str("urn:example:resource").unwrap();
        let with_fragment = urn.with_fragment(Some("section1"));
        assert_eq!(with_fragment.fragment(), Some("section1"));
    }

    #[test]
    fn test_without_query() {
        let urn = Urn::from_str("urn:example:resource?key=value").unwrap();
        let without_query = urn.without_query();
        assert_eq!(without_query.query(), None);
    }

    #[test]
    fn test_without_fragment() {
        let urn = Urn::from_str("urn:example:resource#section1").unwrap();
        let without_fragment = urn.without_fragment();
        assert_eq!(without_fragment.fragment(), None);
    }

    #[test]
    fn test_parse_query() {
        let urn = Urn::from_str("urn:example:resource?key1=value1&key2=value2").unwrap();
        let query_map = urn.parse_query().unwrap();
        assert_eq!(query_map.get("key1"), Some(&"value1".to_string()));
        assert_eq!(query_map.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_is_lexically_equivalent() {
        let urn1 = Urn::from_str("urn:EXAMPLE:resource").unwrap();
        let urn2 = Urn::from_str("urn:example:resource").unwrap();
        let urn3 = Urn::from_str("urn:example:resource?query=value#fragment").unwrap();
        let urn4 = Urn::from_str("urn:example:different").unwrap();

        assert!(urn1.is_lexically_equivalent(&urn2));
        assert!(urn1.is_lexically_equivalent(&urn3)); // query and fragment don't affect lexical equivalence
        assert!(!urn1.is_lexically_equivalent(&urn4));
    }
}
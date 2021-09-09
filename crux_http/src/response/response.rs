use super::{decode::decode_body, new_headers};
use crate::http::{
    self,
    headers::{self, HeaderName, HeaderValues, ToHeaderValues},
    Mime, StatusCode, Version,
};

use http::{headers::CONTENT_TYPE, Headers};
use serde::de::DeserializeOwned;

use std::fmt;
use std::ops::Index;

/// An HTTP Response that will be passed to in a message to an apps update function
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Response<Body> {
    version: Option<http::Version>,
    status: http::StatusCode,
    #[serde(with = "header_serde")]
    headers: Headers,
    body: Option<Body>,
}

impl<Body> Response<Body> {
    /// Create a new instance.
    pub(crate) async fn new(mut res: super::ResponseAsync) -> crate::Result<Response<Vec<u8>>> {
        let body = res.body_bytes().await?;

        let headers: &Headers = res.as_ref();
        let headers = headers.clone();

        Ok(Response {
            status: res.status(),
            headers,
            version: res.version(),
            body: Some(body),
        })
    }

    /// Get the HTTP status code.
    ///
    /// # Examples
    ///
    /// ```
    /// # let res = crux_http::testing::ResponseBuilder::ok().build();
    /// assert_eq!(res.status(), 200);
    /// ```
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the HTTP protocol version.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let res = crux_http::testing::ResponseBuilder::ok().build();
    /// use crux_http::http::Version;
    /// assert_eq!(res.version(), Some(Version::Http1_1));
    /// ```
    pub fn version(&self) -> Option<Version> {
        self.version
    }

    /// Get a header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let res = crux_http::testing::ResponseBuilder::ok()
    /// #   .header("Content-Length", "1")
    /// #   .build();
    /// assert!(res.header("Content-Length").is_some());
    /// ```
    pub fn header(&self, name: impl Into<HeaderName>) -> Option<&HeaderValues> {
        self.headers.get(name)
    }

    /// Get an HTTP header mutably.
    pub fn header_mut(&mut self, name: impl Into<HeaderName>) -> Option<&mut HeaderValues> {
        self.headers.get_mut(name)
    }

    /// Remove a header.
    pub fn remove_header(&mut self, name: impl Into<HeaderName>) -> Option<HeaderValues> {
        self.headers.remove(name)
    }

    /// Insert an HTTP header.
    pub fn insert_header(&mut self, key: impl Into<HeaderName>, value: impl ToHeaderValues) {
        self.headers.insert(key, value);
    }

    /// Append an HTTP header.
    pub fn append_header(&mut self, key: impl Into<HeaderName>, value: impl ToHeaderValues) {
        self.headers.append(key, value);
    }

    /// An iterator visiting all header pairs in arbitrary order.
    #[must_use]
    pub fn iter(&self) -> headers::Iter<'_> {
        self.headers.iter()
    }

    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    #[must_use]
    pub fn iter_mut(&mut self) -> headers::IterMut<'_> {
        self.headers.iter_mut()
    }

    /// An iterator visiting all header names in arbitrary order.
    #[mu
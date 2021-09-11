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
    #[must_use]
    pub fn header_names(&self) -> headers::Names<'_> {
        self.headers.names()
    }

    /// An iterator visiting all header values in arbitrary order.
    #[must_use]
    pub fn header_values(&self) -> headers::Values<'_> {
        self.headers.values()
    }

    /// Get the response content type as a `Mime`.
    ///
    /// Gets the `Content-Type` header and parses it to a `Mime` type.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    ///
    /// # Panics
    ///
    /// This method will panic if an invalid MIME type was set as a header.
    ///
    /// # Examples
    ///
    /// ```
    /// # let res = crux_http::testing::ResponseBuilder::ok()
    /// #   .header("Content-Type", "application/json")
    /// #   .build();
    /// use crux_http::http::mime;
    /// assert_eq!(res.content_type(), Some(mime::JSON));
    /// ```
    pub fn content_type(&self) -> Option<Mime> {
        self.header(CONTENT_TYPE)?.last().as_str().parse().ok()
    }

    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    pub fn take_body(&mut self) -> Option<Body> {
        self.body.take()
    }

    pub fn with_body<NewBody>(self, body: NewBody) -> Response<NewBody> {
        Response {
            body: Some(body),
            headers: self.headers,
            status: self.status,
            version: self.version,
        }
    }
}

impl Response<Vec<u8>> {
    pub(crate) fn new_with_status(status: http::StatusCode) -> Self {
        let headers = new_headers();

        Response {
            status,
            headers,
            version: None,
            body: None,
        }
    }

    /// Reads the entire request body into a byte buffer.
    ///
    /// This method can be called after the body has already been read, but will
    /// produce an empty buffer.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> crux_http::Result<()> {
    /// # let mut res = crux_http::testing::ResponseBuilder::ok()
    /// #   .header("Content-Type", "application/json")
    /// #   .body(vec![0u8, 1])
    /// #   .build();
    /// let bytes: Vec<u8> = res.body_bytes()?;
    /// # Ok(()) }
    /// ```
    pub fn body_bytes(&mut self) -> crate::Result<Vec<u8>> {
        self.body
            .take()
            .ok_or_else(|| crate::Error::new(Some(self.status()), "Body had no bytes"))
    }

    /// Reads the entire response body into a string.
    ///
    /// This method can be called after the body has already been read, but will
    /// produce an empty buffer.
    ///
    /// # Encodings
    ///
    /// If the "encoding" feature is enabled, this method tries to decode the body
    /// with the encoding that is specified in the Content-Type header. If the header
    /// does not specify an encoding, UTF-8 is assumed. If the "encoding" feature is
    /// disabled, Surf only supports reading UTF-8 response bodies. The "encoding"
    /// feature is enabled by default.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// If the body cannot be interpreted because the encoding is unsupported or
    /// incorrect, an `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> crux_http::Result<()> {
    /// # let mut res = crux_http::testing::ResponseBuilder::ok()
    /// #   .header("Content-Type", "application/json")
    /// #   .body("hello".to_string().into_bytes())
    /// #   .build();
    /// let string: String = res.body_string()?;
    /// assert_eq!(string, "hello");
    /// # Ok(()) }
    /// ```
    pub fn body_string(&mut self) -> crate::Result<String> {
        let bytes = self.body_bytes()?;

        let mime = self.content_type();
        let claimed_encoding = mime
            .as_ref()
            .and_then(|mime| mime.param("charset"))
            .map(|name| name.to_string());
        Ok(decode_body(bytes, claimed_encoding.as_deref())?)
    }

    /// Reads and deserialized the entire request body from json.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// If the body cannot be interpreted as valid json for the target type `T`,
    /// an `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde::{Deserialize, Serialize};
    /// # fn main() -> crux_http::Result<()> {
    /// # let mut res = crux_http::testing::ResponseBuilder::ok()
    /// #   .header("Content-Type", 
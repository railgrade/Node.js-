use crate::http::{
    self,
    headers::{self, HeaderName, HeaderValues, ToHeaderValues},
    Body, Mime, StatusCode, Version,
};

use futures_util::io::AsyncRead;
use serde::de::DeserializeOwned;

use std::fmt;
use std::io;
use std::ops::Index;
use std::pin::Pin;
use std::task::{Context, Poll};

use super::decode::decode_body;

pin_project_lite::pin_project! {
    /// An HTTP response that exposes async methods, for use inside middleware.
    ///
    /// If you're not writing middleware you'll never need to interact with
    /// this type and can probably ignore it.
    pub struct ResponseAsync {
        #[pin]
        res: crate::http::Response,
    }
}

impl ResponseAsync {
    /// Create a new instance.
    pub(crate) fn new(res: http::Response) -> Self {
        Self { res }
    }

    /// Get the HTTP status code.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crux_http::client::Client;
    /// # async fn middleware(client: Client) -> crux_http::Result<()> {
    /// let res = client.get("https://httpbin.org/get").await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn status(&self) -> StatusCode {
        self.res.status()
    }

    /// Get the HTTP protocol version.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crux_http::client::Client;
    /// # async fn middleware(client: Client) -> crux_http::Result<()> {
    /// use crux_http::http::Version;
    ///
    /// let res = client.get("https://httpbin.org/get").await?;
    /// assert_eq!(res.version(), Some(Version::Http1_1));
    /// # Ok(()) }
    /// ```
    pub fn version(&self) -> Option<Version> {
        self.res.version()
    }

    /// Get a header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crux_http::client::Client;
    /// # async fn middleware(client: Client) -> crux_http::Result<()> {
    /// let res = client.get("https://httpbin.org/get").await?;
    /// assert!(res.header("Content-Length").is_some());
    /// # Ok(()) }
    /// ```
    pub fn header(&self, name: impl Into<HeaderName>) -> Option<&HeaderValues> {
        self.res.header(name)
    }

    /// Get an HTTP header mutably.
    pub fn header_mut(&mut self, name: impl Into<HeaderName>) -> Option<&mut HeaderValues> {
        self.res.header_mut(name)
    }

    /// Remove a header.
    pub fn remove_header(&mut self, name: impl Into<HeaderName>) -> Option<HeaderValues> {
        self.res.remove_header(name)
    }

    /// Insert an HTTP header.
    pub fn insert_header(&mut self, key: impl Into<HeaderName>, value: impl ToHeaderValues) {
        self.res.insert_header(key, value);
    }

    /// Append an HTTP header.
    pub fn append_header(&mut self, key: impl Into<HeaderName>, value: impl ToHeaderValues) {
        self.res.append_header(key, value);
    }

    /// An iterator visiting all header pairs in arbitrary order.
    #[must_use]
    pub fn iter(&self) -> headers::Iter<'_> {
        self.res.iter()
    }

    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    #[must_use]
    pub fn iter_mut(&mut self) -> headers::IterMut<'_> {
        self.res.iter_mut()
    }

    /// An iterator visiting all header names in arbitrary order.
    #[must_use]
    pub fn header_names(&self) -> headers::Names<'_> {
        self.res.header_names()
    }

    /// An iterator visiting all header values in arbitrary order.
    #[must_use]
    pub fn header_values(&self) -> headers::Values<'_> {
        self.res.header_values()
    }

    /// Get a response scoped extension value.
    #[must_use]
    pub fn ext<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.res.ext().get()
    }

    /// Set a response scoped extension value.
    pub fn insert_ext<T: Send + Sync + 'static>(&mut self, val: T) {
        self.res.ext_mut().insert(val);
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
    /// ```no_run
    /// # use crux_http::client::Client;
  
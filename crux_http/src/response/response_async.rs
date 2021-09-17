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
    pub fn status(&self) -> Status
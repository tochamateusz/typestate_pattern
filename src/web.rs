use crate::prelude::*;

#[derive(Debug)]
pub struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

// region:--States
#[derive(Default, Clone)]
pub struct NoUrl;

#[derive(Default, Clone)]
pub struct Url(String);

#[derive(Default, Clone)]
pub struct NoMethod;

#[derive(Default, Clone)]
pub struct Method(String);
// endregion :--States

#[derive(Default, Clone)]
pub struct RequestBuilder<U, M> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder<NoUrl, NoMethod> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

impl RequestBuilder<Url, Method> {
    pub fn build(self) -> Result<Request> {
        Ok(Request {
            url: self.url.0,
            method: self.method.0,
            headers: self.headers,
            body: self.body,
        })
    }
}

impl<U, M> RequestBuilder<U, M> {
    pub fn url(mut self, url: impl Into<String>) -> RequestBuilder<Url, M> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn method(mut self, method: impl Into<String>) -> RequestBuilder<U, Method> {
        RequestBuilder {
            url: self.url,
            method: Method(method.into()),
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body.insert(body.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}

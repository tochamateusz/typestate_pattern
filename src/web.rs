use crate::prelude::{Error, Result};

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Default, Clone)]
pub struct RequestBuilder {
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Request> {
        let Some(url) = self.url else {
            let request = Err(Error::Static("No url"));
            return request;
        };
        let method = self.method.unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url: url.to_string(),
            method,
            headers: self.headers,
            body: self.body,
        })
    }
}

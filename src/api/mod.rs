use std::marker::PhantomData;
use crate::{
    Result,
    Scope,
};
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;

pub mod video;

pub const API_URL: &str = "https://api.dailymotion.com";

/// Generic type, `P`, is the type of parameters to pass.
pub struct Endpoint<P: EndpointParams, R: DeserializeOwned> {
    // request info
    pub url: &'static str,
    pub request_method: HttpRequestMethod,

    // auth
    pub requires_authentication: bool,
    pub required_scopes: &'static [Scope],

    // phantom data
    pub phantom: (PhantomData<P>, PhantomData<R>),
}

impl<P: EndpointParams, R: DeserializeOwned> Endpoint<P, R> {
    /// Makes a request
    ///
    /// # Errors
    /// * [`crate::Error::ReqwestError`]
    pub fn call(&self, params: &P) -> Result<R>{
        if self.requires_authentication {
            unimplemented!();
        }

        let mut url = String::from(API_URL);
        url.push_str(&params.build_url(self));

        let client = reqwest::blocking::Client::new();
        self.request_method.builder(&client, &url)
            .send()?
            .json()
            .map_err(std::convert::Into::into)
    }
}

#[macro_export]
macro_rules! endpoint {
    ($url:expr, $request_method:expr, $requires_authentication:expr, $required_scopes:expr) => {
        Endpoint {
            url: $url,
            request_method: $request_method,
            requires_authentication: $requires_authentication,
            required_scopes: $required_scopes,
            phantom: (std::marker::PhantomData, std::marker::PhantomData),
        }
    }
}

pub trait EndpointParams {
    /// Builds the full endpoint URL component.
    /// E.g., for [`video::GET_FROM_ID`], "/video/{}" -> "/video/x8gkpx9" if the given id is "x8gkpx9".
    fn build_url<P: EndpointParams, R: DeserializeOwned>(&self, endpoint: &Endpoint<P, R>) -> String;
}

pub enum HttpRequestMethod {
    Get,
    Post,
    Delete,
}

impl HttpRequestMethod {
    // idk what to call this lol
    pub fn builder<C, R>(&self, client: &C, url: &str) -> R
            where C: Reqwester + Reqwester<RequestBuilder = R> {
        match self {
            Self::Get => client.get(url),
            Self::Post => client.post(url),
            Self::Delete => client.delete(url),
        }
    }
}

pub trait Reqwester {
    type RequestBuilder;

    fn get(&self, url: impl IntoUrl) -> Self::RequestBuilder;
    fn post(&self, url: impl IntoUrl) -> Self::RequestBuilder;
    fn delete(&self, url: impl IntoUrl) -> Self::RequestBuilder;
}

impl Reqwester for reqwest::Client {
    type RequestBuilder = reqwest::RequestBuilder;

    fn get(&self, url: impl IntoUrl) -> Self::RequestBuilder {
        self.get(url)
    }

    fn post(&self, url: impl IntoUrl) -> Self::RequestBuilder {
        self.post(url)
    }

    fn delete(&self, url: impl IntoUrl) -> Self::RequestBuilder {
        self.delete(url)
    }
}

impl Reqwester for reqwest::blocking::Client {
    type RequestBuilder = reqwest::blocking::RequestBuilder;

    fn get(&self, url: impl IntoUrl) -> Self::RequestBuilder {
        self.get(url)
    }

    fn post(&self, url: impl IntoUrl) -> Self::RequestBuilder {
        self.post(url)
    }

    fn delete(&self, url: impl IntoUrl) -> Self::RequestBuilder {
        self.delete(url)
    }
}


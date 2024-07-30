use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::{
    codes::HttpError,
    headers::{HttpMethod, HttpRequestHeader, HttpResponseHeader},
};

#[derive(Debug)]
pub enum HttpFunctionCallError {
    PathNotFound,
    FunctionFailure,
    InvalidMethod,
}

pub type HttpFunction = Arc<
    dyn Fn(HttpRequestHeader) -> Pin<Box<dyn Future<Output = HttpFunctionReturnType> + Send>>
        + Send
        + Sync,
>;
pub type HttpFunctionReturnType = Result<(HttpResponseHeader, Vec<u8>), HttpError>;

#[derive(Clone)]
pub struct HttpPath {
    path: &'static str,
    function: HttpFunction,
    req_type: HttpMethod,
}

pub trait HttpPathMethods {
    fn new() -> Self;
    fn get(&mut self, path: &'static str, function: HttpFunction) -> Self;
    fn post(&mut self, path: &'static str, function: HttpFunction) -> Self;
    fn call(&self, path: &'static str) -> Result<(), HttpFunctionCallError>;
}

impl HttpPathMethods for Vec<HttpPath> {
    fn new() -> Self {
        Vec::new()
    }

    fn get(&mut self, path: &'static str, function: HttpFunction) -> Self {
        self.push(HttpPath {
            path,
            function,
            req_type: HttpMethod::GET,
        });

        self.to_vec()
    }

    fn post(&mut self, path: &'static str, function: HttpFunction) -> Self {
        self.push(HttpPath {
            path,
            function,
            req_type: HttpMethod::POST,
        });

        self.to_vec()
    }

    fn call(&self, path: &'static str) -> Result<(), HttpFunctionCallError> {
        todo!()
    }
}

pub fn into_http<F, Fut>(f: F) -> HttpFunction
where
    F: Fn(HttpRequestHeader) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = HttpFunctionReturnType> + Send + 'static,
{
    Arc::new(move |req| Box::pin(f(req)))
}

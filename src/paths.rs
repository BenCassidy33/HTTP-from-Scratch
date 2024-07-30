use std::future::Future;
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

pub type HttpFunctionReturnType =
    Box<dyn Future<Output = Result<(HttpResponseHeader, Vec<u8>), HttpError>> + Send>;

type HttpFunction = fn() -> HttpFunctionReturnType;

pub struct HttpPath {
    path: &'static str,
    function: HttpFunction,
    req_type: HttpMethod,
}

pub trait HttpPathMethods {
    fn new() -> Self;
    fn get(&mut self, path: &'static str, function: HttpFunction);
    fn post(&mut self, path: &'static str, function: HttpFunction);
    fn call(&self, path: &'static str) -> Result<(), HttpFunctionCallError>;
}

impl HttpPathMethods for Vec<HttpPath> {
    fn new() -> Self {
        Vec::new()
    }

    fn get(&mut self, path: &'static str, function: HttpFunction) {
        self.push(HttpPath {
            path,
            function,
            req_type: HttpMethod::GET,
        })
    }

    fn post(&mut self, path: &'static str, function: HttpFunction) {
        self.push(HttpPath {
            path,
            function,
            req_type: HttpMethod::POST,
        })
    }

    fn call(&self, path: &'static str) -> Result<(), HttpFunctionCallError> {
        todo!()
    }
}

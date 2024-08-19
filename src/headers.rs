use super::*;
use std::{any::Any, collections::HashMap};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum HttpMethod {
    #[default]
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl HttpMethod {
    pub fn from_str(string: String) -> Self {
        return match string.as_str() {
            "get" => Self::GET,
            "head" => Self::HEAD,
            "post" => Self::POST,
            "put" => Self::PUT,
            "delete" => Self::DELETE,
            "connect" => Self::CONNECT,
            "options" => Self::OPTIONS,
            "trace" => Self::TRACE,
            _ => panic!("bad request"),
        };
    }
}

#[derive(Debug, Clone, Default)]
pub enum HttpVersion {
    #[default]
    HTTP11,
    HTTP2,
    //#[serde(rename = "HTTP/3")]
    //HTTP3,
}

#[derive(Debug)]
pub struct HttpResponseHeader {
    pub http_method: headers::HttpMethod,
    pub status: codes::HttpStatus,
    pub content_type: content::ContentType,
    pub content_length: usize,
    pub http_version: HttpVersion,
}

pub struct HttpResponse {
    header: HttpResponseHeader,
    body: String,
}

#[derive(Debug, Default, Clone)]
pub struct HttpRequestHeader {
    pub authority: String,
    pub method: headers::HttpMethod,
    pub host: String,
    pub path: String,
    pub scheme: String,
    pub accept: Vec<content::ContentType>,
    pub accept_encoding: Vec<content::AcceptEncoding>,
    pub http_version: headers::HttpVersion,
    pub request_headers: String, // TODO
    pub request_body: String,
    pub user_agent: String,
    pub body: String,
    pub priority: String,
    pub accept_language: String,
    pub sec_fetch_site: String,
    pub upgrade_insecure_requests: String,
    pub sec_fetch_user: String,
    pub sec_fetch_dest: String,
    pub sec_fetch_mode: String,
    pub connection: String,
}

impl HttpRequestHeader {
    pub fn from_map(map: HashMap<String, String>) -> Self {
        let mut res = Self::default();
        res.path = map["path"].clone();
        res
    }
}

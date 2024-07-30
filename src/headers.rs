use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpVersion {
    #[serde(rename = "HTTP/1.1")]
    HTTP11,
    #[serde(rename = "HTTP/2")]
    HTTP2,
    //#[serde(rename = "HTTP/3")]
    //HTTP3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRequestHeader {
    pub authority: String,
    pub method: headers::HttpMethod,
    pub path: String,
    pub scheme: String,
    pub accept: content::ContentTypes,
    #[serde(flatten)]
    pub http_version: headers::HttpVersion,
    #[serde(rename = "Request Headers")]
    pub request_headers: String, // TODO
    #[serde(rename = "Request Body")]
    pub request_body: String,
    #[serde(rename = "user-agent")]
    pub user_agent: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpResponseHeader {
    #[serde(flatten)]
    pub status: codes::HttpStatus,
    #[serde(rename = "version")]
    pub http_version: headers::HttpVersion,
}

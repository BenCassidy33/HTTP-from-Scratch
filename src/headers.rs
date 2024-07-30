use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum HttpVersion {
    #[serde(rename = "HTTP/1.1")]
    #[default]
    HTTP11,
    #[serde(rename = "HTTP/2")]
    HTTP2,
    //#[serde(rename = "HTTP/3")]
    //HTTP3,
}

#[derive(Serialize, Deserialize)]
pub struct HttpResponseHeader {
    #[serde(flatten)]
    pub status: codes::HttpStatus,
    #[serde(rename = "version")]
    pub http_version: headers::HttpVersion,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HttpRequestHeader {
    pub authority: String,
    pub method: headers::HttpMethod,
    pub path: String,
    pub scheme: String,
    pub accept: Vec<content::ContentTypes>,
    pub accept_encoding: Vec<content::AcceptEncoding>,
    #[serde(flatten)]
    pub http_version: headers::HttpVersion,
    #[serde(rename = "Request Headers")]
    pub request_headers: String, // TODO
    #[serde(rename = "Request Body")]
    pub request_body: String,
    #[serde(rename = "user-agent")]
    pub user_agent: String,
    pub body: String,
    pub priority: String,
}

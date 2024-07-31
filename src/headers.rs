use super::*;

#[derive(Debug, Clone, Default)]
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
    pub status: codes::HttpStatus,
    pub http_version: headers::HttpVersion,
}

#[derive(Debug, Default)]
pub struct HttpRequestHeader {
    pub authority: String,
    pub method: headers::HttpMethod,
    pub host: String,
    pub path: String,
    pub scheme: String,
    pub accept: Vec<content::ContentTypes>,
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

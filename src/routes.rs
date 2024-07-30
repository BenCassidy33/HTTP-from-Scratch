use crate::{
    codes::{HttpOk, HttpStatus},
    headers::{HttpMethod, HttpResponseHeader, HttpVersion},
};

pub async fn index(_: crate::headers::HttpRequestHeader) -> crate::paths::HttpFunctionReturnType {
    let html = tokio::fs::read_to_string("pages/index.html").await.unwrap();

    let response_header = HttpResponseHeader {
        status: HttpStatus::Ok(HttpOk::Ok),
        http_version: HttpVersion::HTTP11,
    };

    return Ok((response_header, html.as_bytes().into()));
}

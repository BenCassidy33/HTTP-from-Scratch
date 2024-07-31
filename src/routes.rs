use crate::{
    codes::{HttpOk, HttpStatus},
    headers::HttpResponseHeader,
};

pub async fn index(
    header: crate::headers::HttpRequestHeader,
) -> crate::paths::HttpFunctionReturnType {
    let html = tokio::fs::read_to_string("pages/index.html").await.unwrap();

    let response_header = HttpResponseHeader {
        status: HttpStatus::Ok(HttpOk::Ok),
        content_length: html.len(),
        http_method: header.method.clone(),
        content_type: crate::ContentType::Html,
        http_version: header.http_version.clone(),
    };

    return Ok((response_header, html.bytes().collect()));
}

pub async fn user(
    header: crate::headers::HttpRequestHeader,
) -> crate::paths::HttpFunctionReturnType {
    let html = tokio::fs::read_to_string("pages/user.html").await.unwrap();

    let response_header = HttpResponseHeader {
        status: HttpStatus::Ok(HttpOk::Ok),
        content_length: html.len(),
        http_method: header.method.clone(),
        content_type: crate::ContentType::Html,
        http_version: header.http_version.clone(),
    };

    return Ok((response_header, html.bytes().collect()));
}

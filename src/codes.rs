// codes found here: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#information_responses

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum HttpStatus {
    Ok(HttpOk),
    Redirect(HttpRedirect),
    ServerError(HttpServerError),
    Calamitous(HttpClientError),
}

#[derive(Serialize, Deserialize)]
pub enum HttpOk {
    Ok = 200,
    Created = 201,
    Accpeted = 202,
    #[serde(rename = "Non-Authoritative Information")]
    NonAuthoritativeInformation = 203,
    #[serde(rename = "No Content")]
    NoContent = 204,
    #[serde(rename = "Reset Content")]
    ResetContent = 205,
    #[serde(rename = "Parital Content")]
    PartialContent = 206,
    #[serde(rename = "Multi-Status")]
    MultiStatus = 207,
    #[serde(rename = "Already Reported")]
    AlreadyReported = 208,
    #[serde(rename = "IM Used")]
    ImUsed = 226,
}

#[derive(Serialize, Deserialize)]
pub enum HttpRedirect {
    #[serde(rename = "Multiple Choices")]
    MultipleChoices = 300,
    #[serde(rename = "Moved Permanently")]
    MovedPermanently = 301,
    Found = 302,
}

#[derive(Serialize, Deserialize)]
pub enum HttpServerError {}
#[derive(Serialize, Deserialize)]
pub enum HttpClientError {
    #[serde(rename = "Bad Request")]
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    #[serde(rename = "Not Found")]
    NotFound,
}

pub enum HttpError {
    HttpServerError(HttpServerError),
    HttpClientError(HttpClientError),
}

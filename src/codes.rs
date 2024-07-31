// codes found here: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#information_responses

#[derive(Debug, Clone)]
pub enum HttpStatus {
    Ok(HttpOk),
    Redirect(HttpRedirect),
    ServerError(HttpServerError),
    Calamitous(HttpClientError),
}

#[derive(Debug, Clone)]
pub enum HttpOk {
    Ok = 200,
    Created = 201,
    Accpeted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,
}

#[derive(Debug, Clone)]
pub enum HttpRedirect {
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
}

#[derive(Debug, Clone)]
pub enum HttpServerError {}
#[derive(Debug, Clone)]
pub enum HttpClientError {
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
}

#[derive(Debug, Clone)]
pub enum HttpError {
    HttpServerError(HttpServerError),
    HttpClientError(HttpClientError),
}

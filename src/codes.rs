// codes found here: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#information_responses

#[derive(Debug)]
pub enum HttpStatus {
    Ok(HttpOk),
    Redirect(HttpRedirect),
    ServerError(HttpServerError),
    Calamitous(HttpClientError),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum HttpRedirect {
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
}

#[derive(Debug)]
pub enum HttpServerError {}
#[derive(Debug)]
pub enum HttpClientError {
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound,
}

pub enum HttpError {
    HttpServerError(HttpServerError),
    HttpClientError(HttpClientError),
}

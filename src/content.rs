// https://en.wikipedia.org/wiki/Media_type

use std::any::Any;

use crate::{
    codes::HttpStatus,
    headers::{HttpResponseHeader, HttpVersion},
};

#[derive(Debug, Clone)]
pub enum AcceptEncoding {
    Gzip,
    Deflate,
    Br,
    Zstd,
    Identity,
    Any,
    Qvalues,
}

#[derive(Debug, Clone)]
pub enum ContentType {
    Plain,
    Css,
    Csv,
    Html,
    Javascript,
    Xml,
    Json,
    Pdf,
    Sql,
    Zip,
    Mpeg,
    Avif,
    Jpeg,
    Png,
    Ogg,
    Webp,
    Svg,
    Any,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        return match self {
            Self::Any => "any".to_string(),
            Self::Webp => "webp".to_string(),
            Self::Ogg => "ogg".to_string(),
            Self::Png => "png".to_string(),
            Self::Jpeg => "jpeg".to_string(),
            Self::Avif => "avif".to_string(),
            Self::Mpeg => "mpeg".to_string(),
            Self::Zip => "zip".to_string(),
            Self::Sql => "sql".to_string(),
            Self::Pdf => "pdf".to_string(),
            Self::Json => "json".to_string(),
            Self::Xml => "xml".to_string(),
            Self::Javascript => "javascript".to_string(),
            Self::Html => "html".to_string(),
            Self::Csv => "csv".to_string(),
            Self::Css => "css".to_string(),
            Self::Plain => "plain".to_string(),
            Self::Svg => "svg".to_string(),
        };
    }
}

pub fn format_http_header(header: HttpResponseHeader) -> String {
    let code = match header.status.clone() {
        HttpStatus::Ok(code) => code as u16,
        HttpStatus::ServerError(code) => code as u16,
        HttpStatus::Redirect(code) => code as u16,
        HttpStatus::Calamitous(code) => code as u16,
    };

    let pretty_code = match header.status {
        HttpStatus::Ok(_) => "Ok",
        HttpStatus::Redirect(_) => "Redirect",
        HttpStatus::ServerError(_) => "Error",
        HttpStatus::Calamitous(_) => "Error",
    };

    format!(
        "{:?} {:?} {:?}\r\n",
        match header.http_version {
            HttpVersion::HTTP11 => "HTTP/1.1",
            HttpVersion::HTTP2 => "HTTP/2",
        },
        code,
        pretty_code
    )
    .to_string()
}

pub fn format_http_response(header: HttpResponseHeader, body: Vec<u8>) -> String {
    let head = format_http_header(header);
    format!("{}\r\n\r\n{}", head, String::from_utf8(body).unwrap()).to_string()
}

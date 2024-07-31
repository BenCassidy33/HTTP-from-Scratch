pub mod codes;
pub mod content;
pub mod header_parsing;
pub mod headers;
pub mod paths;
pub mod routes;

use codes::HttpClientError;
use content::{AcceptEncoding, ContentTypes};
use core::panic;
use headers::{HttpRequestHeader, HttpResponseHeader, HttpVersion};
use paths::{into_http, HttpPath, HttpPathMethods};
use std::{collections::HashMap, str};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

static HTTP_CONNECTION_FAILED: HttpResponseHeader = HttpResponseHeader {
    status: codes::HttpStatus::Calamitous(HttpClientError::BadRequest),
    http_version: HttpVersion::HTTP11,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let paths = <std::vec::Vec<paths::HttpPath> as HttpPathMethods>::new()
        .get("/", into_http(routes::index))
        .post("/post", into_http(routes::post));

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(process(socket, paths.clone()));
    }
}

async fn process(mut socket: TcpStream, functions: Vec<HttpPath>) {
    eprintln!("Connection Made");

    let mut buffer: Vec<u8> = vec![0; 30 * 1024 * 1024];
    loop {
        let n = socket
            .read(&mut buffer)
            .await
            .expect("Could not read data, Exiting");

        let res = str::from_utf8(&buffer)
            .expect("Invalid Content Type, Expected: UTF-8")
            .trim_matches(char::from(0));
        if n == 0 {
            let _ = socket
                .write_all(format!("{:?}", HTTP_CONNECTION_FAILED).as_bytes())
                .await;
        }

        let header = header_parsing::parse_request_header(res, &mut socket).await;
    }
}

pub mod codes;
pub mod content;
pub mod header_parsing;
pub mod headers;
pub mod paths;
pub mod routes;

use codes::{HttpClientError, HttpStatus};
use content::{format_http_header, format_http_response, ContentType};
use headers::{HttpMethod, HttpRequestHeader, HttpResponseHeader, HttpVersion};
use paths::{into_http, HttpPath, HttpPathMethods};
use std::str;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

static HTTP_CONNECTION_FAILED: HttpResponseHeader = HttpResponseHeader {
    content_type: ContentType::Plain,
    content_length: 0,
    http_method: HttpMethod::GET,
    status: HttpStatus::Calamitous(HttpClientError::NotFound),
    http_version: HttpVersion::HTTP11,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let paths = <std::vec::Vec<paths::HttpPath> as HttpPathMethods>::new()
        .get("/", into_http(routes::index))
        .get("/user", into_http(routes::user));

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(process(socket, paths.clone()));
    }
}

async fn process(mut socket: TcpStream, functions: Vec<HttpPath>) {
    eprintln!("Connection Made");

    let mut buffer: Vec<u8> = vec![0; 30 * 1024 * 1024];
    'outer: loop {
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

        let header: std::collections::HashMap<String, String> =
            header_parsing::parse_request_header(res).await;

        for func in functions.clone().into_iter() {
            if *func.path == header["path"]
                && func.req_type == HttpMethod::from_str(header["method"].clone().to_lowercase())
            {
                let req_header: HttpRequestHeader = HttpRequestHeader::from_map(header);
                let response = (func.function)(req_header.clone()).await;
                match response {
                    Ok((header, body)) => {
                        let response = format_http_response(header, body);
                        let _ = socket.write(response.as_bytes()).await;
                        break 'outer;
                    }

                    Err(http_error_response) => {
                        let _ = socket
                            .write(format_http_header(http_error_response).as_bytes())
                            .await;
                        break 'outer;
                    }
                }
            }
        }
    }
}

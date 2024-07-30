use codes::HttpClientError;
use headers::{HttpRequestHeader, HttpResponseHeader, HttpVersion};
use paths::{into_http, HttpFunctionReturnType, HttpPathMethods};
use routes::*;
use std::{any::Any, collections::HashMap, str};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub mod codes;
pub mod content;
pub mod headers;
pub mod paths;
pub mod routes;

static HTTP_CONNECTION_FAILED: HttpResponseHeader = HttpResponseHeader {
    status: codes::HttpStatus::Calamitous(HttpClientError::BadRequest),
    http_version: HttpVersion::HTTP11,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let paths = <std::vec::Vec<paths::HttpPath> as HttpPathMethods>::new()
        .get("/", into_http(routes::index));

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(process(socket));
    }
}

async fn process(mut socket: TcpStream) {
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
            let res = serde_json::to_value(&HTTP_CONNECTION_FAILED).unwrap();
            let _ = socket.write_all(res.to_string().as_bytes()).await;
        }

        let _header = parse_request_header(res).await;
        let _ = socket.write_all(res.as_bytes()).await;
    }
}

async fn parse_request_header(input: &str) -> HashMap<String, String> {
    let mut request: HashMap<String, String> = HashMap::new();

    for (idx, val) in input
        .split("\r\n")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
    {
        if val.is_empty() {
            continue;
        }

        if idx == 0 {
            let [req_kind, req_path, http_kind] = *val.split(" ").collect::<Vec<&str>>() else {
                panic!("Idk")
            };

            println!(
                "request kind: {:?}, req_path: {:?}, http_kind: {:?}",
                req_kind, req_path, http_kind
            );

            request.insert("HttpMethod".to_string(), req_kind.to_string());
            request.insert("path".to_string(), req_path.to_string());
            request.insert("HttpVersion".to_string(), http_kind.to_string());

            continue;
        }

        let (k, v) = val.split_once(":").expect("Invalid Request");
        request.insert(k.to_string(), v.trim().to_string());
    }

    request
}

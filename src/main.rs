#![feature(async_closure)]

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

        let header = parse_request_header(res, &mut socket).await;
    }
}

async fn parse_request_header(input: &str, socket: &mut TcpStream) -> HttpRequestHeader {
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
                socket.write(b"Invalid Request Formatting");
                panic!("Invalid request formatting")
            };

            request.insert("HttpMethod".to_string(), req_kind.to_string());
            request.insert("path".to_string(), req_path.to_string());
            request.insert("HttpVersion".to_string(), http_kind.to_string());

            continue;
        }

        let (k, v) = val.split_once(":").expect("Invalid Request");
        request.insert(k.to_string(), v.trim().to_string());
    }

    let mut header = HttpRequestHeader::default();

    for (k, v) in request.iter() {
        match k.as_str() {
            "path" => header.path = v.to_string(),
            "host" => header.host = v.to_string(),
            "HttpVersion" => {
                header.http_version = match v.as_str() {
                    "HTTP/1.1" => HttpVersion::HTTP11,
                    "HTTP/2" => HttpVersion::HTTP2,
                    //"HTTP/3" => HttpVersion::HTTP3,
                    _ => {
                        panic!("Invalid Http Version")
                    }
                }
            }
            "User-Agent" => header.user_agent = v.to_string(),
            "Priority" => header.priority = v.to_string(),
            "Accept-Language" => header.accept_language = v.to_string(),
            "Sec-Fetch-Site" => header.sec_fetch_site = v.to_string(),
            "HttpMethod" => {
                header.method = match v.as_str() {
                    "GET" => headers::HttpMethod::GET,
                    "POST" => headers::HttpMethod::POST,
                    "DELETE" => headers::HttpMethod::DELETE,
                    "PUT" => headers::HttpMethod::PUT,
                    "HEAD" => headers::HttpMethod::HEAD,
                    _ => {
                        panic!("Invalid http request kind")
                    }
                }
            }
            "Upgrade-Insecure-Requests" => header.upgrade_insecure_requests = v.to_string(),
            "Sec-Fetch-User" => header.sec_fetch_user = v.to_string(),
            "Sec-Fetch-Dest" => header.sec_fetch_dest = v.to_string(),
            "Accept-Encoding" => {
                header.accept_encoding = v
                    .split(",")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|f| match f.trim() {
                        "gzip" => content::AcceptEncoding::Gzip,
                        "deflate" => content::AcceptEncoding::Deflate,
                        "br" => content::AcceptEncoding::Br,
                        "zstd" => content::AcceptEncoding::Zstd,
                        "identity" => content::AcceptEncoding::Identity,
                        "*" => content::AcceptEncoding::Any,
                        ";q=" => content::AcceptEncoding::Qvalues,
                        t => {
                            println!("Impliment Accept-Encoding: {:?}", t);
                            content::AcceptEncoding::Any
                        }
                    })
                    .collect()
            }

            //text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8
            "Accept" => {
                header.accept = Vec::new();
                v.split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .for_each(|f| match *f {
                        "text/html" => header.accept.push(ContentTypes::Html),
                        "application/xhtml+xml" => header.accept.push(ContentTypes::Html),
                        "application/xml;q=0.9" => header.accept.push(ContentTypes::Xml),
                        "image/avif" => header.accept.push(ContentTypes::Avif),
                        "image/webp" => header.accept.push(ContentTypes::Webp),
                        "image/png" => header.accept.push(ContentTypes::Png),
                        "image/svg+xml" => header.accept.push(ContentTypes::Svg),
                        "*/*;q=0.8" => header.accept.push(ContentTypes::Any),
                        t => println!("Impliment Accept: {:?}", t),
                    })
            }
            "Sec-Fetch-Mode" => header.sec_fetch_mode = v.to_string(),
            "Connection" => header.connection = v.to_string(),
            "Host" => header.host = v.to_string(),

            t => {
                println!("Impliment Http Header: {}", t)
            }
        }
    }

    header
}

use codes::HttpClientError;
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
            let res = serde_json::to_value(&HTTP_CONNECTION_FAILED).unwrap();
            let _ = socket.write_all(res.to_string().as_bytes()).await;
        }

        let header = parse_request_header(res).await;
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

    let mut header = HttpRequestHeader::default();

    for (k, v) in request.iter() {
        match k.as_str() {
            "path" => header.path = v.to_string(),
            "host" => header.host = v,
            "HttpVersion" => header.http_version = match v {
                "HTTP/1.1" => HttpVersion::HTTP11,
                "HTTP/2" => HttpVersion::HTTP2,
                //"HTTP/3" => HttpVersion::HTTP3,
            }
            "User-Agent" => header.user_agent = v,
            "Priority" => header.priority = v,
            "Accept-Language" => header.accept_language = v,
            "Sec-Fetch-Site" => header.sec_fetch_site = v,
            "HttpMethod" => header.method = match v {
                "GET" => headers::HttpMethod::GET,
                "POST" => headers::HttpMethod::POST,
                "DELETE" => headers::HttpMethod::DELETE,
                "PUT" => headers::HttpMethod::PUT,
                "HEAD" => headers::HttpMethod::HEAD,
            },
            "Upgrade-Insecure-Requests" => header.upgrade_insecure_requests = v,
            "Sec-Fetch-User" => header.sec_fetch_user = v,
            "Accept-Encoding" => header.accept_encoding = v.split(",").collect::<Vec<&str>>().iter().map(|f| match *f {
                "gzip" => content::AcceptEncoding::Gzip,
                "deflate" => content::AcceptEncoding::Deflate,
                "br" => content::AcceptEncoding::Br,
                "zstd" => content::AcceptEncoding::Zstd,
                "identity" => content::AcceptEncoding::Identity,
                "*" => content::AcceptEncoding::Any,
                ";q=" => content::AcceptEncoding::Qvalues
            }).collect()
        }
    }

    println!("Req: {:#?}", request);
    request
}

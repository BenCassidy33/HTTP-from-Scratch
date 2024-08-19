use std::collections::HashMap;

use crate::headers::HttpMethod;

pub async fn parse_request_header(header: &str) -> HashMap<String, String> {
    let mut request: HashMap<String, String> = HashMap::new();

    for l in header.lines().collect::<Vec<&str>>() {
        let line = l.splitn(2, " ").collect::<Vec<&str>>();

        if line[0].to_lowercase() == "get" || line[0].to_lowercase() == "post" {
            request.insert(
                "method".to_owned(),
                format!("{:?}", HttpMethod::from_str(line[0].to_lowercase())),
            );

            let rest = line[1].splitn(2, " ").collect::<Vec<&str>>();

            request.insert("path".to_string(), rest[0].to_string());
            request.insert("version".to_string(), rest[1].to_string());
        }

        if line.len() < 2 {
            continue;
        }

        request.insert(line[0].to_string(), line[1].to_string());
    }

    println!("{request:#?}");
    request
}

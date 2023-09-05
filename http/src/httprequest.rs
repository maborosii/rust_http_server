use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Others,
}

impl From<&str> for Method {
    fn from(source: &str) -> Method {
        match source {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Others,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Others,
}

impl From<&str> for Version {
    fn from(source: &str) -> Version {
        match source {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Others,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub header: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(source: String) -> Self {
        let mut parsed_method = Method::Others;
        let mut parsed_version = Version::Others;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_header = HashMap::new();
        let mut parsed_body = "";

        for line in source.lines() {
            if line.contains("HTTP") {
                (parsed_method, parsed_version, parsed_resource) = process_req_line(line);
            } else if line.contains(':') {
                let (k, v) = process_header_line(line);
                parsed_header.insert(k, v);
            } else if line.is_empty() {
            } else {
                parsed_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            header: parsed_header,
            body: parsed_body.to_string(),
        }
    }
}

fn process_req_line(source: &str) -> (Method, Version, Resource) {
    let mut words = source.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        version.into(),
        Resource::Path(resource.to_string()),
    )
}

fn process_header_line(source: &str) -> (String, String) {
    let mut key = String::from("");
    let mut value = String::from("");
    let mut header_parse = source.split(':');
    if let Some(k) = header_parse.next() {
        key = k.to_string();
    }
    if let Some(v) = header_parse.next() {
        value = v.to_string();
    }
    (key, value)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET)
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/2.0".into();
        assert_eq!(m, Version::V2_0)
    }
    #[test]
    fn test_read_http() {
        let s: String = String::from(
            "GET /index HTTP/1.1\r\n\
        Host: localhost\r\n\
        User-Agent: Curl/7.64.1\r\n\
        Accept: */*\r\n\r\n",
        );
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("User-Agent".into(), " Curl/7.64.1".into());
        headers_expected.insert("Accept".into(), " */*".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::GET, req.method);
        assert_eq!(Resource::Path("/index".to_string()), req.resource);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(headers_expected, req.header);
    }
}

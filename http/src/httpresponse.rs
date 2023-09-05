use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    header: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        HttpResponse {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "".into(),
            header: None,
            body: None,
        }
    }
}
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        header: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut resp = HttpResponse::default();
        if status_code != "200" {
            resp.status_code = status_code;
        }
        resp.header = match &header {
            Some(_h) => header,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        resp.status_text = match status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(), //
        };
        resp.body = body;
        resp
    }
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let resp_string = String::from(self.clone());
        let _ = write!(write_stream, "{}", resp_string);
        Ok(())
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }
    fn version(&self) -> &str {
        self.version
    }
    fn header(&self) -> String {
        let temp_header = self.header.clone().unwrap();
        let mut header_string = "".to_string();
        for (k, v) in temp_header.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(source: HttpResponse<'a>) -> String {
        let temp_source = source.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            temp_source.version(),
            temp_source.status_code(),
            temp_source.status_text(),
            temp_source.header(),
            temp_source.body(),
            source.body.unwrap().len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("nothing for now".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            header: {
                let mut m = HashMap::new();
                m.insert("Content-Type", "text/html");
                Some(m)
            },
            body: Some("nothing for now".into()),
        };
        assert_eq!(response_actual, response_expected)
    }
}

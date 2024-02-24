use std::collections::HashMap;
use std::io::Result;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let response_string = String::from(self);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    pub fn version(&self) -> &'a str {
        self.version
    }
    pub fn status_code(&self) -> &'a str {
        self.status_code
    }
    pub fn status_text(&self) -> &'a str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        match &self.headers {
            Some(headers) => {
                let mut header_string: String = String::new();
                for (k, v) in headers.iter() {
                    header_string = format!("{}{}:{}\r\n", header_string, k, v);
                }
                header_string
            }
            None => String::new(),
        }
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }

    pub fn content_length(&self) -> usize {
        match &self.body {
            Some(b) => b.len(),
            None => 0,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    #[inline]
    fn from(res: HttpResponse<'a>) -> Self {
        String::from(&res)
    }
}

impl<'a> From<&HttpResponse<'a>> for String {
    fn from(res: &HttpResponse<'a>) -> Self {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            res.content_length(),
            res.body()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_default() {
        let response: HttpResponse = HttpResponse::default();
        println!("{:?}", response);
    }

    #[test]
    fn test_response_new() {
        let response = HttpResponse::new("400", None, Some("Xin chao cac ban".into()));
        println!("{:?}", response);
    }

    #[test]
    fn test_response_struct_creation_200() {
        let actual_response = HttpResponse::new(
            "200",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let expected_response = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(expected_response, actual_response);
    }

    #[test]
    fn test_response_struct_creation_400() {
        let actual_response = HttpResponse::new(
            "404",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let expected_response = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(expected_response, actual_response);
    }

    #[test]
    fn test_http_response_creation() {
        let response = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", " text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        let actual_resp_string: String = response.into();
        let expected_resp_string = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 21st Dec 2020";
        assert_eq!(expected_resp_string, actual_resp_string);
    }
}

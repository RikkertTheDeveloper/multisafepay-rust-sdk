use curl::easy::{Easy, List};

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
        }
    }
}

pub struct HttpRequestBuilder<'a> {
    easy: Easy,
    method: Option<HttpMethod>,
    url: Option<&'a str>,
    payload: Option<&'a str>,
    api_key: Option<&'a str>,
}

impl<'a> HttpRequestBuilder<'a> {
    pub fn new() -> Self {
        Self {
            easy: Easy::new(),
            method: None,
            url: None,
            payload: None,
            api_key: None,
        }
    }

    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }

    pub fn url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    pub fn payload(mut self, payload: &'a str) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn api_key(mut self, api_key: &'a str) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn execute(&mut self) -> Result<String, curl::Error> {
        let method = self
            .method
            .as_ref()
            .expect("HTTP method not specified")
            .as_str();
        let url = self.url.expect("URL not specified");

        self.easy.url(url)?;
        self.easy.custom_request(method)?;

        if let Some(payload_data) = self.payload {
            self.easy.post(true)?;
            self.easy.post_fields_copy(payload_data.as_bytes())?;
        }

        if let Some(api_key) = self.api_key {
            let mut headers = List::new();
            headers.append(&format!("api_key: {}", api_key))?;
            headers.append("accept: application/json")?;
            headers.append("content-type: application/json")?;
            self.easy.http_headers(headers)?;
        }

        let mut response_data = Vec::new();
        {
            let mut transfer = self.easy.transfer();
            transfer.write_function(|data| {
                response_data.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }

        let response_string = String::from_utf8_lossy(&response_data).to_string();
        Ok(response_string)
    }
}

pub struct Endpoint {
    url: String,
    api_key: String,
}

impl Endpoint {
    pub fn new(url: String, api_key: String) -> Self {
        Self { url, api_key }
    }

    pub fn format(&self) -> String {
        assert!(self.verify(), "Failed to verify endpoint string.");
        format!("{}?api-key={}", self.url, self.api_key)
    }

    fn verify_format(&self) -> bool {
        self.url.is_empty() || !self.api_key.is_empty()
    }

    pub fn verify(&self) -> bool {
        self.verify_format()
    }
}

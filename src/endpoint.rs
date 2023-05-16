use crate::http::HttpMethod;

pub enum EndpointAction {
    CreateTransaction,
}

impl EndpointAction {
    pub fn builder() -> EndpointActionBuilder {
        EndpointActionBuilder::new()
    }

    pub fn endpoint(&self, api_key: String) -> Endpoint {
        match self {
            Self::CreateTransaction => {
                Endpoint::new("https://testapi.multisafepay.com/v1/json/orders".to_owned(), HttpMethod::POST, api_key)
            }
        }
    }
}

pub struct EndpointActionBuilder {
    api_key: Option<String>,
}

impl EndpointActionBuilder {
    pub fn new() -> Self {
        Self { api_key: None }
    }

    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn build(self, action: EndpointAction) -> Endpoint {
        let api_key = self.api_key.expect("API key not specified");
        action.endpoint(api_key)
    }
}

pub struct Endpoint {
    url: String,
    method: HttpMethod,
    api_key: String,
}

impl Endpoint {
    pub fn new(url: String, method: HttpMethod, api_key: String) -> Self {
        Self {
            url,
            method,
            api_key,
        }
    }

    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}

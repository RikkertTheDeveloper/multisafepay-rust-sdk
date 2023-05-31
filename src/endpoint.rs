use crate::http::HttpMethod;

pub enum EndpointAction {
    CreateTransaction,
    GetTransaction,
}

impl EndpointAction {
    pub fn builder() -> EndpointActionBuilder {
        EndpointActionBuilder::new()
    }

    pub fn endpoint(&self, api_key: String) -> Endpoint {
        match self {
            Self::CreateTransaction => {
                Endpoint::new("v1/json/orders".to_owned(), HttpMethod::POST, api_key)
            }

            Self::GetTransaction => {
                Endpoint::new("v1/json/orders".to_owned(), HttpMethod::GET, api_key)
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
        let api_key: String = self.api_key.expect("API key not specified");
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

    pub fn get_url(&self, is_debug: bool) -> String {
        if is_debug {
            format!("https://testapi.multisafepay.com/{}", self.url)
        } else {
            format!("https://api.multisafepay.com/{}", self.url)
        }
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }
}

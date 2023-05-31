use curl::Error;

use crate::endpoint::Endpoint;
use crate::utils::RestfulUrlBuilder;
use crate::{
    endpoint::{EndpointAction, EndpointActionBuilder},
    http::HttpRequestBuilder,
};

pub struct ClientBuilder {
    api_key: Option<String>,
    is_debug: Option<bool>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            api_key: None,
            is_debug: None,
        }
    }

    pub fn api_key(mut self, api_key: String, is_debug: bool) -> Self {
        self.api_key = Some(api_key);
        self.is_debug = Some(is_debug);

        self
    }

    pub fn get_order(&self, order_id: String) -> Result<String, curl::Error> {
        let retrieval_endpoint: Endpoint = EndpointActionBuilder::new()
            .api_key(self.api_key.as_ref().unwrap().to_owned())
            .build(EndpointAction::GetTransaction);

        let restful_url_info: Result<String, &str> = RestfulUrlBuilder::new()
            .url(
                retrieval_endpoint
                    .get_url(self.is_debug.unwrap())
                    .to_owned(),
            )
            .parameter(order_id.to_string())
            .format();

        if restful_url_info.is_ok() {
            let http_result: Result<String, Error> = HttpRequestBuilder::new()
            .api_key(&self.api_key.as_ref().unwrap().to_owned())
            .url(restful_url_info.as_ref().unwrap())
            .method(retrieval_endpoint.get_method().to_owned())
            .execute();

            match http_result {
                Ok(data) => Ok(data),
                Err(err) => Err(err),
            }
        } else {
             panic!("Failed to verify RESTful URL.")
        }
    }

    pub fn create_order(&self, body: String) -> Result<String, curl::Error> {
        let creation_endpoint: Endpoint = EndpointActionBuilder::new()
            .api_key(self.api_key.as_ref().unwrap().to_owned())
            .build(EndpointAction::CreateTransaction);

        let formatted_url: String = creation_endpoint.get_url(self.is_debug.unwrap());
        let http_result = HttpRequestBuilder::new()
            .api_key(&self.api_key.as_ref().unwrap().to_owned())
            .payload(body.as_ref())
            .url(formatted_url.as_ref())
            .method(creation_endpoint.get_method().to_owned())
            .execute();

        match http_result {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }
}

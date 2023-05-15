use crate::objects::endpoint::Endpoint as Endpoint;

#[test]
fn test_endpoint_formatting() {
    let endpoint_url: &str = "https://my-url.com";
    let endpoint_api_key: &str = "my-api-key";
    let my_endpoint: Endpoint = Endpoint::new(endpoint_url.to_string(), endpoint_api_key.to_string());

    assert_eq!(my_endpoint.format(), format!("{}?api-key={}", endpoint_url.clone(), endpoint_api_key.clone()));
}

#[test]
fn test_endpoint_verification() {
    let endpoint_url: &str = "https://my-url.com";
    let endpoint_api_key: &str = "my-api-key";
    let my_endpoint: Endpoint = Endpoint::new(endpoint_url.to_string(), endpoint_api_key.to_string());

    assert!(my_endpoint.verify())
}

use crate::http::{HttpMethod, HttpRequestBuilder};

#[test]
fn test_endpoint_url() {
    let http_result = HttpRequestBuilder::new()
        .api_key("my-api-key")
        .url("https://example.com")
        .method(HttpMethod::GET)
        .execute();

    assert!(http_result.is_ok());
}

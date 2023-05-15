use crate::objects::httpclient::HttpClient as HttpClient;

#[test]
fn http_get_functionality() {
    let endpoint_url = "https://dummyjson.com/test";
    let endpoint_method = "GET";

    let my_client = HttpClient::new(endpoint_url.to_owned(), endpoint_method.to_owned());
    let http_result = my_client.get();

    assert_ne!(http_result.unwrap(), "")
}

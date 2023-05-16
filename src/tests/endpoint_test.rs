use crate::endpoint::{EndpointActionBuilder, EndpointAction};

#[test]
fn test_endpoint_url() {
    let api_key = "testing-api_key";
    let my_endpoint = EndpointActionBuilder::new()
    .api_key(api_key.to_owned())
    .build(EndpointAction::CreateTransaction);

    assert_eq!(my_endpoint.get_url(true), "https://testapi.multisafepay.com/v1/json/orders")
}

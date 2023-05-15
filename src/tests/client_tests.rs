use crate::objects::client::Client as Client;

#[test]
fn test_client_verification() {
    let api_key = "my-api-key";
    let my_client = Client::new(api_key.to_owned());

    assert!(my_client.verify())
}

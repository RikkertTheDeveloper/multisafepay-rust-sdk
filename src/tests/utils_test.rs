use crate::utils::RestfulUrlBuilder;

#[test]
fn test_restful_url() {
    let restful_base_url = "https://example.com";
    let restful_url_parameter = "1";

    let new_restful_url = RestfulUrlBuilder::new()
        .url(restful_base_url.to_owned())
        .parameter(restful_url_parameter.to_owned())
        .format();

    assert!(new_restful_url.is_ok());
    assert_eq!(new_restful_url.unwrap(), "https://example.com/1");
}

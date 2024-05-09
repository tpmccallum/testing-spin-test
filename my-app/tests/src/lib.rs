use spin_test_sdk::{
    bindings::{fermyon::spin_test_virt, wasi, wasi::http},
    spin_test,
};

#[spin_test]
fn request_without_key() {
    send_get_request_without_key();
}

fn send_get_request_without_key() {
    // Perform the request
    let request = http::types::OutgoingRequest::new(http::types::Headers::new());
    request.set_path_with_query(Some("/")).unwrap();
    let response = spin_test_sdk::perform_request(request);

    // Assert response status and body is 404
    assert_eq!(response.status(), 404);
}

#[spin_test]
fn request_with_invalid_key() {
    send_get_request_with_invalid_key();
}

fn send_get_request_with_invalid_key() {
    // Perform the request
    let request = http::types::OutgoingRequest::new(http::types::Headers::new());
    request.set_path_with_query(Some("/not_here?id=123")).unwrap();
    let response = spin_test_sdk::perform_request(request);

    // Assert response status and body is 404
    assert_eq!(response.status(), 404);
}

#[spin_test]
fn request_with_invalid_key_id() {
    send_get_request_with_invalid_key_id();
}

fn send_get_request_with_invalid_key_id() {
    // Perform the request
    let request = http::types::OutgoingRequest::new(http::types::Headers::new());
    request.set_path_with_query(Some("/user_id?id=0")).unwrap();
    let response = spin_test_sdk::perform_request(request);

    // Assert response status and body is 404
    assert_eq!(response.status(), 404);
}

#[spin_test]
fn cache_hit() {
    let user_json = r#"{"id":123,"name":"Ryan"}"#;

    // Configure the app's 'cache' key-value store
    let key_value = spin_test_virt::key_value::Store::open("default");
    // Set a specific key with a specific value
    key_value.set("user_id", user_json.as_bytes());

    // Make the request against the Spin app
    let request = wasi::http::types::OutgoingRequest::new(wasi::http::types::Headers::new());
    request.set_path_with_query(Some("/user_id?id=123")).unwrap();
    let response = spin_test_sdk::perform_request(request);

    // Assert the response status and body
    assert_eq!(response.status(), 200);
    let body = response.body_as_string().unwrap();
    assert_eq!(body, user_json);

    // Assert the key-value store was queried
    assert_eq!(
        key_value.calls(),
        vec![spin_test_virt::key_value::Call::Get("123".to_owned())]
    );
}

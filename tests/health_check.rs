use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let addresss = spawn_app();

    let client = reqwest::Client::new();


    let response = client
        .get(&format!("{}/health_check", &addresss))
        .send()
        .await
        .expect("failed to execute request");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();
    let server = rust_zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
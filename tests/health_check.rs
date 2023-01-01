//! tests/heal_check.rs
use rust_web::startup::run;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute");

    //Assertions

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
fn spawn_app() {
    let address = format!("127.0.0.1:{}", 8080);
    let listener = TcpListener::bind(address).expect("Failed to create TCP LIstner");

    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

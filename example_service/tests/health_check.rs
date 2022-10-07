//! tests/health_check.rs

use actix_web::dev::Server;
use reqwest::{Client, Response};

const BASE_URL: &str = "http://127.0.0.1:8000";

#[actix_rt::test]
async fn health_check_works() {
    spawn_app();
    let client: Client = reqwest::Client::new();
	let endpoint: String = [BASE_URL, "health_check"].join("/");
    let response: Response = client.get(endpoint).send().await.expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
	let server: Server = example_service::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

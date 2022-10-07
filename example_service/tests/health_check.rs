//! tests/health_check.rs

use actix_web::dev::Server;
use reqwest::{Client, Response};
use std::{net::TcpListener};

const BASE_URL: &str = "http://127.0.0.1:8000";

#[actix_rt::test]
async fn health_check_works() {
    let address: String = spawn_app();
	let endpoint: &String = &format!("{}/health_check", &address);
    let client: Client = reqwest::Client::new();
    let response: Response = client.get(endpoint).send().await.expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
	let listener: TcpListener = TcpListener::bind(BASE_URL).expect("Failed to bind random port");
	let port: u16 = listener.local_addr().unwrap().port();
	let server: Server = example_service::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
	format!("http://127.0.0.1:{}", port)
}

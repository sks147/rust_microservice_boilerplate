//! tests/health_check.rs

const BASE_URL: &str = "http://127.0.0.1:8000";

#[actix_rt::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn our app.");
    let client = reqwest::Client::new();
	let endpoint = [BASE_URL, "health_check"].join("/");
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    example_service::run().await 
}

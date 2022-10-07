use example_service::run;
use std::net::TcpListener;

const BASE_URL: &str = "127.0.0.1:8000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let listener: TcpListener = TcpListener::bind(BASE_URL).expect("Failed to bind random port");
	let port: u16 = listener.local_addr().unwrap().port();
	println!("Listening on port: {}", port);
    run(listener)?.await
}

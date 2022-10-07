use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

const BASE_URL: &str = "127.0.0.1";
const BASE_PORT: &str = "8000";

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
	let service_endpoint: String = [BASE_URL, BASE_PORT].join(":");
    let server: Server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(service_endpoint)?
        .run();
	Ok(server)
}
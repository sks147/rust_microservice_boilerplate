use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}

// http://localhost:8080/1/world/index.html

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

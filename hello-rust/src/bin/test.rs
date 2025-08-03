use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn get_user() -> impl Responder {
    let user = User {
        id: 1,
        name: "Shiva Sajay".to_string(),
        email: "shiva@example.com".to_string(),
    };
    HttpResponse::Ok().json(user)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/user", web::get().to(get_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::mysql::MySqlPool;

use std::env;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/create")]
async fn create(pool: web::Data<MySqlPool>) -> impl Responder {
    sqlx::query!("CREATE TABLE Test(userid BIGINT);")
        .execute(pool)
        .await?;
    "Created"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

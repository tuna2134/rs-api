use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::mysql::MySqlPool;
use sqlx::Executor;

use std::env;

#[get("/create")]
async fn create(pool: web::Data<MySqlPool>) -> impl Responder {
    pool.execute("CREATE TABLE IF NOT EXISTS Hello(userid BIGINT);").await;
    "Created"
}

#[get("/insert")]
async fn insert(pool: web::Data<MySqlPool>) -> impl Responder {
    pool.execute("INSERT INTO Hello VALUES(928193829);").await;
    "Created"
}

#[get("/select")]
async fn select(pool: web::Data<MySqlPool>) -> impl Responder {
    let rows = pool.fetch_all("SELECT * FROM Hello;").await.unwrap();
    for row in rows {
        println!("{}", row.userid);
    }
    "Created"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .service(create)
            .service(insert)
            .service(select)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

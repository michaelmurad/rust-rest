use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use sea_orm::{Database, DatabaseConnection, EntityTrait };
use serde::{Deserialize, Serialize};
mod entities;

#[derive(Debug, Clone)]
struct AppState {
   db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url= env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);
    let db: DatabaseConnection = Database::connect(&db_url).await.unwrap();
    let state = AppState { db };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/users")
                    .route("", web::get().to(get_all_users))
            )
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(&server_url)?
        .run()
        .await
}

// #[get("/")]
async fn get_all_users(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let response = entities::prelude::Users::find().all(&data.db).await.unwrap();
    HttpResponse::Ok().json(response)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("student_data")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
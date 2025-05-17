use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/rust")
                    .route("/users", web::get().to(get_users))
                    .route("/users", web::post().to(create_user))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> HttpResponse {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();
    HttpResponse::Ok().json(users)
}

#[post("/users")]
async fn create_user(pool: web::Data<PgPool>, user: web::Json<User>) -> HttpResponse {
    sqlx::query!("INSERT INTO users (name, email) VALUES ($1, $2)",
        user.name, user.email)
        .execute(pool.get_ref())
        .await
        .unwrap_or_default();
    HttpResponse::Ok().body("User created")
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}
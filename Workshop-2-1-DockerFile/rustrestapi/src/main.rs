use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use redis::AsyncCommands;
use std::env;
use std::sync::Arc;

#[derive(Deserialize)]
struct User {
    id: i32,
    name: String,
    description: String,
}

struct AppState {
    db_pool: Pool<MySql>,
    redis_client: redis::Client,
}

async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome Page from Container Rust Lab")
}

async fn get_user(
    uid: web::Path<i32>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let mut redis_conn = data.redis_client.get_async_connection().await.unwrap();
    if let Ok(name): Result<String, _> = redis_conn.get(uid.to_string()).await {
        return HttpResponse::Ok().body(format!("########### Return from Redis ###########\n{name}"));
    }

    let row = sqlx::query!("SELECT name FROM member WHERE id = ?", *uid)
        .fetch_optional(&data.db_pool)
        .await
        .unwrap();
    
    if let Some(record) = row {
        let name = record.name;
        let _: () = redis_conn.set_ex(uid.to_string(), &name, 30).await.unwrap();
        return HttpResponse::Ok().body(format!("########### Return from Redis ###########\n{name}"));
    }

    HttpResponse::NotFound().body("########### Record not found ###########")
}

async fn add_user(
    user: web::Json<User>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let exists = sqlx::query!("SELECT id FROM member WHERE id = ?", user.id)
        .fetch_optional(&data.db_pool)
        .await
        .unwrap();

    if exists.is_some() {
        return HttpResponse::Conflict().body("########### Record Duplicate Already Exist ###########");
    }

    sqlx::query!("INSERT INTO member (id, name, description) VALUES (?, ?, ?)", user.id, &user.name, &user.description)
        .execute(&data.db_pool)
        .await
        .unwrap();
    
    HttpResponse::Ok().body("########### Record Added ###########")
}

async fn del_user(
    uid: web::Path<i32>,
    data: web::Data<Arc<AppState>>,
) -> impl Responder {
    let mut redis_conn = data.redis_client.get_async_connection().await.unwrap();
    if redis_conn.del::<_, ()>(uid.to_string()).await.is_ok() {
        println!("########### Deleted from Cache ###########");
    }

    let result = sqlx::query!("DELETE FROM member WHERE id = ?", *uid)
        .execute(&data.db_pool)
        .await
        .unwrap();
    
    if result.rows_affected() > 0 {
        return HttpResponse::Ok().body("########### Deleted from Database ###########");
    }

    HttpResponse::NotFound().body("########### Delete Record not found ###########")
}

async fn initialize(data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut redis_conn = data.redis_client.get_async_connection().await.unwrap();
    let _: () = redis_conn.flushdb().await.unwrap();

    let db_url = env::var("MARIADB_URL").unwrap_or("mysql://memberadmin:taleofdestiny@MainDB:3306/".to_string());
    let db_pool = sqlx::MySqlPool::connect(&db_url).await.unwrap();
    
    sqlx::query!("DROP DATABASE IF EXISTS accoutable").execute(&db_pool).await.unwrap();
    sqlx::query!("CREATE DATABASE accoutable").execute(&db_pool).await.unwrap();
    sqlx::query!("USE accoutable").execute(&db_pool).await.unwrap();
    sqlx::query!("CREATE TABLE member (id INT PRIMARY KEY, name CHAR(100), description CHAR(250))")
        .execute(&db_pool)
        .await
        .unwrap();

    HttpResponse::Ok().body("########### Initial Database Done ###########")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("MARIADB_URL").unwrap_or("mysql://memberadmin:taleofdestiny@MainDB:3306/accoutable".to_string());
    let redis_url = env::var("REDIS_URL").unwrap_or("redis://CacheDB:6379".to_string());

    let db_pool = sqlx::MySqlPool::connect(&db_url).await.unwrap();
    let redis_client = redis::Client::open(redis_url).unwrap();

    let state = Arc::new(AppState { db_pool, redis_client });
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(welcome))
            .route("/users/{uid}", web::get().to(get_user))
            .route("/users/adduser", web::put().to(add_user))
            .route("/user/deluser/{uid}", web::delete().to(del_user))
            .route("/initial", web::post().to(initialize))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}

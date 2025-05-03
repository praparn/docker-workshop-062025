use actix_web::{App, HttpServer, Responder, HttpResponse, get};

// Define the GET / handler
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Too many hello, Just hi from rust !!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the server on port 5000
    HttpServer::new(|| {
        App::new()
            .service(index) // Register the route
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
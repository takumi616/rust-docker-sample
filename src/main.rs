use actix_web::{App, HttpServer};

mod task;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(task::list)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

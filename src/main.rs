use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting backend server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new().service(
            Files::new("/", "./webroot")
                .index_file("index.html")
                .use_last_modified(true),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

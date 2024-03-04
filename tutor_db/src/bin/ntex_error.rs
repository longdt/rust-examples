use ntex::web;
use ntex::web::{App, Error, HttpResponse, HttpServer};
use std::fs::File;
use std::io;

#[web::get("/hello")]
async fn hello() -> Result<HttpResponse, Error> {
    let _ = File::open("fictional_file.txt")?;
    Ok(HttpResponse::Ok().body("File read successfully"))
}

#[ntex::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

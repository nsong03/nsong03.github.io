use actix_web::{web, App, HttpResponse, HttpServer};
use actix_files as fs;

async fn index() -> HttpResponse {
    let html = fs::NamedFile::open("C:/cleen/nsong/nsong03.github.io/index.html")?; // Replace with the actual path to your HTML file
    Ok(html.into_response())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(fs::Files::new("/static", "C:/cleen/nsong/nsong03.github.io").show_files_listing())
    })
    .bind("localhost:80")? // Replace with the desired IP address and port
    .run()
    .await
}




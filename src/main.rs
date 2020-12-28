use actix::Actor;
use actix_files::Files;
use actix_web::{App, HttpServer};
use std::sync::{Arc, Mutex};
use ztransfer::{structures::Broadcast, transfer, websocket};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let broadcaster = Arc::new(Mutex::new(Broadcast {}.start()));
    std::fs::create_dir_all("./tmp").unwrap();
    HttpServer::new(move || {
        // The order in which you add services/routes DOES matter.
        // Precedence is first come first serve, so the most general (static serve) should come last.
        App::new()
            .data(broadcaster.clone())
            // Host the multipart uploader endpoint.
            .service(transfer::upload)
            // Host the websocket endpoint.
            .service(websocket::endpoint)
            // Host static files from "tmp" and host them as "downloads/...".
            .service(Files::new("/download", "tmp"))
            // Host static files from "public" and host "index.html" as the index.
            .service(Files::new("/", "public").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

mod structures;
mod transfer;
mod util;
mod websocket;

use actix::Actor;
use actix_files::Files;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::fs;
use std::sync::{Arc, Mutex};
use structures::Broadcast;
use util::port;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    #[cfg(debug_assertions)]
    let host = "127.0.0.1";
    #[cfg(not(debug_assertions))]
    let host = "0.0.0.0";

    let port = port();
    let address = format!("{}:{}", host, port);

    println!("Now hosting {} on {}.", env!("CARGO_PKG_NAME"), address);

    // Then initialize the server
    let broadcaster = Arc::new(Mutex::new(Broadcast {}.start()));

    // Make sure that the tmp directory exists
    fs::create_dir_all("./tmp").unwrap();

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
    .bind(address)?
    .run()
    .await
}

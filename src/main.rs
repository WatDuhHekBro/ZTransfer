use actix::Actor;
use actix_files::Files;
use actix_web::{App, HttpServer};
use serde_json::from_str;
use std::fs;
use std::sync::{Arc, Mutex};
use ztransfer::{
    structures::{Broadcast, ServerSettings},
    transfer, websocket,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings;

    // Read settings.json
    if let Ok(settings_file) = fs::read_to_string("settings.json") {
        settings = from_str::<ServerSettings>(&settings_file)?;
    } else {
        settings = ServerSettings {
            host: String::from("127.0.0.1"),
            port: 8080,
        };
    }

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
    .bind(format!("{}:{}", settings.host, settings.port))?
    .run()
    .await
}

use actix::{Actor, Context, Handler};
use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_files::Files;
use actix_web::{web, App, HttpServer};
mod transfer;
mod websocket;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct MyLittlePogchamp {
    //pub connections: Vec<Addr<websocket::Connection>>
//pub context: Context<Self>
}

impl MyLittlePogchamp {
    pub fn broadcast(&mut self, message: &str) {
        //self.issue_system_sync(websocket::BroadcastMessage);
    }
}

impl Actor for MyLittlePogchamp {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {}
}

// Why it asks me to implement a handler when I'm just sending data is beyond me
impl Handler<websocket::BroadcastMessage> for MyLittlePogchamp {
    type Result = ();

    fn handle(&mut self, message: websocket::BroadcastMessage, ctx: &mut Self::Context) {
        println!("Handler What - {}", message.0);
        self.issue_system_sync(message, ctx);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let broadcaster = Arc::new(Mutex::new(MyLittlePogchamp {}.start()));
    //let a = data.send(websocket::BroadcastMessage(String::from(""))).await;
    //let data = Arc::new(Mutex::new(MyLittlePogchamp {}));
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
            .service(websocket::test)
            // Host static files from "public" and host "index.html" as the index.
            .service(Files::new("/", "public").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

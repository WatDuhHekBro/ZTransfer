use crate::MyLittlePogchamp;
use actix::{Actor, Addr, Handler, Message, StreamHandler};
use actix_broker::BrokerSubscribe;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::{from_str /*, json, to_string*/, Value};
use std::sync::{Arc, Mutex};

// HTTP actor
pub struct Connection {
    //connections: Vec<Addr<Connection>>
}

impl Actor for Connection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<BroadcastMessage>(ctx);
    }
}

// Handle input
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Connection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let data: Value = from_str(&text).unwrap_or(Value::Null);
                println!("{:?}", data);
                ctx.text(text)
            }
            _ => (),
        }
    }
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

impl Handler<BroadcastMessage> for Connection {
    type Result = ();

    fn handle(&mut self, message: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(message.0);
    }
}

#[get("websocket")]
pub async fn endpoint(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Connection {}, &req, stream)
}

#[get("test")]
pub async fn test(data: web::Data<Arc<Mutex<Addr<MyLittlePogchamp>>>>) -> HttpResponse {
    let broadcaster = &*data.lock().unwrap();
    let result = broadcaster
        .send(BroadcastMessage(String::from("testuwu")))
        .await;

    match result {
        Ok(_) => println!("yay"),
        Err(_) => println!("rip"),
    }

    HttpResponse::Ok().body("test")
}

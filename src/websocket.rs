use crate::structures::{Connection, SendDeleteRequest, SEND_DELETE_REQUEST};
use actix::StreamHandler;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::from_str;

// Handle input sent from clients
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Connection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // If the client-to-server message is a valid text message
        if let Ok(ws::Message::Text(text)) = msg {
            // Ignore error cases because they're just malformed requests
            if let Ok(data) = from_str::<SendDeleteRequest>(&text) {
                if data.action == SEND_DELETE_REQUEST {
                    // Do some magic here and stuff
                    ctx.text(text) // For now, just echo the text back to the client
                }
            }
        }
    }
}

#[get("websocket")]
pub async fn endpoint(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Connection {}, &req, stream)
}

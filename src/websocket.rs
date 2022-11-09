use crate::structures::{Connection, WsMessage};
use actix::StreamHandler;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

// Handle input sent from clients
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Connection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // If the client-to-server message is a valid text message
        if let Ok(ws::Message::Text(text)) = msg {
            // Ignore error cases because they're just malformed requests
            let message = serde_json::from_str::<WsMessage>(&text);

            match message {
                Ok(message) => {
                    match message {
                        WsMessage::SendDeleteRequest { filename: _ } => {
                            // Do some magic here and stuff
                            ctx.text(text) // For now, just echo the text back to the client
                        }
                        _ => {}
                    }
                }
                Err(_error) => {
                    // Relay the error back to the client
                }
            }
        }
    }
}

#[get("websocket")]
pub async fn endpoint(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Connection {}, &req, stream)
}

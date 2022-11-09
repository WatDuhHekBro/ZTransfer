// How this system works:
// There are two actors in place: Broadcasts and Connections
// There is one Broadcast object instantiated and stored and is stored in the application's data, accessible by any endpoint
// Connections are created for each WebSocket connection only to receive input and relay broadcast messages out
// Connections subscribe to BroadcastMessages which are sent by the Broadcast object and all relay the message over
use actix::{Actor, Context, Handler, Message};
use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

/////////////////////
// JSON Structures //
/////////////////////

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "action")]
pub enum WsMessage {
    AddFile {
        filename: String,
        size: u64,
    },
    RemoveFile {
        filename: String,
    },
    UploadProgress {
        filename: String,
        progress: u64,
        total: u64,
    },
    SendDeleteRequest {
        filename: String,
    },
}

////////////////////////
// WebSocket Handlers //
////////////////////////

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

#[derive(Debug)]
pub struct Broadcast;

impl Actor for Broadcast {
    type Context = Context<Self>;
}

// Global message relayer
impl Handler<BroadcastMessage> for Broadcast {
    type Result = ();

    fn handle(&mut self, message: BroadcastMessage, ctx: &mut Self::Context) {
        self.issue_system_sync(message, ctx);
    }
}

// Setup handler for each WebSocket connection, listen in to
pub struct Connection;

// Start listening for broadcast messages and relay it to the handler
impl Actor for Connection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<BroadcastMessage>(ctx);
    }
}

// Relay global messages to all clients
impl Handler<BroadcastMessage> for Connection {
    type Result = ();

    fn handle(&mut self, message: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(message.0);
    }
}

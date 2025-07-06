use actix::{Actor, Addr, AsyncContext, Handler, Message, Running, StreamHandler};
use actix_web_actors::ws;
use crate::session_manager::WsSessionManager;

// ------------------------------------------
// WebSocket messages
// ------------------------------------------

// This message is used to notify the WsSessionManager when a new client connects to the WebSocket.
// addr - address of the WebSocket actor representing the new connection.
#[derive(Message)]
#[rtype(result ="()")]
pub struct Connect {
  pub(crate) addr: Addr<WebSocket>
}

// This message is used to notify the WsSessionManager when a client disconnects from the WebSocket.
// addr - address of the WebSocket actor representing the disconnected client.
#[derive(Message)]
#[rtype(result ="()")]
pub struct Disconnect {
  pub(crate) addr: Addr<WebSocket>
}

// A message to be broadcast to all connected WebSocket clients.
// message - the content of the message to be sent.
// sender - the address of the WebSocket actor that sent the message
#[derive(Message, Clone)] // Clone is derived to allow easy duplication of this message.
#[rtype(result = "()")]
pub struct BroadcastMessage {
  pub(crate) message: String,
  pub(crate) sender: Addr<WebSocket>,
}

#[derive(Message)]
#[rtype(result ="()")]
pub struct DefaultMessage {
  pub text: String,
}

impl Handler<DefaultMessage> for WebSocket {
  type Result = ();

  fn handle(&mut self, msg: DefaultMessage, ctx: &mut Self::Context) {
    ctx.text(msg.text);    
  }
}

impl Handler<BroadcastMessage> for WebSocket {
  type Result = ();

  fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
    ctx.text(msg.message);    
  }
}

pub struct WebSocket {
  pub(crate) manager: Addr<WsSessionManager>,
}

// Implement Actor trait, so WS can be used as an Actix actor.
impl Actor for WebSocket {
  type Context = ws::WebsocketContext<Self>; // Define the type of context used by WS actor.

  fn started(&mut self, ctx: &mut Self::Context) {
    self.manager.do_send(Connect {
      addr: ctx.address(), // send the address of this WebSocket actor to the manager
    });
  }

  fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
    self.manager.do_send(Disconnect {
      addr: ctx.address(), // send the address of this WebSocket actor to the manager
    });

    Running::Stop // Indicate to the actor that the running should stop
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    // check if the message is valid text message
    if let Ok(ws::Message::Text(text)) = msg {
      self.manager.do_send(BroadcastMessage {
        message: text.to_string(),
        sender: ctx.address(),
      });
    }
  }
}

use std::collections::HashSet;
use actix::{Actor, Addr, Context, Handler};
use crate::actor::{Connect, Disconnect, BroadcastMessage, WebSocket};

pub struct WsSessionManager {
  // A set of active WebSocket sessions
  sessions:HashSet<Addr<WebSocket>>,

  // The last text message sent to the WebSocket
  pub(crate) last_text: String,
}

impl WsSessionManager {
  pub(crate) fn new() -> Self {
    let defult_srt = "{\"ops\": [{\"insert\": \"Write \"},{\"attributes\": {\"underline\": true},\"insert\": \"here\"},{\"insert\": \" some \"},{\"attributes\": {\"bold\": true},\"insert\": \"text\"},{\"insert\": \"!\"}]}";

    Self {
      sessions: HashSet::new(),
      last_text: String::from(defult_srt)
    }
  }
}

impl Actor for WsSessionManager {
  type Context = Context<Self>; // start context type for Actix actor  
}

// handles the connect message that signals a new WebSocket connection
impl Handler<Connect> for WsSessionManager {
  type Result = (); // no specific result is returned after handling the message

  fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
    let addr = msg.addr;
    println!("New WebSocket connection: {:?}", addr);
    self.sessions.insert(addr.clone());
  }
}

impl Handler<Disconnect> for WsSessionManager {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
    let addr = msg.addr;
    println!("WebSocket disconnected: {:?}", addr);
    self.sessions.remove(&addr);
  }
}

impl Handler<BroadcastMessage> for WsSessionManager {
  type Result = ();

  fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
    self.last_text = msg.message.clone();

    for addr in &self.sessions {
      if *addr != msg.sender {
        let msg_clone = msg.clone();
        addr.do_send(msg_clone);
      }
    }
  }
}

//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use actix::prelude::*;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct ChatServer {
    sessions: HashMap<String, Recipient<Message>>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
        }
    }
}

impl ChatServer {
    /// Send message to all users in the room
    fn send_message(&self, msg_type: String, receiver: String, message: String) {
        match msg_type.as_str() {
            "heartbeat" => {
                println!("heartbeat");
            }
            _ => {
                if let Some(addr) = self.sessions.get(&receiver) {
                    addr.do_send(Message(message));
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub uuid: String,
    pub addr: Recipient<Message>,
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let uuid = msg.uuid;
        self.sessions.insert(uuid, msg.addr);
        0
    }
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub uuid: String,
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        // remove address
        self.sessions.remove(&msg.uuid);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMessage {
    pub msg_type: String,
    pub receiver: String,
    pub sender: String,
    pub msg: String,
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        let msg_type = msg.msg_type.clone();
        match msg_type.as_str() {
            "heartbeat" => {
                println!("heartbeat.")
            }
            _ => {
                let receiver = msg.receiver.clone();
                let msg_type = msg.msg_type.clone();
                let json = serde_json::to_string(&msg).unwrap();
                self.send_message(msg_type, receiver, json);
            }
        }
    }
}

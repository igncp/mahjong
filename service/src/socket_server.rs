use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use service_contracts::SocketMessage;
use std::collections::{HashMap, HashSet};

#[derive(Message)]
#[rtype(result = "()")]
pub struct SocketMessageStr(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub room: String,
    pub addr: Recipient<SocketMessageStr>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub msg: SocketMessage,
    pub room: String,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Debug)]
pub struct MahjongWebsocketServer {
    sessions: HashMap<usize, Recipient<SocketMessageStr>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
}

impl MahjongWebsocketServer {
    pub fn new() -> Self {
        let rooms = HashMap::new();

        Self {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
        }
    }
}

impl MahjongWebsocketServer {
    fn send_message(&self, room: &str, message: &SocketMessage, skip_id: usize) {
        let message = serde_json::to_string(&message).unwrap();
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(SocketMessageStr(message.to_owned()));
                    }
                }
            }
        }
    }
}

impl Actor for MahjongWebsocketServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MahjongWebsocketServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let sent_msg = SocketMessage::PlayerJoined;
        self.send_message(&msg.room, &sent_msg, 0);
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        self.rooms
            .entry(msg.room)
            .or_insert_with(HashSet::new)
            .insert(id);

        id
    }
}

impl Handler<Disconnect> for MahjongWebsocketServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let mut rooms: Vec<String> = Vec::new();
        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }

        for room in rooms {
            let sent_msg = SocketMessage::PlayerLeft;
            self.send_message(&room, &sent_msg, 0);
        }
    }
}

impl Handler<ClientMessage> for MahjongWebsocketServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room, &msg.msg, msg.id);
    }
}

impl Handler<ListRooms> for MahjongWebsocketServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

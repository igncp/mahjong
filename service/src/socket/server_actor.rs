use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use rustc_hash::{FxHashMap, FxHashSet};
use service_contracts::SocketMessage;

use super::{
    session::{RoomId, SessionId},
    SocketClientMessage, SocketMessageConnect, SocketMessageDisconnect, SocketMessageListRooms,
    SocketMessageListSessions, SocketMessageStr,
};

#[derive(Debug)]
pub struct MahjongWebsocketServer {
    sessions: FxHashMap<SessionId, Recipient<SocketMessageStr>>,
    rooms: FxHashMap<RoomId, FxHashSet<SessionId>>,
    rng: ThreadRng,
}

impl MahjongWebsocketServer {
    pub fn new() -> Self {
        let rooms = FxHashMap::default();

        Self {
            sessions: FxHashMap::default(),
            rooms,
            rng: rand::thread_rng(),
        }
    }

    fn send_message(&self, room: &str, message: &SocketMessage, skip_id: SessionId) {
        let message = serde_json::to_string(&message).unwrap();
        if let Some(sessions) = self.rooms.get(room) {
            for session_id in sessions {
                if *session_id != skip_id {
                    if let Some(addr) = self.sessions.get(session_id) {
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

impl Handler<SocketMessageConnect> for MahjongWebsocketServer {
    type Result = SessionId;

    fn handle(&mut self, msg: SocketMessageConnect, _: &mut Context<Self>) -> Self::Result {
        let sent_msg = SocketMessage::PlayerJoined;
        self.send_message(&msg.room, &sent_msg, 0);
        let session_id = self.rng.gen::<SessionId>();
        self.sessions.insert(session_id, msg.addr);

        self.rooms.entry(msg.room).or_default().insert(session_id);

        session_id
    }
}

impl Handler<SocketMessageDisconnect> for MahjongWebsocketServer {
    type Result = ();

    fn handle(&mut self, msg: SocketMessageDisconnect, _: &mut Context<Self>) {
        let mut rooms: Vec<RoomId> = Vec::new();
        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }

        for room in rooms.clone() {
            let sent_msg = SocketMessage::PlayerLeft;
            self.send_message(&room, &sent_msg, 0);
        }

        for room in rooms {
            if self.rooms.get(&room).unwrap().is_empty() {
                self.rooms.remove(&room);
            }
        }
    }
}

impl Handler<SocketClientMessage> for MahjongWebsocketServer {
    type Result = ();

    fn handle(&mut self, msg: SocketClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room, &msg.msg, msg.id);
    }
}

impl Handler<SocketMessageListRooms> for MahjongWebsocketServer {
    type Result = MessageResult<SocketMessageListRooms>;

    fn handle(&mut self, _: SocketMessageListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

impl Handler<SocketMessageListSessions> for MahjongWebsocketServer {
    type Result = MessageResult<SocketMessageListSessions>;

    fn handle(&mut self, _: SocketMessageListSessions, _: &mut Context<Self>) -> Self::Result {
        let mut sessions = FxHashMap::default();

        for key in self.rooms.keys() {
            let sessions_num = self.rooms.get(key).unwrap().len();
            sessions.insert(key.to_owned(), sessions_num);
        }

        MessageResult(sessions)
    }
}

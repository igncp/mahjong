use super::session::{RoomId, SessionId};
use actix::prelude::*;
use rustc_hash::FxHashMap;
use service_contracts::SocketMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SocketMessageStr(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct SocketMessageConnect {
    pub room: RoomId,
    pub addr: Recipient<SocketMessageStr>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SocketMessageDisconnect {
    pub id: SessionId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SocketClientMessage {
    pub id: SessionId,
    pub msg: SocketMessage,
    pub room: RoomId,
}

pub struct SocketMessageListRooms;

impl actix::Message for SocketMessageListRooms {
    type Result = Vec<RoomId>;
}

pub struct SocketMessageListSessions;

impl actix::Message for SocketMessageListSessions {
    type Result = FxHashMap<RoomId, usize>;
}

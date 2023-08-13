use super::{
    MahjongWebsocketServer, SocketMessageConnect, SocketMessageDisconnect, SocketMessageListRooms,
    SocketMessageStr,
};
use actix::prelude::*;
use actix_web_actors::ws;
use mahjong_core::{GameId, PlayerId};
use service_contracts::SocketMessage;
use std::time::{Duration, Instant};
use tracing::debug;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct MahjongWebsocketSession {
    pub addr: Addr<MahjongWebsocketServer>,
    pub hb: Instant,
    pub id: usize,
    pub name: Option<String>,
    pub room: String,
}

impl MahjongWebsocketSession {
    pub fn get_room_id(game_id: &GameId, player_id: Option<&PlayerId>) -> String {
        if player_id.is_none() {
            return game_id.to_string();
        }

        format!("{}__{}", game_id, player_id.unwrap())
    }
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, new_ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(SocketMessageDisconnect { id: act.id });
                new_ctx.stop();
                return;
            }

            new_ctx.ping(b"");
        });
    }
}

impl Actor for MahjongWebsocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        let room = self.room.clone();

        self.addr
            .send(SocketMessageConnect {
                room,
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, new_ctx| {
                match res {
                    Ok(res) => {
                        act.id = res;
                        debug!("{} joined room {}", act.id, act.room);
                    }
                    _ => new_ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        debug!("{} disconnected from {}", self.id, self.room);
        self.addr.do_send(SocketMessageDisconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<SocketMessageStr> for MahjongWebsocketSession {
    type Result = ();

    fn handle(&mut self, msg: SocketMessageStr, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MahjongWebsocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let message = serde_json::from_str::<SocketMessage>(&text);
                if message.is_err() {
                    return;
                }

                if let Ok(SocketMessage::ListRooms) = message {
                    self.addr
                        .send(SocketMessageListRooms)
                        .into_actor(self)
                        .then(|res, _, new_ctx| {
                            if let Ok(rooms) = res {
                                for room in rooms {
                                    let room =
                                        serde_json::to_string(&SocketMessage::Name(room)).unwrap();

                                    new_ctx.text(room);
                                }
                            }
                            fut::ready(())
                        })
                        .wait(ctx)
                }
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            _ => {}
        }
    }
}

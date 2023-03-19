use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use service_contracts::SocketMessage;

use crate::socket_server::{
    Connect, Disconnect, ListRooms, MahjongWebsocketServer, SocketMessageStr,
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct MahjongWebsocketSession {
    pub id: usize,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>,
    pub addr: Addr<MahjongWebsocketServer>,
}

impl MahjongWebsocketSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
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
            .send(Connect {
                room,
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {
                        act.id = res;
                        println!("{} joined room {}", act.id, act.room);
                    }
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        println!("{} disconnected from {}", self.id, self.room);
        self.addr.do_send(Disconnect { id: self.id });
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

                match message.unwrap() {
                    SocketMessage::ListRooms => self
                        .addr
                        .send(ListRooms)
                        .into_actor(self)
                        .then(|res, _, ctx| {
                            if let Ok(rooms) = res {
                                for room in rooms {
                                    let room =
                                        serde_json::to_string(&SocketMessage::Name(room)).unwrap();

                                    ctx.text(room);
                                }
                            }
                            fut::ready(())
                        })
                        .wait(ctx),
                    SocketMessage::GameUpdate(_game) => {}
                    _ => {}
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
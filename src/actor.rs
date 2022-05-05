use actix::prelude::*;
use actix_rt::task::JoinHandle;
use std::{net::UdpSocket, thread};

pub type BufferChunk = [u8; 1024];

pub struct TH12TunnelActor {
    pub receive_port: u16,
    dest_port: u16,
    state: State,
}

pub struct RemoteTunnelActor {
    pub target_port: u16,
}

#[derive(Message)]
#[rtype(result = "u16")]
pub struct InitialTh123PortEvent(pub u16);

#[derive(Message)]
#[rtype(result = "BufferChunk")]
pub struct PacketRelayEvent(pub BufferChunk);

impl TH12TunnelActor {
    pub fn create(receive_port: u16) -> Self {
        TH12TunnelActor {
            receive_port,
            dest_port: 0,
            state: State::Initialized,
        }
    }
}

impl Actor for TH12TunnelActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {}

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        self.state = State::Initialized;
    }
}

impl Handler<InitialTh123PortEvent> for TH12TunnelActor {
    type Result = u16;

    fn handle(&mut self, msg: InitialTh123PortEvent, ctx: &mut Context<Self>) -> Self::Result {
        println!("{}", msg.0);
        msg.0
    }
}

pub enum State {
    Initialized,
    PortDetected,
    Established,
    Tunneling,
    SessionEnded,
}

enum Event {
    InitialTh123Port(u16),
    InitialTunnel,
    StartForward,
    EndForward,
}

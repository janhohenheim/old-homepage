extern crate chat;
extern crate mio;

use mio::*;

struct WebSocketServer;

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();
}

fn main() {
    loop {
        let poll = Poll::new()?;
        //let mut events = Event::new(Ready::all(), Token)
        //let result = Poll::poll
    }
}
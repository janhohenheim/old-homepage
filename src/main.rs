extern crate chat;
extern crate mio;

use mio::*;

struct WebSocketServer;

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();
}

fn main() {

}
mod message;
mod server;

use self::server::WsServer;

fn main() {
    let ws = ws::WebSocket::new(WsServer::default()).expect("cannot create websocket");
    ws.listen("127.0.0.1:8000").unwrap();
}

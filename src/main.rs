mod connection;

use self::connection::ConnectionFactory;

fn main() {
    let factory = ConnectionFactory::default();
    let ws = ws::WebSocket::new(factory).expect("cannot create websocket");
    ws.listen("127.0.0.1:8000").unwrap();
}

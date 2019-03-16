fn main() -> ws::Result<()> {
    ws::listen("127.0.0.1:8000", |out| move |msg| out.send(msg))
}

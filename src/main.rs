use rustirc_server::handle_listener;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener_local = TcpListener::bind("127.0.0.1:6667").await?;
    println!("IRC PoC 서버가 127.0.0.1:6667에서 대기 중입니다.");
    handle_listener(listener_local).await?;
    Ok(())
}

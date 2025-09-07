mod parser;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn handle_listener(listener: TcpListener) -> std::io::Result<()> {
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("client \"{}\" connected", addr);
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            socket
                .write_all("hello world\n".to_string().as_bytes())
                .await
                .unwrap();
            match socket.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    let recv_msg = String::from_utf8_lossy(&buf[..n]);
                    println!("수신: {}", recv_msg);
                    let _ = socket.write_all(b":server NOTICE :Hello IRC!\r\n").await;
                }
                _ => println!("클라이언트 데이터 없음 또는 에러"),
            }
        });
        println!("test1");
    }
}

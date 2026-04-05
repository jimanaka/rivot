use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn tcp_connect(host: String, port: u16) -> io::Result<()> {
    println!("Connecting to {host}:{port}");

    let mut socket = TcpStream::connect(format!("{host}:{port}")).await?;

    socket.write_all(b"Hello, World!").await?;

    Ok(())
}

pub async fn tcp_listen(host: String, port: u16) {
    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            println!("Connected!");
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}

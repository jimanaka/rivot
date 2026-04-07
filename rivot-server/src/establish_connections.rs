use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn tcp_connect(host: String, port: u16) {
    let mut stdout = io::stdout();
    stdout
        .write(format!("Connecting to {host}:{port}\n").as_bytes())
        .await;
    stdout.flush().await;

    let mut socket = TcpStream::connect(format!("{host}:{port}")).await.unwrap();

    socket.write_all(b"Hello, World!").await.unwrap();
}

pub async fn tcp_listen(host: String, port: u16) {
    let mut stdout = io::stdout();

    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    stdout.write(b"\nListening...\n").await.unwrap();
    stdout.flush().await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let mut stdout = io::stdout();
            stdout.write(b"Connected!\n").await.unwrap();
            stdout.flush().await.unwrap();
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

use dashmap::DashMap;
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

pub enum TunnelType {
    Forward,
    Reverse,
}

impl Display for TunnelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TunnelType::Forward => write!(f, "Forward"),
            TunnelType::Reverse => write!(f, "Reverse"),
        }
    }
}

pub enum ConnectionCommand {
    Kill,
}

pub struct Connection {
    pub name: String,
    pub kind: TunnelType,
    pub tx: mpsc::Sender<ConnectionCommand>,
}

pub type ConnectionMap = Arc<DashMap<String, Connection>>;

pub async fn tcp_connect(host: String, port: u16, connections_map: ConnectionMap) {
    let mut stdout = io::stdout();

    stdout.write(b"\r\nConnecting...\r\n").await.unwrap();

    stdout.flush().await.unwrap();

    let mut socket = TcpStream::connect(format!("{host}:{port}")).await.unwrap();

    let index = connections_map.len();
    let (tx, rx) = mpsc::channel(32);

    let connection = Connection {
        name: index.to_string(),
        kind: TunnelType::Forward,
        tx: tx,
    };

    connections_map.insert(index.to_string(), connection);

    socket.write_all(b"Hello, World!").await.unwrap();
}

pub async fn tcp_listen(host: String, port: u16) {
    let mut stdout = io::stdout();

    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    stdout.write(b"\r\nListening...\r\n").await.unwrap();
    stdout.flush().await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let mut stdout = io::stdout();
            stdout.write(b"\r\nConnected!\r\n").await.unwrap();
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

use clap::{Parser, Subcommand};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser)]
#[command(name = "rpivot-server", about = "Rusty tunneling tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Forward {
        #[arg(short = 'H', long)]
        host: String,

        #[arg(short = 'P', long, default_value_t = 4444)]
        port: u16,
    },
    Reverse {
        #[arg(short = 'H', long)]
        host: String,

        #[arg(short = 'P', long, default_value_t = 4444)]
        port: u16,
    },
}

async fn tcp_connect(host: String, port: u16) -> io::Result<()> {
    println!("Connecting to {host}:{port}");

    let mut socket = TcpStream::connect(format!("{host}:{port}")).await?;

    socket.write_all(b"Hello, World!").await?;

    Ok(())
}

async fn tcp_listen(host: String, port: u16) {
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

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Forward { host, port } => tcp_connect(host, port).await?,
        Commands::Reverse { host, port } => tcp_listen(host, port).await,
    }

    Ok(())
}

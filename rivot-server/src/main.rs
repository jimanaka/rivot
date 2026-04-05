mod establish_connections;
use clap::{Parser, Subcommand};
use establish_connections::{tcp_connect, tcp_listen};
use tokio::io;

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

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Forward { host, port } => tcp_connect(host, port).await?,
        Commands::Reverse { host, port } => tcp_listen(host, port).await,
    }

    Ok(())
}

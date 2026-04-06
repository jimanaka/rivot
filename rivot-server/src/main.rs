mod cli_util;
mod establish_connections;
use clap::{Parser, Subcommand};
use cli_util::run_cli;
use establish_connections::{tcp_connect, tcp_listen};
use tokio::io;
use tokio::sync::mpsc;
use tokio::task;

use crate::cli_util::CliCommand;

// #[derive(Parser)]
// #[command(name = "rpivot-server", about = "Rusty tunneling tool")]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     Forward {
//         #[arg(short = 'H', long)]
//         host: String,

//         #[arg(short = 'P', long, default_value_t = 4444)]
//         port: u16,
//     },
//     Reverse {
//         #[arg(short = 'H', long)]
//         host: String,

//         #[arg(short = 'P', long, default_value_t = 4444)]
//         port: u16,
//     },
// }

#[tokio::main]
async fn main() -> io::Result<()> {
    // let cli = Cli::parse();

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        run_cli(tx).await;
    });

    while let Some(cmd) = rx.recv().await {
        match cmd {
            CliCommand::Forward { host, port } => {
                tokio::spawn(async move { tcp_connect(host, port).await });
            }
            CliCommand::Reverse { host, port } => {
                tokio::spawn(async move { tcp_listen(host, port).await });
            }
            CliCommand::Quit => break,
        }
    }

    Ok(())
}

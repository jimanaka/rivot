mod cli_util;
mod error;
mod establish_connections;
use cli_util::run_cli;
use establish_connections::{tcp_connect, tcp_listen};
use tokio::io;
use tokio::sync::mpsc;

use crate::cli_util::CliCommand;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, mut rx) = mpsc::channel(32);

    let mut tasks = vec![];

    tasks.push(tokio::spawn(async move {
        run_cli(tx).await;
    }));

    while let Some(cmd) = rx.recv().await {
        match cmd {
            CliCommand::Forward { host, port } => {
                tasks.push(tokio::spawn(async move { tcp_connect(host, port).await }));
            }
            CliCommand::Reverse { host, port } => {
                tasks.push(tokio::spawn(async move { tcp_listen(host, port).await }));
            }
            CliCommand::Quit => break,
        }
    }

    for task in tasks {
        task.await.unwrap();
    }

    Ok(())
}

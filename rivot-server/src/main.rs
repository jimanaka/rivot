mod cli_util;
mod error;
mod establish_connections;
use cli_util::run_cli;
use dashmap::DashMap;
use establish_connections::{Connection, ConnectionMap, tcp_connect, tcp_listen};
use std::sync::Arc;
use tokio::io;
use tokio::sync::mpsc;

use crate::cli_util::CliCommand;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, mut rx) = mpsc::channel(32);

    let mut tasks = vec![];

    let connections_map: ConnectionMap = Arc::new(DashMap::new());

    let cli_con_map = connections_map.clone();
    tasks.push(tokio::spawn(async move {
        run_cli(tx, cli_con_map).await;
    }));

    while let Some(cmd) = rx.recv().await {
        match cmd {
            CliCommand::Forward { host, port } => {
                let map = connections_map.clone();
                tasks.push(tokio::spawn(
                    async move { tcp_connect(host, port, map).await },
                ));
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

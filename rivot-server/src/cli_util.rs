use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;

pub enum CliCommand {
    Forward { host: String, port: u16 },
    Reverse { host: String, port: u16 },
    Quit,
}

pub async fn run_cli(tx: mpsc::Sender<CliCommand>) {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    let mut stdout = io::stdout();

    loop {
        line.clear();
        stdout.write(b"> ").await.unwrap();
        stdout.flush().await.unwrap();
        reader.read_line(&mut line).await.unwrap();

        let split = line.trim().split(' ').collect::<Vec<&str>>();

        match split[0] {
            "quit" => {
                tx.send(CliCommand::Quit).await.unwrap();
            }
            "forward" => {
                if split.len() < 2 {
                    return;
                }
                tx.send(CliCommand::Forward {
                    host: "127.0.0.1".to_string(),
                    port: 4444,
                })
                .await
                .unwrap();
            }
            "reverse" => {
                tx.send(CliCommand::Reverse {
                    host: "127.0.0.1".to_string(),
                    port: 4444,
                })
                .await
                .unwrap();
            }
            cmd => println!("Unknown command: {cmd}"),
        }
    }
}

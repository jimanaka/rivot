use crate::error::RivotCliError;
use crate::establish_connections::ConnectionMap;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};
use std::borrow::Cow;
use std::collections::HashMap;
use tabled::{Table, Tabled};
use tokio::sync::mpsc;
use tokio::task;

pub enum CliCommand {
    Forward { host: String, port: u16 },
    Reverse { host: String, port: u16 },
    Quit,
}

struct RivotPrompt;

impl Prompt for RivotPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        Cow::Owned("rivot".to_string())
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _prompt_mode: PromptEditMode) -> Cow<'_, str> {
        Cow::Owned("> ".to_string())
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed("...")
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        Cow::Borrowed("search: ")
    }
}

#[derive(Tabled)]
pub struct ConnectionRow {
    #[tabled(rename = "ID")]
    name: String,
    #[tabled(rename = "Type")]
    kind: String,
}

fn print_help(command: &str) {
    match command {
        "help" => println!(
            r#"
forward - Start a forward tunnel
reverse - Start a reverse tunnel
ls      - List active connections
help    - Print this help statement
"#
        ),
        "forward" => println!(
            r#"
Start a forward tunnel

Usage: forward -H <host> -P <port>

Options:
    -H, --host    Target host to forward to
    -P, --port    Target port
    -h            Print this help statement

Example:
    forward -H 192.168.1.10 -P 8080
"#
        ),
        "reverse" => println!(
            r#"
Start a reverse tunnel

Usage: reverse -H <host> -P <port>

Options:
    -H, --host    Address to bind to
    -P, --port    Local bind port
    -h            Print this help statement

Example:
    reverse -H 127.0.0.1 -P 8080
"#
        ),
        "ls" => println!(
            r#"
List active connections

Usage: ls

Options:
    -h            Print this help statement

Example:
    ls                
"#
        ),
        _ => println!("This is default"),
    }
}

fn parse_flags<'a>(tokens: &[&'a str]) -> Result<HashMap<&'a str, &'a str>, RivotCliError> {
    let mut map = HashMap::new();

    let mut i = 0;

    while i < tokens.len() {
        if tokens[i].starts_with('-') {
            let key;
            let value;

            if tokens[i].len() < 2 {
                return Err(RivotCliError::InvalidArgError(format!(
                    "Invalid argument: {}",
                    tokens[i]
                )));
            }

            if tokens[i][1..].starts_with('-') {
                if tokens[i][1..].len() < 2 {
                    return Err(RivotCliError::InvalidArgError(format!(
                        "Invalid argument: {}",
                        tokens[i]
                    )));
                }
            }

            key = tokens[i];

            if i + 1 >= tokens.len() {
                value = "";
            } else {
                if tokens[i + 1].starts_with('-') {
                    value = "";
                } else {
                    value = tokens[i + 1];
                }
            }

            map.insert(key, value);
        }
        i += 1;
    }

    Ok(map)
}

pub async fn run_cli(tx: mpsc::Sender<CliCommand>, connections_map: ConnectionMap) {
    task::spawn_blocking(move || {
        let mut line = String::new();
        let mut line_editor = Reedline::create();
        let prompt = RivotPrompt;

        loop {
            line.clear();

            let sig = line_editor.read_line(&prompt);
            match sig {
                Ok(Signal::Success(buffer)) => {
                    line = buffer.clone();
                }
                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                    break;
                }
                x => {
                    println!("Event: {:?}", x);
                }
            }

            let tokens = line.trim().split_whitespace().collect::<Vec<&str>>();

            if tokens.len() < 1 {
                continue;
            }

            println!("Tokens: {:?}", tokens);

            match tokens[0] {
                "help" => {
                    print_help(tokens[0]);
                }
                "quit" => {
                    tx.blocking_send(CliCommand::Quit).unwrap();
                    break;
                }
                "ls" => {
                    let flags = match parse_flags(&tokens[1..]) {
                        Ok(map) => map,
                        Err(e) => {
                            println!("{e}");
                            print_help(tokens[0]);
                            continue;
                        }
                    };
                    if let Some(_help) = flags.get("-h") {
                        print_help(tokens[0]);
                        continue;
                    }

                    let rows: Vec<ConnectionRow> = connections_map
                        .iter()
                        .map(|entry| ConnectionRow {
                            name: entry.key().clone(),
                            kind: entry.value().kind.to_string(),
                        })
                        .collect();

                    if rows.is_empty() {
                        println!("No active connections");
                    } else {
                        println!("{}", Table::new(rows));
                    }
                }
                "forward" => {
                    let flags = match parse_flags(&tokens[1..]) {
                        Ok(map) => map,
                        Err(e) => {
                            println!("{e}");
                            print_help(tokens[0]);
                            continue;
                        }
                    };
                    if flags.is_empty() {
                        print_help(tokens[0]);
                        continue;
                    }
                    if let Some(_help) = flags.get("-h") {
                        print_help(tokens[0]);
                        continue;
                    }
                    let Some(host) = flags.get("-H").or_else(|| flags.get("--host")) else {
                        print_help(tokens[0]);
                        continue;
                    };
                    let Some(port) = flags.get("-P").or_else(|| flags.get("--port")) else {
                        print_help(tokens[0]);
                        continue;
                    };

                    tx.blocking_send(CliCommand::Forward {
                        host: host.to_string(),
                        port: port.parse().unwrap(),
                    })
                    .unwrap();
                }
                "reverse" => {
                    let flags = match parse_flags(&tokens[1..]) {
                        Ok(map) => map,
                        Err(e) => {
                            println!("{e}");
                            print_help(tokens[0]);
                            continue;
                        }
                    };
                    if flags.is_empty() {
                        print_help(tokens[0]);
                        continue;
                    }
                    if let Some(_help) = flags.get("-h") {
                        print_help(tokens[0]);
                        continue;
                    }
                    let Some(host) = flags.get("-H").or_else(|| flags.get("--host")) else {
                        print_help(tokens[0]);
                        continue;
                    };
                    let Some(port) = flags.get("-P").or_else(|| flags.get("--port")) else {
                        print_help(tokens[0]);
                        continue;
                    };
                    tx.blocking_send(CliCommand::Reverse {
                        host: host.to_string(),
                        port: port.parse().unwrap(),
                    })
                    .unwrap();
                }
                cmd => {
                    println!("Unknown command: {cmd}");
                }
            }
        }
    })
    .await
    .unwrap();
}

use reedline::{Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};
use std::borrow::Cow;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tokio::task;

pub enum CliCommand {
    Forward { host: String, port: u16 },
    Reverse { host: String, port: u16 },
    Quit,
}

impl CliCommand {
    pub fn help() -> &'static str {
        "Commands:
  forward    Start a forward tunnel
  reverse    Start a reverse tunnel
  help       Show this help message
  quit       Exit

Use 'help <command>' for more information."
    }

    pub fn help_forward() -> &'static str {
        "forward -H <host> -P <port>
  
  Start a forward tunnel to the specified host and port.
  
  Options:
    -H, --host    Target host
    -P, --port    Target port (default: 4444)"
    }

    pub fn help_reverse() -> &'static str {
        "reverse -H <host> -P <port>
  
  Start a reverse tunnel listening on the specified host and port.
  
  Options:
    -H, --host    Bind host
    -P, --port    Bind port (default: 4444)"
    }
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
        history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        Cow::Borrowed("search: ")
    }
}

pub async fn run_cli(tx: mpsc::Sender<CliCommand>) {
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

            let split = line.trim().split(' ').collect::<Vec<&str>>();

            match split[0] {
                "quit" => {
                    tx.blocking_send(CliCommand::Quit).unwrap();
                    break;
                }
                "help" => match split.get(1).map(|s| *s) {
                    None => println!("{}", CliCommand::help()),
                    Some("forward") => println!("{}", CliCommand::help_forward()),
                    Some("reverse") => println!("{}", CliCommand::help_reverse()),
                    Some(cmd) => println!("Unknown command: {cmd}"),
                },
                "forward" => {
                    if split.len() < 2 {
                        return;
                    }
                    tx.blocking_send(CliCommand::Forward {
                        host: "127.0.0.1".to_string(),
                        port: 4444,
                    })
                    .unwrap();
                }
                "reverse" => {
                    tx.blocking_send(CliCommand::Reverse {
                        host: "127.0.0.1".to_string(),
                        port: 4444,
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

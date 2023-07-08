// use std::future::Future;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use workflow_terminal::Terminal;
// use workflow_terminal::Options;
use workflow_log::*;
use workflow_terminal::parse;
use workflow_terminal::Cli;
use workflow_terminal::Result;

struct ExampleCli {
    term: Arc<Mutex<Option<Arc<Terminal>>>>,
}

impl ExampleCli {
    fn new() -> Self {
        ExampleCli {
            term: Arc::new(Mutex::new(None)),
        }
    }

    fn term(&self) -> Option<Arc<Terminal>> {
        self.term.lock().unwrap().as_ref().cloned()
    }
}

impl workflow_log::Sink for ExampleCli {
    fn write(&self, _target: Option<&str>, _level: Level, args: &std::fmt::Arguments<'_>) -> bool {
        // note, the terminal may not be initialized
        // if workflow_log::pipe() is bound before the
        // Terminal::init() is complete.
        if let Some(term) = self.term() {
            term.writeln(args.to_string());
            // true to disable further processing (no further output is made)
            true
        } else {
            // false for default log output handling (print to stdout or web console)
            false
        }
    }
}

#[async_trait]
impl Cli for ExampleCli {
    fn init(&self, term: &Arc<Terminal>) -> Result<()> {
        *self.term.lock().unwrap() = Some(term.clone());
        Ok(())
    }

    async fn digest(self: Arc<Self>, term: Arc<Terminal>, cmd: String) -> Result<()> {
        let argv = parse(&cmd);
        match argv[0].as_str() {
            "help" => {
                let commands = vec![
                    "help - this list",
                    "hello - simple text output",
                    "test - log_trace!() macro output",
                    "history - list command history",
                    "sleep - sleep for 5 seconds",
                    "ask - ask user for text input (with echo)",
                    "pass - ask user for password text input (no echo)",
                    "exit - exit terminal",
                ];
                term.writeln("\n\rCommands:\n\r");
                term.writeln("\t".to_string() + &commands.join("\n\r\t") + "\n\r");
            }
            "hello" => {
                term.writeln("hello back to you!");
            }
            "history" => {
                let history = term.history();
                for line in history.iter() {
                    term.writeln(line);
                }
            }
            "test" => {
                log_trace!("log_trace!() macro test");
            }
            "sleep" => {
                log_trace!("start sleep (5 sec)");
                workflow_core::task::sleep(Duration::from_millis(5000)).await;
                log_trace!("finish sleep");
            }
            "ask" => {
                let text = term.ask(false, "Enter something:").await?;
                log_info!("You have entered something: {}", text);
            }
            "pass" => {
                let text = term.ask(true, "Enter something:").await?;
                log_info!("You have entered something: {}", text);
            }
            "exit" => {
                term.writeln("bye!");
                term.exit().await;
            }
            _ => return Err(format!("command not found: {cmd}").into()),
        }

        Ok(())
    }

    async fn complete(self: Arc<Self>, _term: Arc<Terminal>, cmd: String) -> Result<Vec<String>> {
        let argv = parse(&cmd);
        if argv.is_empty() {
            return Ok(vec![]);
        }
        let last = argv.last().unwrap();
        if last.starts_with('a') {
            Ok(vec![
                "alpha".to_string(),
                "aloha".to_string(),
                "albatross".to_string(),
            ])
        } else {
            Ok(vec![])
        }
    }

    fn prompt(&self) -> Option<String> {
        None
    }
}

pub async fn example_terminal() -> Result<()> {
    let cli = Arc::new(ExampleCli::new());
    let term = Arc::new(Terminal::try_new(cli.clone(), "$ ")?);
    term.init().await?;

    // IMPORTANT: if redirecting workflow_log, using pipe()
    // the handler must be installed after Terminal::init()
    // is invoked.
    workflow_log::pipe(Some(cli.clone()));

    term.writeln("Terminal example (type 'help' for list of commands)");
    term.run().await?;

    Ok(())
}

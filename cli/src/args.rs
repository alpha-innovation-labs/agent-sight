#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    OpenCode,
    Claude,
}

impl Source {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenCode => "opencode",
            Self::Claude => "claude",
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Query {
        since: String,
        directory: Option<String>,
    },
    Session {
        session_id: String,
    },
    Filter {
        since: String,
        directory: Option<String>,
        filter: String,
    },
}

#[derive(Debug)]
pub struct Args {
    pub source: Source,
    pub command: Command,
    pub verbose: bool,
    pub full: bool,
}

pub fn print_help() {
    eprintln!("Promsight queries local OpenCode or Claude user history.\n");
    eprintln!("Usage:");
    eprintln!(
        "  cargo run -- query --since <duration> [--source <provider>] [--directory <path>] [--full] [--verbose]"
    );
    eprintln!("  cargo run -- session --id <session-id> [--source opencode] [--full] [--verbose]");
    eprintln!(
        "  cargo run -- filter <text> --since <duration> [--source <provider>] [--directory <path>] [--full] [--verbose]\n"
    );
    eprintln!("Providers:");
    eprintln!("  opencode    Query the OpenCode SQLite database");
    eprintln!("  claude      Query ~/.claude/history.jsonl\n");
    eprintln!("Options:");
    eprintln!("  --source     Provider to query, default: opencode");
    eprintln!("  --full       Return expanded conversation objects");
    eprintln!("  --verbose    Print step-by-step progress and timing");
    eprintln!("  --since      Time window like 24h or 7d");
    eprintln!("  --directory  Restrict to one directory/project");
    eprintln!("  --id         Session id for the `session` command\n");
    eprintln!("Examples:");
    eprintln!("  cargo run -- query --since 24h");
    eprintln!("  cargo run -- query --since 24h --source claude");
    eprintln!("  cargo run -- session --id ses_123");
    eprintln!("  cargo run -- filter \"rust sqlite\" --since 7d --source claude");
}

pub fn parse_args(argv: &[String]) -> Result<Option<Args>, String> {
    if argv.is_empty() || argv.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Ok(None);
    }

    let Some((command, rest)) = argv.split_first() else {
        return Ok(None);
    };

    match command.as_str() {
        "query" => parse_query_args(rest),
        "session" => parse_session_args(rest),
        "filter" => parse_filter_args(rest),
        _ => Err(format!("Unknown command: {command}")),
    }
    .map(Some)
}

fn parse_query_args(argv: &[String]) -> Result<Args, String> {
    let mut source = Source::OpenCode;
    let mut since = None;
    let mut directory = None;
    let mut verbose = false;
    let mut full = false;
    let mut index = 0;

    while index < argv.len() {
        match argv[index].as_str() {
            "--source" => {
                let Some(value) = argv.get(index + 1) else {
                    return Err("Missing value for --source".to_string());
                };
                source = parse_source(value)?;
                index += 2;
            }
            "--full" => {
                full = true;
                index += 1;
            }
            "--verbose" => {
                verbose = true;
                index += 1;
            }
            "--since" => {
                let Some(value) = argv.get(index + 1) else {
                    return Err("Missing value for --since".to_string());
                };
                since = Some(value.clone());
                index += 2;
            }
            "--directory" => {
                let Some(value) = argv.get(index + 1) else {
                    return Err("Missing value for --directory".to_string());
                };
                directory = Some(value.clone());
                index += 2;
            }
            token => return Err(format!("Unknown argument: {token}")),
        }
    }

    let Some(since) = since else {
        return Err("--since is required".to_string());
    };

    Ok(Args {
        source,
        command: Command::Query { since, directory },
        verbose,
        full,
    })
}

fn parse_session_args(argv: &[String]) -> Result<Args, String> {
    let mut source = Source::OpenCode;
    let mut session_id = None;
    let mut verbose = false;
    let mut full = false;
    let mut index = 0;

    while index < argv.len() {
        match argv[index].as_str() {
            "--source" => {
                let Some(value) = argv.get(index + 1) else {
                    return Err("Missing value for --source".to_string());
                };
                source = parse_source(value)?;
                index += 2;
            }
            "--full" => {
                full = true;
                index += 1;
            }
            "--verbose" => {
                verbose = true;
                index += 1;
            }
            "--id" => {
                let Some(value) = argv.get(index + 1) else {
                    return Err("Missing value for --id".to_string());
                };
                session_id = Some(value.clone());
                index += 2;
            }
            token => return Err(format!("Unknown argument: {token}")),
        }
    }

    let Some(session_id) = session_id else {
        return Err("--id is required".to_string());
    };

    Ok(Args {
        source,
        command: Command::Session { session_id },
        verbose,
        full,
    })
}

fn parse_filter_args(argv: &[String]) -> Result<Args, String> {
    let Some((filter, rest)) = argv.split_first() else {
        return Err("The `filter` command requires a filter string".to_string());
    };

    if filter.starts_with('-') {
        return Err("The `filter` command requires the filter text first".to_string());
    }

    let mut source = Source::OpenCode;
    let mut since = None;
    let mut directory = None;
    let mut verbose = false;
    let mut full = false;
    let mut index = 0;

    while index < rest.len() {
        match rest[index].as_str() {
            "--source" => {
                let Some(value) = rest.get(index + 1) else {
                    return Err("Missing value for --source".to_string());
                };
                source = parse_source(value)?;
                index += 2;
            }
            "--full" => {
                full = true;
                index += 1;
            }
            "--verbose" => {
                verbose = true;
                index += 1;
            }
            "--since" => {
                let Some(value) = rest.get(index + 1) else {
                    return Err("Missing value for --since".to_string());
                };
                since = Some(value.clone());
                index += 2;
            }
            "--directory" => {
                let Some(value) = rest.get(index + 1) else {
                    return Err("Missing value for --directory".to_string());
                };
                directory = Some(value.clone());
                index += 2;
            }
            token => return Err(format!("Unknown argument: {token}")),
        }
    }

    let Some(since) = since else {
        return Err("--since is required".to_string());
    };

    Ok(Args {
        source,
        command: Command::Filter {
            since,
            directory,
            filter: filter.clone(),
        },
        verbose,
        full,
    })
}

fn parse_source(value: &str) -> Result<Source, String> {
    match value {
        "opencode" => Ok(Source::OpenCode),
        "claude" => Ok(Source::Claude),
        _ => Err(format!("Unsupported --source value: {value}")),
    }
}

pub fn parse_since_to_hours(value: &str) -> Result<u64, String> {
    let normalized = value.trim().to_lowercase();
    let digits_len = normalized
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .count();

    if digits_len == 0 {
        return Err(format!(
            "Unsupported --since value: {value}. Use values like 24h or 7d."
        ));
    }

    let amount: u64 = normalized[..digits_len]
        .parse()
        .map_err(|_| format!("Unsupported --since value: {value}. Use values like 24h or 7d."))?;

    if amount < 1 {
        return Err("--since must be greater than 0".to_string());
    }

    let unit = normalized[digits_len..].trim();

    match unit {
        "h" | "hr" | "hrs" | "hour" | "hours" => Ok(amount),
        "d" | "day" | "days" => Ok(amount * 24),
        _ => Err(format!(
            "Unsupported --since value: {value}. Use values like 24h or 7d."
        )),
    }
}

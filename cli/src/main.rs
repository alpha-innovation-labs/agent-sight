mod args;
mod claude;
mod logger;
mod model;
mod opencode;
mod output;

use crate::args::{parse_args, parse_since_to_hours, print_help, Command as CliCommand, Source};
use crate::claude::history::query_history as query_claude_history;
use crate::logger::{format_duration, Logger};
use crate::model::{FullOutput, OutputConversation};
use crate::opencode::db::{query_history as query_opencode_history, query_session};
use crate::output::{filter_conversations_by_text, to_default_output};
use std::env;
use std::error::Error;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn run() -> AppResult<()> {
    let argv: Vec<String> = env::args().skip(1).collect();
    let Some(args) = parse_args(&argv).map_err(|message| -> Box<dyn Error> { message.into() })?
    else {
        print_help();
        return Ok(());
    };

    let logger = Logger::new(args.verbose);
    let total_started_at = SystemTime::now();

    let (source, command_name, since, directory, conversations) = match args.command {
        CliCommand::Query { since, directory } => {
            run_query_command(args.source, since, directory, &logger)?
        }
        CliCommand::Session { session_id } => {
            run_session_command(args.source, session_id, &logger)?
        }
        CliCommand::Filter {
            since,
            directory,
            filter,
        } => run_filter_command(args.source, since, directory, filter, &logger)?,
    };

    logger.log(&format!(
        "[promsight] Output mode: {}",
        if args.full { "full" } else { "compact" }
    ));
    logger.log(&format!(
        "[promsight] Grouped into {} conversation(s)",
        conversations.len()
    ));

    let full_output = FullOutput {
        source: source.as_str().to_string(),
        since,
        directory,
        conversation_count: conversations.len(),
        message_count: conversations
            .iter()
            .map(|conversation| conversation.user_message_count)
            .sum(),
        conversations,
    };

    logger.log(&format!(
        "[promsight] Final result has {} user message(s)",
        full_output.message_count
    ));
    logger.log(&format!(
        "[promsight] Total command time: {}",
        format_duration(total_started_at.elapsed()?)
    ));

    print_output(args.full, command_name, full_output)?;
    Ok(())
}

fn run_query_command(
    source: Source,
    since: String,
    directory: Option<String>,
    logger: &Logger,
) -> AppResult<(
    Source,
    &'static str,
    Option<String>,
    Option<String>,
    Vec<OutputConversation>,
)> {
    logger.log("[promsight] Starting query command");
    logger.log(&format!("[promsight] Source: {}", source.as_str()));
    logger.log(&format!("[promsight] Since filter: {since}"));
    logger.log(&format!(
        "[promsight] Directory filter: {}",
        directory.as_deref().unwrap_or("<all>")
    ));

    let cutoff_ms = cutoff_ms_from_since(&since)?;
    let conversations = match source {
        Source::OpenCode => query_opencode_history(cutoff_ms, directory.as_deref(), logger)?,
        Source::Claude => query_claude_history(cutoff_ms, directory.as_deref(), None, logger)?,
    };

    Ok((source, "query", Some(since), directory, conversations))
}

fn run_session_command(
    source: Source,
    session_id: String,
    logger: &Logger,
) -> AppResult<(
    Source,
    &'static str,
    Option<String>,
    Option<String>,
    Vec<OutputConversation>,
)> {
    if source != Source::OpenCode {
        return Err("The `session` command only supports --source opencode".into());
    }

    logger.log("[promsight] Starting session command");
    logger.log(&format!("[promsight] Session id: {session_id}"));

    Ok((
        source,
        "session",
        None,
        None,
        query_session(&session_id, logger)?,
    ))
}

fn run_filter_command(
    source: Source,
    since: String,
    directory: Option<String>,
    filter: String,
    logger: &Logger,
) -> AppResult<(
    Source,
    &'static str,
    Option<String>,
    Option<String>,
    Vec<OutputConversation>,
)> {
    logger.log("[promsight] Starting filter command");
    logger.log(&format!("[promsight] Source: {}", source.as_str()));
    logger.log(&format!("[promsight] Since filter: {since}"));
    logger.log(&format!(
        "[promsight] Directory filter: {}",
        directory.as_deref().unwrap_or("<all>")
    ));
    logger.log(&format!("[promsight] Filter text: {filter}"));

    let cutoff_ms = cutoff_ms_from_since(&since)?;
    let conversations = match source {
        Source::OpenCode => filter_conversations_by_text(
            query_opencode_history(cutoff_ms, directory.as_deref(), logger)?,
            &filter,
        ),
        Source::Claude => {
            query_claude_history(cutoff_ms, directory.as_deref(), Some(&filter), logger)?
        }
    };

    Ok((source, "filter", Some(since), directory, conversations))
}

fn cutoff_ms_from_since(since: &str) -> AppResult<i64> {
    let hours =
        parse_since_to_hours(since).map_err(|message| -> Box<dyn Error> { message.into() })?;
    let now_ms = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;
    Ok(now_ms - (hours as i64 * 3_600_000))
}

fn print_output(full: bool, command_name: &str, full_output: FullOutput) -> AppResult<()> {
    let output = if full {
        serde_json::to_value(&full_output)?
    } else {
        compact_output(command_name, &full_output.conversations)
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn compact_output(command_name: &str, conversations: &[OutputConversation]) -> serde_json::Value {
    if command_name == "session" && conversations.len() == 1 {
        let conversation = &conversations[0];
        return serde_json::Value::Array(
            conversation
                .messages
                .iter()
                .map(|message| serde_json::Value::String(message.content.clone()))
                .collect(),
        );
    }

    to_default_output(conversations)
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

use crate::logger::Logger;
use crate::model::{OutputConversation, OutputMessage};
use crate::AppResult;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::OffsetDateTime;

const DEFAULT_GAP_MS: i64 = 30 * 60 * 1000;
static DATETIME_FORMAT: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

#[derive(Deserialize)]
struct HistoryEntry {
    display: String,
    timestamp: i64,
    project: String,
}

pub fn query_history(
    cutoff_ms: i64,
    directory: Option<&str>,
    filter: Option<&str>,
    logger: &Logger,
) -> AppResult<Vec<OutputConversation>> {
    let history_path = resolve_history_path()?;
    logger.log(&format!(
        "[promsight] Reading Claude history from {}",
        history_path.display()
    ));

    let file = File::open(&history_path)?;
    let reader = BufReader::new(file);
    let normalized_filter = filter.map(str::to_lowercase);

    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let Ok(entry) = serde_json::from_str::<HistoryEntry>(&line) else {
            continue;
        };

        if entry.timestamp < cutoff_ms {
            continue;
        }

        if directory.is_some_and(|expected| entry.project != expected) {
            continue;
        }

        if normalized_filter.as_ref().is_some_and(|filter| {
            let text = entry.display.to_lowercase();
            !text.contains(filter)
        }) {
            continue;
        }

        rows.push(entry);
    }

    rows.sort_by(|left, right| {
        left.project
            .cmp(&right.project)
            .then_with(|| left.timestamp.cmp(&right.timestamp))
    });

    logger.log(&format!(
        "[promsight] Filtered to {} Claude history row(s)",
        rows.len()
    ));

    Ok(group_rows(rows))
}

fn resolve_history_path() -> AppResult<PathBuf> {
    let home = env::var("HOME")?;
    Ok(PathBuf::from(home).join(".claude/history.jsonl"))
}

fn group_rows(rows: Vec<HistoryEntry>) -> Vec<OutputConversation> {
    let mut conversations = Vec::new();
    let mut current_project = String::new();
    let mut current_started_at = 0_i64;
    let mut current_ended_at = 0_i64;
    let mut current_messages = Vec::<OutputMessage>::new();

    for row in rows {
        let starts_new_conversation = current_messages.is_empty()
            || row.project != current_project
            || row.timestamp - current_ended_at > DEFAULT_GAP_MS;

        if starts_new_conversation {
            flush_conversation(
                &mut conversations,
                &current_project,
                current_started_at,
                current_ended_at,
                &mut current_messages,
            );
            current_project = row.project.clone();
            current_started_at = row.timestamp;
        }

        current_ended_at = row.timestamp;
        current_messages.push(OutputMessage {
            message_id: format!("claude:{}", row.timestamp),
            created_at: format_timestamp(row.timestamp),
            content: row.display,
        });
    }

    flush_conversation(
        &mut conversations,
        &current_project,
        current_started_at,
        current_ended_at,
        &mut current_messages,
    );

    conversations.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));
    conversations
}

fn flush_conversation(
    conversations: &mut Vec<OutputConversation>,
    project: &str,
    started_at_ms: i64,
    ended_at_ms: i64,
    messages: &mut Vec<OutputMessage>,
) {
    if messages.is_empty() {
        return;
    }

    conversations.push(OutputConversation {
        session_id: format!("{}#{}", project, started_at_ms),
        title: None,
        directory: Some(project.to_string()),
        created_at: Some(format_timestamp(started_at_ms)),
        updated_at: Some(format_timestamp(ended_at_ms)),
        user_message_count: messages.len(),
        messages: std::mem::take(messages),
    });
}

fn format_timestamp(timestamp_ms: i64) -> String {
    OffsetDateTime::from_unix_timestamp(timestamp_ms / 1_000)
        .expect("valid unix timestamp")
        .format(DATETIME_FORMAT)
        .expect("valid datetime format")
}

use crate::logger::{format_duration, Logger};
use crate::model::{MessageRow, OutputConversation, PartRow};
use crate::output::group_rows;
use crate::AppResult;
use rusqlite::{params, Connection, OpenFlags, Row, ToSql};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::SystemTime;

const QUERY_MESSAGES_ALL_DIRECTORIES: &str = r#"
select
  s.id as session_id,
  s.title as title,
  s.directory as directory,
  s.time_created as session_created_at_ms,
  s.time_updated as session_updated_at_ms,
  m.id as message_id,
  m.time_created as message_created_at_ms,
  m.data as message_data
from message m
join session s on s.id = m.session_id
where m.time_created >= ?1
"#;

const QUERY_MESSAGES_BY_DIRECTORY: &str = r#"
select
  s.id as session_id,
  s.title as title,
  s.directory as directory,
  s.time_created as session_created_at_ms,
  s.time_updated as session_updated_at_ms,
  m.id as message_id,
  m.time_created as message_created_at_ms,
  m.data as message_data
from message m
join session s on s.id = m.session_id
where m.time_created >= ?1
  and s.directory = ?2
"#;

const QUERY_MESSAGES_BY_SESSION: &str = r#"
select
  s.id as session_id,
  s.title as title,
  s.directory as directory,
  s.time_created as session_created_at_ms,
  s.time_updated as session_updated_at_ms,
  m.id as message_id,
  m.time_created as message_created_at_ms,
  m.data as message_data
from message m
join session s on s.id = m.session_id
where s.id = ?1
"#;

const QUERY_PROJECTS: &str = r#"
select
  s.directory as directory
from session s
where trim(s.directory) <> ''
group by s.directory
order by max(s.time_updated) desc, s.directory asc
"#;

pub fn query_history(
    cutoff_ms: i64,
    directory: Option<&str>,
    logger: &Logger,
) -> AppResult<Vec<OutputConversation>> {
    let database_path = resolve_database_path()?;
    let (messages, parts_by_message) = run_query(&database_path, cutoff_ms, directory, logger)?;
    Ok(group_rows(messages, parts_by_message))
}

pub fn query_session(session_id: &str, logger: &Logger) -> AppResult<Vec<OutputConversation>> {
    let database_path = resolve_database_path()?;
    let (messages, parts_by_message) = run_session_query(&database_path, session_id, logger)?;
    Ok(group_rows(messages, parts_by_message))
}

pub fn query_projects(logger: &Logger) -> AppResult<Vec<String>> {
    let database_path = resolve_database_path()?;
    run_projects_query(&database_path, logger)
}

fn resolve_database_path() -> AppResult<String> {
    if let Ok(path) = env::var("PROMSIGHT_OPENCODE_DB") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    if let Ok(path) = env::var("XDG_DATA_HOME") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Ok(PathBuf::from(trimmed)
                .join("opencode")
                .join("opencode.db")
                .display()
                .to_string());
        }
    }

    let home = env::var("HOME")?;
    let path = PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("opencode")
        .join("opencode.db");
    Ok(path.display().to_string())
}

fn run_query(
    database_path: &str,
    cutoff_ms: i64,
    directory: Option<&str>,
    logger: &Logger,
) -> AppResult<(Vec<MessageRow>, HashMap<String, Vec<PartRow>>)> {
    logger.log("[promsight] Running SQLite query against OpenCode DB");
    logger.log(&format!("[promsight] Database path: {database_path}"));

    let started_at = SystemTime::now();
    let connection = Connection::open_with_flags(
        database_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;

    let recent_messages = match directory {
        Some(directory) => query_message_rows(
            &connection,
            QUERY_MESSAGES_BY_DIRECTORY,
            params![cutoff_ms, directory],
        )?,
        None => query_message_rows(
            &connection,
            QUERY_MESSAGES_ALL_DIRECTORIES,
            params![cutoff_ms],
        )?,
    };

    logger.log(&format!(
        "[promsight] Retrieved {} recent message row(s)",
        recent_messages.len()
    ));

    let mut messages = recent_messages
        .into_iter()
        .filter(|message| message_role_is_user(&message.data))
        .collect::<Vec<_>>();

    messages.sort_by(|left, right| {
        right
            .session_updated_at_ms
            .cmp(&left.session_updated_at_ms)
            .then_with(|| left.message_created_at_ms.cmp(&right.message_created_at_ms))
            .then_with(|| left.message_id.cmp(&right.message_id))
    });

    logger.log(&format!(
        "[promsight] Filtered to {} user message row(s)",
        messages.len()
    ));

    let parts_by_message = query_part_rows(&connection, &messages, logger)?;

    logger.log(&format!(
        "[promsight] Query completed in {}",
        format_duration(started_at.elapsed()?)
    ));

    Ok((messages, parts_by_message))
}

fn run_session_query(
    database_path: &str,
    session_id: &str,
    logger: &Logger,
) -> AppResult<(Vec<MessageRow>, HashMap<String, Vec<PartRow>>)> {
    logger.log("[promsight] Running SQLite session query against OpenCode DB");
    logger.log(&format!("[promsight] Database path: {database_path}"));

    let started_at = SystemTime::now();
    let connection = Connection::open_with_flags(
        database_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;

    let mut messages =
        query_message_rows(&connection, QUERY_MESSAGES_BY_SESSION, params![session_id])?
            .into_iter()
            .filter(|message| message_role_is_user(&message.data))
            .collect::<Vec<_>>();

    messages.sort_by(|left, right| {
        right
            .session_updated_at_ms
            .cmp(&left.session_updated_at_ms)
            .then_with(|| left.message_created_at_ms.cmp(&right.message_created_at_ms))
            .then_with(|| left.message_id.cmp(&right.message_id))
    });

    logger.log(&format!(
        "[promsight] Filtered to {} user message row(s) for session",
        messages.len()
    ));

    let parts_by_message = query_part_rows(&connection, &messages, logger)?;

    logger.log(&format!(
        "[promsight] Query completed in {}",
        format_duration(started_at.elapsed()?)
    ));

    Ok((messages, parts_by_message))
}

fn run_projects_query(database_path: &str, logger: &Logger) -> AppResult<Vec<String>> {
    logger.log("[promsight] Running SQLite projects query against OpenCode DB");
    logger.log(&format!("[promsight] Database path: {database_path}"));

    let started_at = SystemTime::now();
    let connection = Connection::open_with_flags(
        database_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;

    let mut statement = connection.prepare(QUERY_PROJECTS)?;
    let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
    let projects = rows.collect::<Result<Vec<_>, _>>()?;

    logger.log(&format!(
        "[promsight] Retrieved {} project(s)",
        projects.len()
    ));
    logger.log(&format!(
        "[promsight] Query completed in {}",
        format_duration(started_at.elapsed()?)
    ));

    Ok(projects)
}

fn query_message_rows<P>(
    connection: &Connection,
    sql: &str,
    params: P,
) -> AppResult<Vec<MessageRow>>
where
    P: rusqlite::Params,
{
    let mut statement = connection.prepare(sql)?;
    let rows = statement.query_map(params, map_message_row)?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn map_message_row(row: &Row<'_>) -> rusqlite::Result<MessageRow> {
    Ok(MessageRow {
        session_id: row.get(0)?,
        title: row.get(1)?,
        directory: row.get(2)?,
        session_created_at_ms: row.get(3)?,
        session_updated_at_ms: row.get(4)?,
        message_id: row.get(5)?,
        message_created_at_ms: row.get(6)?,
        data: row.get(7)?,
    })
}

fn query_part_rows(
    connection: &Connection,
    messages: &[MessageRow],
    logger: &Logger,
) -> AppResult<HashMap<String, Vec<PartRow>>> {
    if messages.is_empty() {
        return Ok(HashMap::new());
    }

    let mut all_parts = Vec::new();
    for chunk in messages.chunks(500) {
        let sql = build_part_query(chunk.len());
        let params = chunk
            .iter()
            .map(|message| &message.message_id as &dyn ToSql);
        let mut statement = connection.prepare(&sql)?;
        let rows = statement.query_map(rusqlite::params_from_iter(params), map_part_row)?;
        all_parts.extend(rows.collect::<Result<Vec<_>, _>>()?);
    }

    all_parts.sort_by(|left, right| {
        left.message_id
            .cmp(&right.message_id)
            .then_with(|| left.time_created_ms.cmp(&right.time_created_ms))
    });

    let mut parts_by_message = HashMap::<String, Vec<PartRow>>::new();
    for part in all_parts
        .into_iter()
        .filter(|part| part_type_is_text(&part.data))
    {
        parts_by_message
            .entry(part.message_id.clone())
            .or_default()
            .push(part);
    }

    logger.log(&format!(
        "[promsight] Retrieved text parts for {} message(s)",
        parts_by_message.len()
    ));

    Ok(parts_by_message)
}

fn build_part_query(parameter_count: usize) -> String {
    let mut sql =
        String::from("select message_id, time_created, data from part where message_id in (");

    for index in 0..parameter_count {
        if index > 0 {
            sql.push_str(", ");
        }
        let _ = write!(sql, "?{}", index + 1);
    }

    sql.push(')');
    sql
}

fn map_part_row(row: &Row<'_>) -> rusqlite::Result<PartRow> {
    Ok(PartRow {
        message_id: row.get(0)?,
        time_created_ms: row.get(1)?,
        data: row.get(2)?,
    })
}

fn message_role_is_user(data: &str) -> bool {
    json_string_field_equals(data, "role", "user")
}

fn part_type_is_text(data: &str) -> bool {
    json_string_field_equals(data, "type", "text")
}

fn json_string_field_equals(data: &str, field: &str, expected: &str) -> bool {
    serde_json::from_str::<Value>(data)
        .ok()
        .and_then(|value| value.get(field).and_then(Value::as_str).map(str::to_owned))
        .as_deref()
        == Some(expected)
}

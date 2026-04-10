use crate::model::{MessageRow, OutputConversation, OutputMessage, PartRow};
use serde_json::{Map, Value};
use std::collections::HashMap;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::OffsetDateTime;

static DATETIME_FORMAT: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

pub fn group_rows(
    messages: Vec<MessageRow>,
    mut parts_by_message: HashMap<String, Vec<PartRow>>,
) -> Vec<OutputConversation> {
    let mut conversations = Vec::<OutputConversation>::new();
    let mut indexes = HashMap::<String, usize>::new();

    for message in messages {
        let Some(parts) = parts_by_message.remove(&message.message_id) else {
            continue;
        };

        let content = parts
            .into_iter()
            .filter_map(|part| text_from_part_data(&part.data))
            .collect::<Vec<_>>()
            .join("\n\n");

        if content.is_empty() {
            continue;
        }

        if let Some(index) = indexes.get(&message.session_id).copied() {
            let conversation = &mut conversations[index];
            conversation.messages.push(OutputMessage {
                message_id: message.message_id,
                created_at: format_timestamp(message.message_created_at_ms),
                content,
            });
            conversation.user_message_count += 1;
            continue;
        }

        indexes.insert(message.session_id.clone(), conversations.len());
        conversations.push(OutputConversation {
            session_id: message.session_id,
            title: Some(message.title),
            directory: Some(message.directory),
            created_at: Some(format_timestamp(message.session_created_at_ms)),
            updated_at: Some(format_timestamp(message.session_updated_at_ms)),
            user_message_count: 1,
            messages: vec![OutputMessage {
                message_id: message.message_id,
                created_at: format_timestamp(message.message_created_at_ms),
                content,
            }],
        });
    }

    conversations
}

pub fn to_default_output(conversations: &[OutputConversation]) -> Value {
    let mut output = Map::new();
    for conversation in conversations {
        output.insert(
            conversation.session_id.clone(),
            Value::Array(
                conversation
                    .messages
                    .iter()
                    .map(|message| Value::String(message.content.clone()))
                    .collect(),
            ),
        );
    }

    Value::Object(output)
}

pub fn filter_conversations_by_text(
    conversations: Vec<OutputConversation>,
    filter: &str,
) -> Vec<OutputConversation> {
    let normalized_filter = filter.to_lowercase();

    conversations
        .into_iter()
        .filter_map(|mut conversation| {
            conversation.messages.retain(|message| {
                let content = message.content.to_lowercase();
                content.contains(&normalized_filter)
            });

            if conversation.messages.is_empty() {
                return None;
            }

            conversation.user_message_count = conversation.messages.len();
            Some(conversation)
        })
        .collect()
}

fn format_timestamp(timestamp_ms: i64) -> String {
    OffsetDateTime::from_unix_timestamp(timestamp_ms / 1_000)
        .expect("valid unix timestamp")
        .format(DATETIME_FORMAT)
        .expect("valid datetime format")
}

fn text_from_part_data(data: &str) -> Option<String> {
    let value = serde_json::from_str::<Value>(data).ok()?;
    value.get("text")?.as_str().map(ToOwned::to_owned)
}

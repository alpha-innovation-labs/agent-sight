use serde::Serialize;

#[derive(Debug)]
pub struct MessageRow {
    pub session_id: String,
    pub title: String,
    pub directory: String,
    pub session_created_at_ms: i64,
    pub session_updated_at_ms: i64,
    pub message_id: String,
    pub message_created_at_ms: i64,
    pub data: String,
}

#[derive(Debug)]
pub struct PartRow {
    pub message_id: String,
    pub time_created_ms: i64,
    pub data: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputMessage {
    pub message_id: String,
    pub created_at: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputConversation {
    pub session_id: String,
    pub title: Option<String>,
    pub directory: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub user_message_count: usize,
    pub messages: Vec<OutputMessage>,
}

#[derive(Debug, Serialize)]
pub struct FullOutput {
    pub source: String,
    pub since: Option<String>,
    pub directory: Option<String>,
    pub conversation_count: usize,
    pub message_count: usize,
    pub conversations: Vec<OutputConversation>,
}

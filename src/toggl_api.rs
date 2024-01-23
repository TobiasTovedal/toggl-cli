use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};
use crate::config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    email: String,
    api_token: String,
}

// Placeholder struct for return variable from POST time entry
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryResponse {
    at: String,
    billable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    duration: i64,
    duronly: bool,
    id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid: Option<i64>,  // Project ID, legacy field
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<i64>,  // Project ID. Can be null if project was not provided or project was later deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    server_deleted_at: Option<String>,  // When was deleted, null if not deleted
    start: String,  // Start time in UTC
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<String>,  // Stop time in UTC, can be null if it's still running or created with "duration" and "duronly" fields
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_ids: Option<Vec<i64>>,  // Array of integer Tag IDs, null if tags were not provided or were later deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,  // Array of string Tag names, null if tags were not provided or were later deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    task_id: Option<i64>,  // Task ID. Can be null if task was not provided or project was later deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    tid: Option<i64>,  // Task ID, legacy field
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<i64>,  // Time Entry creator ID, legacy field
    user_id: i64,  // Time Entry creator ID
    #[serde(skip_serializing_if = "Option::is_none")]
    wid: Option<i64>,  // Workspace ID, legacy field
    workspace_id: i64,  // Workspace ID
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,
    pub created_with: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duronly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i32>,
    pub start: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wid: Option<i64>,
    pub workspace_id: i64,
}

pub struct TogglApiWrapper {
    client: Client,
}

impl TogglApiWrapper {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        TogglApiWrapper { client }
    }

    pub async fn get_user_info(&self) -> Result<Person, reqwest::Error> {
        let response: Person = self.client.get(config::TOGGL_URL_ME)
        .basic_auth(config::API_KEY, Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json()
        .await?;

        Ok(response)
    }

    pub async fn add_time_entry(&self, time_entry: TimeEntryRequest) -> Result<TimeEntryResponse, reqwest::Error> {
        // Serialize the TimeEntryRequest instance to a JSON string
        let time_entry_json = serde_json::to_string(&time_entry);

        let time_entry_response: TimeEntryResponse = self.client
            .post(config::TOGGL_URL_TIME_ENTRIES)
            .basic_auth(config::API_KEY, Some("api_token"))
            .header(CONTENT_TYPE, "application/json")
            .body(time_entry_json.unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok(time_entry_response)   
    }
}

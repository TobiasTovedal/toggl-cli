use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

extern crate tokio;
extern crate serde_json;
mod access_token;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    email: String,
    api_token: String,
}

// Placeholder struct for return variable from POST time entry
#[derive(Debug, Serialize, Deserialize)]
struct TimeEntryResponse {
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
struct TimeEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    billable: Option<bool>,
    created_with: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    duronly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<i64>,
    start: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wid: Option<i64>,
    workspace_id: i64,
}

#[tokio::main]

async fn main() -> Result<(), reqwest::Error>  {
    let client = reqwest::Client::new();

    // Get information on current user?    
    let response: Person = client.get("https://api.track.toggl.com/api/v9/me")
        .basic_auth(access_token::get_access_token(), Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", response);

    // Create an instance of the TimeEntry struct with optional fields set to None
    let time_entry = TimeEntry {
        billable: None,
        created_with: "MyApp".to_string(),
        description: None,
        duration: -1,
        duronly: None,
        pid: None,
        project_id: None,
        start: "2024-01-16T06:00:00Z".to_string(),
        start_date: None,
        stop: None,
        tag_action: None,
        tag_ids: None,
        tags: None,
        task_id: None,
        tid: None,
        uid: None,
        user_id: None,
        wid: None,
        workspace_id: 1127770,
    };

    // Serialize the TimeEntryRequest instance to a JSON string
    let json_string = serde_json::to_string_pretty(&time_entry).unwrap();
    println!("{:#?}", json_string);


    // Post time entry
    // TODO: Update json payload
    let time_entry_response: TimeEntryResponse = client.post("https://api.track.toggl.com/api/v9/workspaces/1127770/time_entries")
        .basic_auth(access_token::get_access_token(), Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .body(json_string)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", time_entry_response);

    Ok(())

    // Different behaviours depending on response status.
    // TODO: Implement cases for all status code responses according to Toggl API reference: https://developers.track.toggl.com/docs/#generic-responses
    /* 
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("ok {:?}", response.);
        }
        reqwest::StatusCode::FORBIDDEN => {
            println!("forbidden");
        }
        reqwest::StatusCode::NOT_FOUND => {
            println!("not found");
        }
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
            println!("internal server error");
        }
        _ => {
            println!("oh god, no");
        }
    }
    */
    
}
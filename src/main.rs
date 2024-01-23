extern crate tokio;
extern crate serde_json;
mod config;
mod toggl_api;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use toggl_api::{TogglApiWrapper, TimeEntryRequest};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    project: String,
    duration: i32,
    description: String,
}

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

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Parse input arguments
    let args = Cli::parse();

    // Greate hashmap from array of projects and corresponding IDs
    let projects = HashMap::from(config::PROJECTS);
    
    // TODO: make better use of the cargo crate
    // If the project provided as argument exists, return it's id from hashmap
    match projects.get(args.project.as_str()){
        Some(project_id) => {
            // Instance of API wrapper
            let toggl_api = TogglApiWrapper::new();
            let time_entry: TimeEntryRequest = create_time_entry(project_id, args.duration, &args.description);
            let result = toggl_api.add_time_entry(time_entry).await;
        },
        None => eprintln!("No {:?} project exist.", args.project.as_str()),
    };

    Ok(())  
}

fn create_time_entry(project_id: &i32, duration: i32, description: &str) -> TimeEntryRequest {
    // Create an instance of the TimeEntry struct with optional fields set to None
    let time_entry = TimeEntryRequest {
        billable: None,
        created_with: "MyApp".to_string(),
        description: None,
        duration: -1,
        duronly: None,
        pid: None,
        project_id: Some(project_id.to_owned()),
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

    time_entry
}
extern crate tokio;
extern crate serde_json;
mod config;
mod toggl_api;
use std::collections::HashMap;
use chrono::Utc;

use toggl_api::{TogglApiWrapper, TimeEntryRequest};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        project: String,
        duration: i32,
        description: String,
    },
    Ls,
}

#[tokio::main]
async fn main() {
    // Parse input arguments
    let args = Cli::parse();

    // Greate hashmap from array of projects and corresponding IDs
    let projects = HashMap::from(config::PROJECTS);
    
    match &args.command {
        Commands::Add { project, duration, description } => {
            add_time_entry(project, duration, description, projects).await;
        }
        Commands::Ls => {
            print_available_projects(projects);
        }
    }
}

async fn add_time_entry(project: &str, duration: &i32, description: &str, projects: HashMap<&str, i32>) {
    // If the project provided as argument exists, return it's id from hashmap
    match projects.get(project){
        Some(project_id) => {
            // Instance of API wrapper
            let toggl_api = TogglApiWrapper::new();
            // Create time entry object
            let time_entry: TimeEntryRequest = create_time_entry(project_id, duration, description);
            // Try to add time entry
            match toggl_api.add_time_entry(time_entry).await {
                Ok(_) => {
                    println!("\x1b[92mSuccessfully added time entry \x1b[0m")
                }, 
                Err(error) => {
                    eprintln!("\x1b[91mError adding time entry. No time entry added. {:?}\x1b[0m", error);
                }
            };
        },
        None => {
            eprintln!("\x1b[93mNo {:?} project exist. Available projects are: \x1b[0m", project);
            print_available_projects(projects);
        }
    };
}

fn print_available_projects(projects: HashMap<&str, i32>) {
    // Print all available projects
    for project_key in projects.keys() {
        println!{"{}", project_key};
    }
}

fn create_time_entry(project_id: &i32, duration: &i32, description: &str) -> TimeEntryRequest {
    let now = Utc::now();

    // Create an instance of the TimeEntry struct with optional fields set to None
    let time_entry = TimeEntryRequest {
        billable: None,
        created_with: "toggl-cli".to_string(),
        description: Some(description.to_owned()),
        duration: duration * 60, // Kanske behöver vara owned? Varför idk?
        duronly: None,
        pid: None,
        project_id: Some(project_id.to_owned()),
        start: format!("{:?}", now).to_string(),
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
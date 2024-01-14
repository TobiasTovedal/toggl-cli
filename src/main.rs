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

    print!("{:#?}", response);

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
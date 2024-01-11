use reqwest::header::CONTENT_TYPE;

extern crate tokio;
extern crate serde_json;
mod access_token;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Get information on current user?
    let response = client.get("https://api.track.toggl.com/api/v9/me")
        .basic_auth(access_token::get_access_token(), Some("api_token"))
        .header(CONTENT_TYPE, "application/jsaon")
        .send()
        .await
        .unwrap();

    // Different behaviours depending on response status.
    // TODO: Implement cases for all status code responses according to Toggl API reference: https://developers.track.toggl.com/docs/#generic-responses
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("ok {:?}", response.text().await);
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
}
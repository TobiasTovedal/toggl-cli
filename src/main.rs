extern crate tokio;
extern crate serde_json;
mod access_token;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Get information on current user?
    let response = client.get("https://api.track.toggl.com/api/v9/me").basic_auth(access_token::get_access_token(), Some("api_token")).send().await.unwrap();

    // Different behaviours depending on response status. Possible statuses from Toggl API: 200, 403, 404, 500.
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("ok");
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
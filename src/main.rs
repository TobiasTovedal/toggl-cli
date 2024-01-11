extern crate tokio;
extern crate serde_json;
mod access_token;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    // Get information on current user?
    let result = client.get("https://api.track.toggl.com/api/v9/me").basic_auth(access_token::get_access_token(), Some("api_token")).send().await.unwrap().text().await.unwrap();

    println!("{:?}", result);
}
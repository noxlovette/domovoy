use domovoy::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    // Load OAuth token from environment variable
    let token = env::var("YANDEX_SMART_HOME_TOKEN")
        .expect("YANDEX_SMART_HOME_TOKEN environment variable must be set");

    // Initialize the client
    let client = Client::new(token)?;

    // Fetch user info
    println!("Fetching user info...");
    let user_info = client.user_info().await?;

    // Print the result as pretty-printed JSON
    println!("{}", serde_json::to_string_pretty(&user_info)?);

    Ok(())
}

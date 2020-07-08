use std::time::Duration;

use reqwest::Client;
use serde::Deserialize;
use tokio::time::delay_for;

#[derive(Deserialize, Debug)]
struct ImageTags {
    name: String,
    tags: Vec<String>,
}

const DURATION: Duration = Duration::from_millis(1000);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    loop {
        delay_for(DURATION).await;
        let resp = client
            .get("http://localhost:5000/v2/myfirstimage3/tags/list")
            .send()
            .await?
            .json::<ImageTags>()
            .await?;
        println!("{:#?}", resp);
    }
}

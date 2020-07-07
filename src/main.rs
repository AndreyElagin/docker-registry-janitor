use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ImageTags {
    name: String,
    tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:5000/v2/myfirstimage/tags/list")
        .await?
        .json::<ImageTags>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

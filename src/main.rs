use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
    Client,
};
use std::error::Error;
use std::env;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    dotenv::from_filename(".env").ok();
    let api_key = dotenv::var("OPENAI_API_KEY").unwrap();
    env::set_var("OPENAI_API_KEY", &api_key);

    let client = Client::new();

    let request = CreateImageRequestArgs::default()
        .prompt("A metallic blue bird with a pig's head flying over the Empire State Building in a storm")
        .n(2)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S512x512)
        .user("async-openai")
        .build()?;

    let response = client.images().create(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}
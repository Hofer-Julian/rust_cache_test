use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?
        .text()
        .await?;
    println!("Response: {}", response);
    Ok(())
}

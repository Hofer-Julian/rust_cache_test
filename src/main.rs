use reqwest::Error;

async fn fetch_todo() -> Result<String, Error> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?
        .text()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = fetch_todo().await?;
    println!("Response:{}", response);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_todo() {
        let result = fetch_todo().await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("\"id\": 1"));
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://httpbin.org/get")
        .await?
        .text()
        .await?;

    println!("{}", response);
    Ok(())
}

use reqwest::Client;

#[derive(serde::Serialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(serde::Deserialize, Debug)]
struct HttpBinResponse {
    json: serde_json::Value, // httpbin.org/post 返回的数据会放在 json 字段
    url: String,
    headers: serde_json::Value,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let addr = "https://httpbin.org/get";
    match send_get_request_async(&client, addr).await {
        Ok(body) => {
            println!("Success:\n {}", body);
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
        }
    }

    let payload = LoginPayload {
        username: "user".to_string(),
        password: "pass".to_string(),
    };

    let addr = "https://httpbin.org/post";

    let resp: HttpBinResponse = send_post_request_async(&client, addr, &payload).await?;
    println!("Response: {:?}", resp);

    Ok(())
}

async fn send_get_request_async(client: &Client, addr: &str) -> anyhow::Result<String> {
    let resp = client.get(addr).send().await?;

    if resp.status().is_success() {
        let body = resp.text().await?;
        Ok(body)
    } else {
        let status = resp.status();
        let url = resp.url().clone();
        Err(anyhow::anyhow!(
            "Http Code {} from {}: {}",
            status,
            url,
            resp.text().await?
        ))
    }
}

async fn send_post_request_async<T, R>(client: &Client, addr: &str, data: &T) -> anyhow::Result<R>
where
    T: serde::Serialize,
    R: serde::de::DeserializeOwned,
{
    let resp = client
        .post(addr)
        .json(data)
        .send()
        .await?
        .error_for_status()?
        .json::<R>()
        .await?;
    Ok(resp)
}

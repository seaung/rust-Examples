// use reqwest::Client;
//
// #[derive(serde::Serialize)]
// struct LoginPayload {
//     username: String,
//     password: String,
// }
//
// #[derive(serde::Deserialize, Debug)]
// struct HttpBinResponse {
//     json: serde_json::Value, // httpbin.org/post 返回的数据会放在 json 字段
//     url: String,
//     headers: serde_json::Value,
// }
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let client = Client::new();
//
//     let addr = "https://httpbin.org/get";
//     match send_get_request_async(&client, addr).await {
//         Ok(body) => {
//             println!("Success:\n {}", body);
//         }
//         Err(e) => {
//             eprintln!("Request failed: {}", e);
//         }
//     }
//
//     let payload = LoginPayload {
//         username: "user".to_string(),
//         password: "pass".to_string(),
//     };
//
//     let addr = "https://httpbin.org/post";
//
//     let resp: HttpBinResponse = send_post_request_async(&client, addr, &payload).await?;
//     println!("Response: {:?}", resp);
//
//     Ok(())
// }
//
// async fn send_get_request_async(client: &Client, addr: &str) -> anyhow::Result<String> {
//     let resp = client.get(addr).send().await?;
//
//     if resp.status().is_success() {
//         let body = resp.text().await?;
//         Ok(body)
//     } else {
//         let status = resp.status();
//         let url = resp.url().clone();
//         Err(anyhow::anyhow!(
//             "Http Code {} from {}: {}",
//             status,
//             url,
//             resp.text().await?
//         ))
//     }
// }
//
// async fn send_post_request_async<T, R>(client: &Client, addr: &str, data: &T) -> anyhow::Result<R>
// where
//     T: serde::Serialize,
//     R: serde::de::DeserializeOwned,
// {
//     let resp = client
//         .post(addr)
//         .json(data)
//         .send()
//         .await?
//         .error_for_status()?
//         .json::<R>()
//         .await?;
//     Ok(resp)
// }

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let body = reqwest::get("https://www.baidu.com")
//         .await?
//         .text()
//         .await?;
//     println!("{:#?}", body);
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let client = reqwest::Client::new();
//     let resp = client.post("https://httpbin.org/post")
//         .body("request post body")
//         .send()
//     .await?;
//
//     println!("Status: {}", resp.status());
//     println!("Headers:\n{:#?}", resp.headers());
//     let body = resp.text().await?;
//     println!("Body:\n{}", body);
//
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let params = [("username", "admin"), ("password", "admin")];
//     let client = reqwest::Client::new();
//     let resp = client.post("https://api.github.com/graphql")
//         .form(&params)
//         .send().await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

// use std::collections::HashMap;
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let mut json_data = HashMap::new();
//     json_data.insert(String::from("key"), String::from("value"));
//     json_data.insert(String::from("key1"), String::from("value1"));
//     let client = reqwest::Client::new();
//     let response = client.post("https://httpbin.org/post")
//         .json(&json_data)
//         .send()
//         .await?;
//     println!("{:#?}", response);
//
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let mut headers = reqwest::header::HeaderMap::new();
//     headers.insert("X-HeaderValue", reqwest::header::HeaderValue::from_static("hello"));
//     let client = reqwest::Client::builder()
//     .default_headers(headers)
//     .build()?;
//
//     // let res = client
//     //     .get("https://www.rust-lang.org")
//     //     .header("X-HEADER-1", "overriden val1")
//     //     .send()
//     //     .await?;
//
//     let res = client.get("https://www.rust-lang.org/").send().await?;
//     println!("Status: {}", res.status());
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let client = reqwest::Client::builder().build()?;
//
//     let resp = client
//         .get("https://httpbin.org/status/400")
//         .send()
//         .await?;
//
//     match resp.status() {
//         reqwest::StatusCode::OK => {
//             println!("200: {}", resp.text().await?);
//         }
//         reqwest::StatusCode::BAD_REQUEST => {
//             println!("400: {}", resp.text().await?);
//         }
//         status => {
//             anyhow::bail!("http status {} - {}", status, resp.text().await?);
//         }
//     }
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let client = reqwest::Client::builder().build()?;
//     let res = client
//         .get("https://httpbin.org/image/png")
//         .send()
//         .await?
//         .bytes()
//         .await?;
//     let mut data = res.as_ref();
//     let mut f = std::fs::File::create("image.png")?;
//     std::io::copy(&mut data, &mut f)?;
//     Ok(())
// }

// fn main() -> anyhow::Result<()> {
//     let resp = reqwest::blocking::get("https://httpbin.org/get")?;
//     println!("Status: {}", resp.status());
//     Ok(())
// }

// fn main() -> anyhow::Result<()> {
//     let client = reqwest::blocking::Client::new();
//
//     let resp = client.post("https://httpbin.org/post")
//         .body("body")
//         .send()?;
//     println!("{:#?}", resp);
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let client = reqwest::Client::builder()
//         .redirect(reqwest::redirect::Policy::limited(2))
//         .timeout(std::time::Duration::from_secs(60))
//         .build()?;
//
//     let resp = client.get("http://httpbin.org/redirect/2").send().await?;
//     println!("Status: {}", resp.status());
//     Ok(())
// }

fn main() {
    
}
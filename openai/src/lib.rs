use openai::OpenAIRequest;
use reqwest::header::HeaderValue;

mod openai;

pub use openai::*;

pub async fn get_from_openai(
    url: &str,
    key: &str,
    request: &OpenAIRequest,
) -> anyhow::Result<openai::OpenAIResponse> {
    // 我们需要使用POST请求url并设置header
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(format!("Bearer {}", key).as_str()).unwrap(),
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    tracing::debug!("will request: {}", url);
    let response_body = client.post(url).json(request).send().await?;

    let response_str = response_body.text().await?;
    tracing::debug!("response: {}", response_str);

    let response: OpenAIResponse = serde_json::from_str(&response_str)?;

    Ok(response)
}

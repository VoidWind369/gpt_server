use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::log_info;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub msg: String,
}

pub async fn new(message: String) -> String {
    use crate::config::get;
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    headers.insert("Authorization", HeaderValue::try_from(get().await.gpt_token).unwrap());

    let json = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": message,
            },
        ],
    });

    log_info!("发送json {}", get().await.gpt_url);
    let link = reqwest::Client::new().post(get().await.gpt_url)
        .headers(headers).json(&json).send().await;

    let link = match link {
        Ok(l) => l,
        Err(e) => { return format!("{}", e.to_string()); }
    };

    let status = link.status().as_u16();
    if status != 200 {
        return format!("错误代码 {status}");
    }
    let res = link.json::<serde_json::Value>().await.expect("value转换错误");
    log_info!("输出结果");
    res["choices"][0]["message"]["content"].as_str().expect("content输出错误").to_string()
}
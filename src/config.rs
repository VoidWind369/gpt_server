use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub server: u32,
    pub gpt_url: String,
    pub gpt_token: String,
}

pub async fn get() -> Config {
    let mut json = tokio::fs::File::open("config.json").await.expect("config文件不存在");
    let mut config_str = String::new();
    json.read_to_string(&mut config_str).await.expect("config文件无效");
    let config_str = include_str!("../config.json");
    serde_json::from_str::<Config>(&config_str).expect("config配置文件错误")
}
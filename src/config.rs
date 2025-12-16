use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub deepseek_api_key: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenv().ok();
        let deepseek_api_key =
            env::var("DEEPSEEK_API_KEY").context("DEEPSEEK_API_KEY not found")?;
        Ok(Self { deepseek_api_key })
    }

    pub fn validate(&self) -> Result<()> {
        if self.deepseek_api_key.is_empty() {
            anyhow::bail!("API密钥为空");
        }
        if !self.deepseek_api_key.starts_with("sk-") {
            anyhow::bail!("API密钥格式错误");
        }
        Ok(())
    }
}

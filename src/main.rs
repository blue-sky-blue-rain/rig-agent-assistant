mod agent;
mod config;
mod tools;
mod utils;

use anyhow::Result;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new()?;
    config.validate()?;

    println!("AI文件操作助手");
    println!("输入 'quit' 或 'exit' 退出\n");

    let file_agent = agent::FileAgent::new(&config).await?;

    loop {
        utils::print_prompt();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "quit" | "exit" | "q" => {
                println!("再见!");
                break;
            }
            _ => {
                if let Err(e) = file_agent.process_query(input).await {
                    println!("{} 错误: {}", "agent >".red(), e);
                }
            }
        }
    }

    Ok(())
}

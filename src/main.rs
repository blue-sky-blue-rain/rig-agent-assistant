mod agent;
mod config;
mod tools;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new()?;
    config.validate()?;

    println!("Rig Agent - 文件操作助手");
    println!("输入 'help' 查看帮助，'quit' 退出\n");

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
            "help" | "?" => {
                println!("\n可用命令:");
                println!("  创建文件: 创建 hello.txt 内容为 Hello World");
                println!("  读取文件: 读取 Cargo.toml 的内容");
                println!("  删除文件: 删除 test.txt");
                println!("  列出文件: 列出当前目录");
                println!("  执行命令: 运行 pwd 或 git init");
                println!();
            }
            "" => continue,
            _ => {
                if let Err(e) = file_agent.process_query(input).await {
                    println!("错误: {}", e);
                }
                println!();
            }
        }
    }

    Ok(())
}

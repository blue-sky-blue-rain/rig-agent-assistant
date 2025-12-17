use crate::config::Config;
use crate::tools::*;
use anyhow::Result;
use colored::*;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::deepseek::{self, DEEPSEEK_CHAT};

const AGENT_PREAMBLE: &str = "你是一个文件操作助手。你可以创建、删除、读取文件，也可以执行shell命令。\
                               优先使用专用的文件操作工具（create_file, delete_file, read_file），\
                               只有在这些工具无法完成任务时，才使用通用的run_command工具。\
                               每次执行操作前，系统都会向用户请求确认。\
                               请严格按照JSON格式返回结果，不要包含额外的文本或格式。\
                               如果用户有代码需求，默认使用英文代码中文注释。\
                               对于文件操作，请优先使用专门的工具而不是运行shell命令。";

pub struct FileAgent {
    file_agent: Agent<deepseek::CompletionModel>,
}

impl FileAgent {
    pub async fn new(config: &Config) -> Result<Self> {
        let client = deepseek::Client::new(&config.deepseek_api_key)
            .map_err(|e| anyhow::anyhow!("创建DeepSeek客户端失败: {}", e))?;

        let file_agent = client
            .agent(DEEPSEEK_CHAT)
            .preamble(AGENT_PREAMBLE)
            .temperature(0.1)
            .max_tokens(2000)
            .tool(CreateFile)
            .tool(ReadFile)
            .tool(DeleteFile)
            .tool(RunCommand)
            .build();

        Ok(Self { file_agent })
    }

    pub async fn process_query(&self, query: &str) -> Result<String> {
        println!("\n{} : {}", "agent >".green(), query);

        match self.file_agent.prompt(query).multi_turn(20).await {
            Ok(response) => {
                let result = response.to_string();
                println!("{} {}", "agent >".green(), result);
                Ok(result)
            }
            Err(e) => {
                println!("{} 错误: {}", "agent >".red(), e);
                Err(anyhow::anyhow!("{}", e))
            }
        }
    }
}

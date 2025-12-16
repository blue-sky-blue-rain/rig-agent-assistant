use crate::utils::*;
use colored::*;
use rig_derive::rig_tool;
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct CreateFileParams {
    filename: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeleteFileParams {
    filename: String,
}

#[derive(Debug, Deserialize)]
struct ReadFileParams {
    filename: String,
}

#[derive(Debug, Deserialize)]
struct ListFilesParams {
    detailed: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct RunCommandParams {
    command: String,
    args: Option<Vec<String>>,
}

#[rig_tool(
    name = "create_file",
    description = "创建文件",
    required(filename, content)
)]
fn create_file(params: CreateFileParams) -> Result<serde_json::Value, rig::tool::ToolError> {
    if !confirm_tool_call("create_file", &params.filename) {
        return Ok(json!({ "status": "cancelled" }));
    }

    if is_sensitive_path(&params.filename) && !confirm_dangerous("创建敏感文件") {
        return Ok(json!({ "status": "cancelled" }));
    }

    println!("{} 创建: {}", "agent command >".yellow(), params.filename);

    fs::write(&params.filename, &params.content)
        .map_err(|e| rig::tool::ToolError::ToolCallError(format!("失败: {}", e).into()))?;

    Ok(json!({ "status": "success" }))
}

#[rig_tool(name = "delete_file", description = "删除文件", required(filename))]
fn delete_file(params: DeleteFileParams) -> Result<serde_json::Value, rig::tool::ToolError> {
    if !confirm_tool_call("delete_file", &params.filename) {
        return Ok(json!({ "status": "cancelled" }));
    }

    if !Path::new(&params.filename).exists() {
        return Err(rig::tool::ToolError::ToolCallError(
            format!("文件不存在: {}", params.filename).into(),
        ));
    }

    if !confirm_dangerous("删除文件") {
        return Ok(json!({ "status": "cancelled" }));
    }

    println!("{} 删除: {}", "agent command >".yellow(), params.filename);

    fs::remove_file(&params.filename)
        .map_err(|e| rig::tool::ToolError::ToolCallError(format!("失败: {}", e).into()))?;

    Ok(json!({ "status": "success" }))
}

#[rig_tool(name = "read_file", description = "读取文件", required(filename))]
fn read_file(params: ReadFileParams) -> Result<serde_json::Value, rig::tool::ToolError> {
    if !confirm_tool_call("read_file", &params.filename) {
        return Ok(json!({ "status": "cancelled" }));
    }

    if !Path::new(&params.filename).exists() {
        return Err(rig::tool::ToolError::ToolCallError(
            format!("文件不存在: {}", params.filename).into(),
        ));
    }

    if is_sensitive_path(&params.filename) && !confirm_dangerous("读取敏感文件") {
        return Ok(json!({ "status": "cancelled" }));
    }

    println!("{} 读取: {}", "agent command >".yellow(), params.filename);

    let content = fs::read_to_string(&params.filename)
        .map_err(|e| rig::tool::ToolError::ToolCallError(format!("失败: {}", e).into()))?;

    println!("{}", content);

    Ok(json!({ "status": "success" }))
}

#[rig_tool(name = "list_files", description = "列出文件", optional(detailed))]
fn list_files(params: ListFilesParams) -> Result<serde_json::Value, rig::tool::ToolError> {
    let detailed = params.detailed.unwrap_or(false);

    if !confirm_tool_call("list_files", &format!("详细: {}", detailed)) {
        return Ok(json!({ "status": "cancelled" }));
    }

    println!("{} 列出文件", "agent command >".yellow());

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "dir"]).output()
    } else {
        let mut cmd = Command::new("ls");
        if detailed {
            cmd.arg("-al");
        }
        cmd.output()
    }
    .map_err(|e| rig::tool::ToolError::ToolCallError(format!("失败: {}", e).into()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    Ok(json!({ "status": "success" }))
}

#[rig_tool(
    name = "run_command",
    description = "执行命令",
    required(command),
    optional(args)
)]
fn run_command(params: RunCommandParams) -> Result<serde_json::Value, rig::tool::ToolError> {
    let args = params.args.unwrap_or_default();
    let full_command = format!("{} {}", params.command, args.join(" "));

    if !confirm_tool_call("run_command", &params.command) {
        return Ok(json!({ "status": "cancelled" }));
    }

    if is_dangerous_command(&full_command) && !confirm_dangerous("执行危险命令") {
        return Ok(json!({ "status": "cancelled" }));
    }

    println!("{} 执行: {}", "agent command >".yellow(), full_command);

    let output = if cfg!(target_os = "windows") {
        let mut cmd_args = vec!["/C".to_string(), params.command.clone()];
        cmd_args.extend(args.iter().cloned());
        Command::new("cmd").args(&cmd_args[1..]).output()
    } else {
        Command::new(&params.command).args(&args).output()
    }
    .map_err(|e| rig::tool::ToolError::ToolCallError(format!("失败: {}", e).into()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        println!("{}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }

    Ok(json!({
        "status": if output.status.success() { "success" } else { "error" },
    }))
}

use colored::*;
use rig_derive::rig_tool;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;

#[rig_tool(
    name = "create_file",
    description = "创建或修改文件",
    required(filename, content)
)]
fn create_file(
    filename: String,
    content: String,
) -> Result<serde_json::Value, rig::tool::ToolError> {
    if !crate::utils::confirm_action(&format!("创建/修改文件 {}", filename)) {
        println!("{} 操作已取消", "agent command >".yellow());
        return Ok(json!({
            "status": "cancelled",
            "action": "create_file",
            "filename": filename,
            "message": "用户取消了创建文件操作"
        }));
    }

    println!("{} 创建: {}", "agent command >".yellow(), filename);

    if let Some(parent) = Path::new(&filename).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                rig::tool::ToolError::ToolCallError(format!("创建目录失败: {}", e).into())
            })?;
        }
    }

    fs::write(&filename, &content)
        .map_err(|e| rig::tool::ToolError::ToolCallError(format!("写入文件失败: {}", e).into()))?;

    Ok(json!({
        "status": "success",
        "file": filename,
        "size": content.len()
    }))
}

#[rig_tool(name = "read_file", description = "读取文件内容", required(filename))]
fn read_file(filename: String) -> Result<serde_json::Value, rig::tool::ToolError> {
    if !crate::utils::confirm_action(&format!("读取文件 {}", filename)) {
        println!("{} 操作已取消", "agent command >".yellow());
        return Ok(json!({
            "status": "cancelled",
            "action": "read_file",
            "filename": filename,
            "message": "用户取消了读取文件操作"
        }));
    }

    println!("{} 读取: {}", "agent command >".yellow(), filename);

    let path = Path::new(&filename);
    if !path.exists() {
        return Err(rig::tool::ToolError::ToolCallError(
            format!("文件不存在: {}", filename).into(),
        ));
    }

    let content = fs::read_to_string(&filename)
        .map_err(|e| rig::tool::ToolError::ToolCallError(format!("读取文件失败: {}", e).into()))?;

    Ok(json!({
        "status": "success",
        "content": content,
        "filename": filename
    }))
}

#[rig_tool(name = "delete_file", description = "删除文件", required(filename))]
fn delete_file(filename: String) -> Result<serde_json::Value, rig::tool::ToolError> {
    if !crate::utils::confirm_action(&format!("删除文件 {}", filename)) {
        println!("{} 操作已取消", "agent command >".yellow());
        return Ok(json!({
            "status": "cancelled",
            "action": "delete_file",
            "filename": filename,
            "message": "用户取消了删除文件操作"
        }));
    }

    println!("{} 删除: {}", "agent command >".yellow(), filename);

    let path = Path::new(&filename);
    if !path.exists() {
        return Err(rig::tool::ToolError::ToolCallError(
            format!("文件不存在: {}", filename).into(),
        ));
    }

    fs::remove_file(&filename)
        .map_err(|e| rig::tool::ToolError::ToolCallError(format!("删除文件失败: {}", e).into()))?;

    Ok(json!({ "status": "success" }))
}

#[rig_tool(
    name = "run_command",
    description = "执行系统命令",
    required(command, args)
)]
fn run_command(
    command: String,
    args: Option<Vec<String>>,
) -> Result<serde_json::Value, rig::tool::ToolError> {
    let args = args.unwrap_or_default();
    let full_command = format!("{} {}", command, args.join(" "));

    if !crate::utils::confirm_action(&format!("执行命令 {}", full_command)) {
        println!("{} 操作已取消", "agent command >".yellow());
        return Ok(json!({
            "status": "cancelled",
            "action": "run_command",
            "command": full_command,
            "message": "用户取消了执行命令操作"
        }));
    }

    println!("{} 执行: {}", "agent command >".yellow(), full_command);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &full_command]).output()
    } else {
        Command::new("sh").arg("-c").arg(&full_command).output()
    }
    .map_err(|e| rig::tool::ToolError::ToolCallError(format!("执行命令失败: {}", e).into()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    Ok(json!({
        "status": if output.status.success() { "success" } else { "error" },
        "exit_code": output.status.code(),
        "stdout": stdout.to_string(),
        "stderr": stderr.to_string()
    }))
}

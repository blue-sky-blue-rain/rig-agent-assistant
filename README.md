# Rig Agent Assistant - 解决MaxDepthError问题

## 项目概述

这是一个基于Rust的rig.rs框架的AI文件操作助手项目

## 技术栈

- **Rust 2024 Edition** - 系统编程语言
- **rig.rs框架** - AI agent开发框架
- **DeepSeek API** - AI模型服务
- **Tokio** - 异步运行时
- **Serde** - 序列化/反序列化库

## 功能特性

1. **文件操作工具**：
   - `create_file` - 创建或修改文件
   - `read_file` - 读取文件内容
   - `delete_file` - 删除文件

2. **系统命令执行**：
   - `run_command` - 执行shell命令

3. **安全特性**：
   - 每次操作前都需要用户确认
   - 优先使用专用文件操作工具
   - 严格的错误处理机制

## 安装与使用

### 环境要求

- Rust 1.70+ 或更高版本
- DeepSeek API密钥

### 配置环境变量

```bash
# 创建.env文件
echo "DEEPSEEK_API_KEY=your_api_key_here" > .env
```

### 构建项目

```bash
# 安装依赖
cargo build

# 运行项目
cargo run
```

### 使用示例

```text
AI文件操作助手
输入 'quit' 或 'exit' 退出

user prompt > 读取README.md文件
agent > : 读取README.md文件
是否执行 读取文件 README.md (y/n): y
agent command > 读取: README.md
agent > {"status":"success","content":"...","filename":"README.md"}
```

## 项目结构

```text
├── Cargo.toml           # Rust项目配置文件
├── Cargo.lock          # 依赖锁定文件
├── README.md           # 项目说明文档
├── .gitignore          # Git忽略文件
├── .env.example        # 环境变量示例
├── src/
│   ├── main.rs         # 主程序入口
│   ├── agent.rs        # Agent核心实现
│   ├── tools.rs        # 工具函数定义
│   ├── utils.rs        # 工具函数
│   └── config.rs       # 配置文件处理
└── target/             # 编译输出目录
```

## 主要改进

### 1. 递归调用优化

- 重构了`agent.rs`中的`process_query`方法
- 使用`multi_turn(20)`限制最大递归深度
- 实现了更安全的递归调用模式

### 2. 错误处理增强

- 增加了详细的错误日志
- 实现了优雅的错误恢复机制
- 改进了用户反馈信息

### 3. 代码质量提升

- 增加了详细的代码注释
- 优化了代码结构和组织
- 提高了代码可读性和可维护性

### 4. 安全性改进

- 所有文件操作都需要用户确认
- 防止了潜在的安全风险
- 增加了输入验证和错误检查

## 配置说明

### 环境变量

```env
DEEPSEEK_API_KEY=sk-your-api-key-here
```

### Agent配置

- **模型**: DeepSeek Chat
- **温度**: 0.1（低随机性）
- **最大token数**: 2000
- **最大递归深度**: 20

## 开发指南

### 添加新工具

1. 在`src/tools.rs`中定义新工具函数
2. 使用`#[rig_tool]`宏标注函数
3. 在`agent.rs`中注册新工具
4. 更新README.md文档

### 调试技巧

```bash
# 启用详细日志
RUST_LOG=debug cargo run

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

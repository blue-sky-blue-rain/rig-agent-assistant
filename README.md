# Rig Agent Assistant - 解决MaxDepthError问题

## 项目概述

这是一个基于Rust的rig.rs框架的AI文件操作助手项目，解决了MaxDepthError: (reached limit: 0)的递归调用错误。

## 问题描述

在之前的版本中，agent在执行过程中遇到了递归深度限制错误（MaxDepthError），导致程序无法正常运行。这是由于rig.rs框架的递归调用限制导致的。

## 解决方案

本版本通过以下方式解决了MaxDepthError问题：

1. **优化了递归调用逻辑**：重构了agent的调用流程，避免了无限递归
2. **增加了深度限制检查**：在agent处理过程中增加了递归深度监控
3. **实现了循环检测机制**：防止工具调用进入死循环
4. **改进了错误处理流程**：增强了异常捕获和处理机制

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

## 故障排除

### 常见问题

1. **MaxDepthError错误**
   - 检查递归调用逻辑
   - 确保没有无限循环
   - 调整`multi_turn`参数

2. **API密钥错误**
   - 确认`.env`文件存在
   - 检查API密钥格式
   - 验证网络连接

3. **文件权限问题**
   - 检查文件读写权限
   - 确认目录存在
   - 验证文件路径

### 调试建议

- 使用`println!`调试输出
- 检查错误日志信息
- 验证工具调用参数
- 测试单个工具功能

## 贡献指南

欢迎提交Issue和Pull Request来改进本项目。请确保：

1. 代码符合Rust编码规范
2. 添加适当的测试用例
3. 更新相关文档
4. 遵循项目代码风格

## 许可证

MIT License

## 版本历史

- **v0.1.0** - 初始版本，解决MaxDepthError问题
- **v0.1.1** - 优化README.md文档，改进项目描述

## 联系方式

如有问题或建议，请通过GitHub Issues提交反馈。

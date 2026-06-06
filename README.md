<div align="center">

# 执行工作台 Enforcement Agents

**执行律师专属 AI 桌面工作台**

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![macOS](https://img.shields.io/badge/macOS-13.0+-black.svg)](https://www.apple.com/macos/)
[![Windows](https://img.shields.io/badge/Windows-10+-blue.svg)](https://www.microsoft.com/windows/)

基于 [MyAgents](https://github.com/hAcKklyc/MyAgents) (Apache-2.0)

</div>

---

## 这是什么

**执行工作台**是执行律师的 AI 桌面应用。把案件材料扔进来，AI 帮你整理、分析、调查、评估，输出能直接拿去执行的结果——查档指引、财产报告、申请书、评估建议。

不是通用 AI 工具，是从**收案到回款**每个环节都知道该做什么的执行搭档。

## 内置执行能力

### 6 个执行专用 Skills

| Skill | 做什么 | 输入 → 输出 |
|-------|--------|-------------|
| `file-sorter` | 材料智能整理 | 杂乱文件 → 规范目录 + 文件映射表 |
| `case-analysis` | 案件分析底稿 | 材料目录 → 案件基本信息 + 时间线 + 财产线索 + 缺口清单 |
| `execution-survey` | 被执行人财产调查 | 企业名称 → 企查查全维度扫描 + 财产调查报告 + 关联图谱 |
| `archive-inquiry` | 查档指引生成 | 注册地址 → 哪里查/带什么/几点开门的出发指南 |
| `wechat-flow-analysis` | 微信/银行流水分析 | 流水 Excel → 异常交易识别 + 限消违规检测 + 财产线索 |
| `case-evaluation` | 案件综合评估 | 三个上游报告 → 回款概率 + 工作量 + 收费建议 |

### 内置执行 Agent

打开应用即是执行律师模式。AI 懂执行程序、知道关键期限、会算冻结到期、能识别隐匿财产手段。

### 预配 MCP 工具

- **企查查 MCP** — 工商信息、司法风险、知识产权、关联企业穿透
- **小红书 MCP** — 搜索各地查档窗口信息、律师实地经验
- **Playwright MCP** — 浏览器自动化，下载法院文书、查询公开信息

## 快速体验

从 [Releases](https://github.com/394112866-spec/EnforcementAgents/releases) 下载安装包。

- **Windows**: 下载 `EnforcementAgents_Setup.exe`，双击安装
- **macOS**: 下载 `EnforcementAgents.dmg`，拖入 Applications

安装后打开"执行工作台"即可使用。无需安装任何开发工具。

## 支持的模型

内置 10+ 模型供应商，按需选择：

| 供应商 | 模型 |
|--------|------|
| Anthropic | Claude Opus 4.7, Sonnet 4.6, Haiku 4.5 |
| DeepSeek | DeepSeek Chat, Reasoner |
| Moonshot | Kimi K2.5, K2 Thinking |
| 智谱 AI | GLM 5, 4.7 |
| MiniMax | M2.5 |
| 火山方舟 | Doubao Seed 2.0 系列 |
| 硅基流动 | DeepSeek V3.2, GLM 4.7 等 |
| ZenMux | Claude 4.6, Gemini 3.1 等 |
| OpenRouter | GPT-5.2, Gemini 3 等 |

## 执行案件工作流

```
客户材料 → [file-sorter] → 规范化目录
              ↓
       [case-analysis] → 案件分析底稿
              ↓
  ┌──────────┼──────────┐
  ↓          ↓          ↓
[企业调查]  [查档指引]  [流水分析]
  ↓          ↓          ↓
  └──────────┼──────────┘
              ↓
       [case-evaluation] → 决策：接不接/怎么收
              ↓
        申请书生成 + 进度跟踪
```

## 面向执行律师的功能

- **执行 Agent** — 内置执行法律知识，懂查封冻结期限、被执行人财产调查清单、拒执罪构成要件
- **Chat + IM Bot 双通道** — 桌面端 + Telegram/钉钉/飞书，随时随地推进案件
- **定时提醒** — 冻结到期、查封到期、续冻续封提醒，不错过期限
- **工作区文件管理** — 每个案件独立目录，材料、分析、文书全在一个地方
- **全文搜索** — 本地搜索所有案件材料和历史分析，数据不出电脑
- **权限可控** — 行动/规划/自主三种模式，重要操作需确认

## 开发者

### 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 (Rust) |
| 前端 | React 19 + TypeScript + TailwindCSS |
| Agent Runtime | Node.js v24 + Claude Agent SDK |
| 构建 | GitHub Actions 云编译（本地不需 Rust） |

### 开发

```bash
git clone git@github.com:394112866-spec/EnforcementAgents.git
cd EnforcementAgents
npm install
npm run dev:web     # 前端开发
npm run server      # 后端开发
```

推送 tag 触发 CI 构建安装包：
```bash
git tag v0.1.0 && git push origin v0.1.0
```

### 致谢

基于 [MyAgents](https://github.com/hAcKklyc/MyAgents) 构建，感谢 MyAgents 团队的优秀工作。

## 许可证

[Apache License 2.0](LICENSE)

---

<a name="english"></a>

## English

MyAgents is an open-source desktop AI Agent that combines the powerful Agent capabilities of "Claude Code" with flexible IM Bot interaction — two-in-one, one-click install, zero barrier.

As of early 2026, AI capability is advancing rapidly — software developers were the first to become 10x or 100x more productive. 2026 is going to be the inaugural year of intelligence abundance. We hope MyAgents brings that power to everyone — students, content creators, educators, domain experts, product managers, anyone who *wants to make something*. We want MyAgents to be the soul of your computer, an amplifier for your taste and ideas, turning intent into impact.

### Quick Download
- Visit https://myagents.io to download the installer
- Mac version supports both Apple Silicon and Intel chips
- Windows version supports Windows 10 and above

### Core Capabilities

- **Zero-Barrier GUI** - Chrome-style multi-tab interface, each Tab runs an independent Agent for true parallel workflows
- **Multi-Agent Runtime (Lab)** - Beyond the built-in Claude Agent SDK, optionally pick **Claude Code CLI** or **OpenAI Codex CLI** as the external runtime — choose the engine that fits your task
- **Multi-Model Freedom** - Anthropic, DeepSeek, Moonshot, Zhipu, MiniMax, Volcengine, ZenMux, SiliconFlow, OpenRouter and 10+ providers, choose by need, control your cost
- **Skills System** - Codify your common workflows into reusable capability modules the Agent can invoke; built-in + custom
- **MCP Tool Integration** - Built-in MCP protocol support (STDIO/HTTP/SSE), connect external tools and data sources for unlimited extensibility
- **Custom Agents** - Configure dedicated prompts, tools, and models to build your own Agents
- **Agent + Channel Architecture** - Built-in Telegram / DingTalk adapters; more IM platforms (Feishu / WeChat / QQ etc.) plug in via the OpenClaw plugin ecosystem; multi-bot management, interactive permission approval, multimedia messages
- **Cron Task System** - Three scheduling modes — fixed interval / cron expression / one-shot — usable from Chat, AI tool calls, and IM bots
- **Embedded Terminal** - Interactive PTY in the right split panel (xterm.js + portable-pty), auto-rooted at the workspace, lifecycle bound to the Tab
- **Embedded Browser** - Tauri multi-Webview child view, AI-generated links and HTML files preview in one click, with persistent cookie store
- **Full-Text Search** - Local Tantivy + jieba search engine, sub-second retrieval over session history and workspace files — fully local, nothing uploaded
- **Self-Config CLI & MA Helper** - Built-in `myagents` command lets both AI and you manage app config from Bash; the MA Helper is the in-app support agent that diagnoses issues and configures tools for you
- **Smart Permissions** - Act / Plan / Auto modes for safety and control
- **Local Data, Continuous Evolution** - All conversations, files, and memories stay on your machine. API connects directly to providers. Your AI grows smarter the more you use it
- **Fully Open Source** - Apache-2.0 license, code fully open

### Supported Model Providers

| Provider | Models | Type |
|----------|--------|------|
| Anthropic | Claude Sonnet 4.6, Opus 4.6, Haiku 4.5 | Subscription/API |
| DeepSeek | DeepSeek Chat, Reasoner | API |
| Moonshot | Kimi K2.5, K2 Thinking, K2 | API |
| Zhipu AI | GLM 5, 4.7, 4.5 Air | API |
| MiniMax | M2.5, M2.5 Lightning, M2.1, M2.1 Lightning | API |
| Volcengine Coding Plan | Doubao Seed 2.0 Code, GLM 4.7, DeepSeek V3.2, Kimi K2.5 | API |
| Volcengine API | Doubao Seed 2.0 Pro, Code Preview, Lite | API |
| ZenMux | ZenMux Auto, Gemini 3.1 Pro, Claude 4.6, Doubao Seed 2.0 and more | API |
| SiliconFlow | Kimi K2.5, GLM 4.7, DeepSeek V3.2, Step 3.5 Flash and more | API |
| OpenRouter | GPT-5.2 Codex, GPT-5.2 Pro, Gemini 3 and more | API |

### System Requirements

#### End Users

- **macOS 13.0 (Ventura)** or later, Apple Silicon and Intel supported
- **Windows 10** or later

#### Developers

- macOS 13.0+ / Windows 10+ / Linux (Ubuntu 20.04+ AppImage/deb)
- [Node.js](https://nodejs.org) (v20+) — required at build time; Node.js v24 is bundled into production builds so end users install nothing
- [Rust](https://rustup.rs)

### Quick Start (Developers)

#### Installation

```bash
git clone https://github.com/hAcKlyc/MyAgents.git
cd MyAgents
./setup.sh
```

#### Build

```bash
# Debug build (with DevTools)
./build_dev.sh

# Production build (macOS DMG)
./build_macos.sh

# Production build (Windows NSIS)
# PowerShell: .\build_windows.ps1
```

### Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Framework | Tauri v2 (Rust) + multi-Webview |
| Frontend | React 19 + TypeScript + TailwindCSS + xterm.js |
| Agent Runtime | Node.js v24 + Claude Agent SDK (default) / Claude Code CLI / OpenAI Codex CLI / Gemini CLI |
| Community Ecosystem | Node.js (MCP servers / npm packages / `myagents` CLI — single runtime, bundled in app) |
| Communication | Rust HTTP/SSE Proxy (reqwest, unified localhost no-proxy) |
| Terminal | portable-pty (PTY process) + xterm.js (frontend renderer) |
| Search | Tantivy + tantivy-jieba (Chinese tokenizer) |
| Plugin | OpenClaw Plugin Bridge (separate Node.js process loading community Channel plugins) |

### Architecture

**Session-Centric multi-instance Sidecar architecture** — each session owns an isolated Agent process with strict 1:1 mapping; a multi-owner mechanism lets Tabs, scheduled tasks, and Agent Channels safely share the same Sidecar; the Rust proxy layer handles all traffic with zero CORS issues. **Single runtime**: Node.js v24 is bundled for everything (Sidecar / Plugin Bridge / MCP / community npm ecosystem / `myagents` CLI), plus Git for Windows is silently installed on Windows — users install nothing.

```
┌────────────────────────────────────────────────────────────────────┐
│                          Tauri Desktop App                         │
├────────────────────────────────────────────────────────────────────┤
│  React Frontend                                                    │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────────────────┐ │
│  │  Chat 1  │  │  Chat 2  │  │ Settings │  │   Agent Channels    │ │
│  │  Tab SSE │  │  Tab SSE │  │Global API│  │  TG / DT / Plugin   │ │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └──────────┬──────────┘ │
│       │             │             │                   │            │
├───────┼─────────────┼─────────────┼───────────────────┼────────────┤
│  Rust │             │             │                   │            │
│  ┌────┴─────────────┴─────┐  ┌────┴─────┐  ┌──────────┴──────────┐ │
│  │     SidecarManager     │  │  Global  │  │    ManagedAgents    │ │
│  │  Session:Sidecar 1:1   │  │ Sidecar  │  │    Plugin Bridge    │ │
│  │  Owner Tab/Cron/Agent  │  │          │  │     (OpenClaw)      │ │
│  └────┬─────────────┬─────┘  └──────────┘  └──────────┬──────────┘ │
│       ▼             ▼                                 ▼            │
│  Node.js Sidecar  (Claude Agent SDK / CC / Codex / Gemini CLI)     │
│    + MCP servers / community npm ecosystem / myagents CLI          │
└────────────────────────────────────────────────────────────────────┘
```

> For full details on session switching, owner lifecycle, and communication flow, see the [Architecture Documentation](specs/ARCHITECTURE.md).

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

### License

[Apache License 2.0](LICENSE)

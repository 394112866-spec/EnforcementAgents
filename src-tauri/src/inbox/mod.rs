// Session Inbox module (PRD 0.2.18)
//
// 实现 session 间异步消息通道——AI 通过 `myagents session send` CLI 命令
// 把 prompt 投递给另一个 session,target 处理后回应自动推回 caller。
//
// 关键设计(详见 specs/prd/prd_0.2.18_session_inbox.md):
//
//   - Fire-and-forget(无持久化、无 at-least-once 重试、无 correlationId 幂等)
//   - 跨 sidecar 路由:复用现有 cron→IM 的 `pending_*` 模式对称化为双向
//   - 投递路径:CLI → admin API → `cmd_inbox_deliver` → ensure_session_sidecar
//     → SessionSidecar.pending_inbox_messages push → HTTP POST 到 target
//     `/api/inbox/drain` → sidecar 包裹 <inbox-message> 注入 enqueueUserMessage
//   - Reply 路径:target turn-end → builtin SDK result / external persistTurnResult
//     → 同一 `cmd_inbox_deliver`(kind=Reply, reply_back=false)→ caller sidecar
//     `/api/inbox/drain` → 包裹 <inbox-reply> 注入

pub mod deliver;
pub mod types;

pub use deliver::cmd_inbox_deliver;
pub use types::{InboxMessageKind, PendingInboxMessage};

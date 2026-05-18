// Sanitize an inbox label before injecting into prompt context (PRD 0.2.18).
//
// **Pit-of-success 集中点**——所有 `<inbox-message from="...">` / `<inbox-reply
// from="...">` 注入前必经此 helper,避免每个注入点各自实现 escape + 截断。
//
// 必须 escape AND 截断(不是 OR):
//   - HTML escape 防 label 内含 `</inbox-message>` 等闭合标签破坏注入结构
//   - 80 字符上限防恶意长 label 占满 prompt 预算
//
// 注意:HTML escape **不能**挡住自然语言 prompt injection
// (例如 label 写 "IGNORE PRIOR INSTRUCTIONS...")——这是 inherent limitation,
// 缓解依赖 AI 自身 instruction following 健壮性,在 PRD 范畴外。
//
// 详见 PRD §4.4。

const HTML_ESCAPE_MAP: Record<string, string> = {
  '<': '&lt;',
  '>': '&gt;',
  '&': '&amp;',
  '"': '&quot;',
  "'": '&#39;',
};

const MAX_LABEL_LENGTH = 80;
const FALLBACK_LABEL = 'a session';

/**
 * Sanitize a raw label string for safe injection into prompt context.
 *
 * Returns `'a session'` if the input is null/undefined/empty after processing.
 *
 * **Order is slice → escape** (not escape → slice). Cross-review CC flagged that
 * the reverse order can mid-truncate an escape sequence (e.g. `&amp;` is 5 chars
 * → if escape grows length past the cap, the trailing chars become `&am` which
 * is an orphaned entity). Slicing raw input first guarantees escape output is
 * well-formed even if longer than the cap.
 */
export function sanitizeInboxLabel(raw: string | undefined | null): string {
  if (!raw) return FALLBACK_LABEL;

  const truncated = raw.slice(0, MAX_LABEL_LENGTH).trim();
  if (!truncated) return FALLBACK_LABEL;
  return truncated.replace(/[<>&"']/g, (c) => HTML_ESCAPE_MAP[c]!) || FALLBACK_LABEL;
}

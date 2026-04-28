// macOS NSEvent encodes function keys (arrows, F1-F35, page up/down, …)
// as Unicode private-use codepoints in `[NSEvent characters]` — see
// `NSFunctionKey` family in AppKit. WebKit *should* consume them in
// `keydown` (cursor move / scroll) and never dispatch `keypress` /
// `input`, but at input boundaries (cursor at index 0 pressing ←, or
// cursor at end pressing →) the keydown handler does nothing → the raw
// private-use codepoint falls through to the `input` event → ends up in
// the textarea/input value as a tofu glyph (no font carries U+F700-F74F).
//
// Tauri's WKWebView on macOS exposes the bug; WebView2 (Win) and
// webkit2gtk (Linux) don't, so this helper is effectively a macOS-only
// workaround that is a no-op on every other path.
//
// Apple reserves U+F700 through U+F8FF for "function key Unicodes",
// but in practice the keys we care about live in U+F700-F74F. We strip
// the whole F700-F74F band — wider than the canonical 4 arrows because
// page-up / home / end / fn-arrow can all leak the same way, and nothing
// legitimate puts those codepoints into user text.
//
// Built via `RegExp` constructor with explicit `\\u` escapes so the
// source stays printable in editors / git diffs — a literal U+F702
// inside a regex is a tofu in most fonts, indistinguishable from
// whitespace and very easy to corrupt during a copy-paste.
const MAC_FUNCTION_KEY_RANGE = new RegExp('[\\u{F700}-\\u{F74F}]', 'gu');

export function stripMacFunctionKeys(s: string): string {
  // Cheap fast-path so the common case (no leak) does not pay regex cost.
  // `search` returns -1 quickly when the input is ASCII / non-private-use,
  // which is ~all of the time.
  if (s.search(MAC_FUNCTION_KEY_RANGE) === -1) return s;
  return s.replace(MAC_FUNCTION_KEY_RANGE, '');
}

// Strip the leak from an `onChange` event AND force-sync the live DOM value.
//
// Why this exists in addition to `stripMacFunctionKeys`:
// boundary leaks (cursor at 0 pressing ←, or at end pressing →) insert a
// codepoint into the *DOM* value, but stripping it produces the same string
// React state already holds. React 19 bails out of identical-value
// `setState` via `Object.is`, which means **no commit happens** and the
// controlled `value` prop is never written back. The leaked tofu therefore
// stays in the DOM textarea forever, and each subsequent boundary press
// accumulates another one — visible to the user as a growing run of boxes,
// even though React state is clean.
//
// The fix is to write the clean value directly back to the DOM element when
// we detect a leak, bypassing the bailout. We also restore the caret —
// the leaked codepoint pushed it forward by `removed` chars; we need to
// pull it back so subsequent typing inserts in the right place.
//
// React's value tracker (the prototype-setter intercept it installs on
// HTMLInputElement / HTMLTextAreaElement) sees this assignment and updates
// its own "last known value" in lockstep, so the next genuine user input
// still fires `onChange` correctly.
export function sanitizeMacFunctionKeysFromEvent(
  e: { target: HTMLTextAreaElement | HTMLInputElement },
): string {
  const el = e.target;
  const dirty = el.value;
  const clean = stripMacFunctionKeys(dirty);
  if (clean === dirty) return clean;

  const start = el.selectionStart;
  const end = el.selectionEnd;
  el.value = clean;
  if (start !== null && end !== null) {
    // Adjust each endpoint by the count of leaked chars **before** that
    // endpoint in `dirty`, not by the total removed. The boundary-arrow
    // bug always inserts at the caret so a global subtract is enough,
    // but we keep this exact in case a leak ever lands further right
    // than the caret (would otherwise drag the caret left of where it
    // should be).
    const newStart = Math.max(
      0,
      Math.min(start - countLeakedBefore(dirty, start), clean.length),
    );
    const newEnd = Math.max(
      0,
      Math.min(end - countLeakedBefore(dirty, end), clean.length),
    );
    el.setSelectionRange(newStart, newEnd);
  }
  return clean;
}

function countLeakedBefore(dirty: string, pos: number): number {
  let count = 0;
  const limit = Math.min(pos, dirty.length);
  for (let i = 0; i < limit; i++) {
    const cp = dirty.charCodeAt(i);
    if (cp >= 0xf700 && cp <= 0xf74f) count++;
  }
  return count;
}

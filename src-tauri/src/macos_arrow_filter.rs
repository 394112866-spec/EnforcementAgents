//! Re-apply wry PR #769 (2022) at runtime.
//!
//! ## Why this exists
//!
//! On macOS Tauri/WKWebView, pressing left/right (and up/down) arrow keys
//! at a textarea boundary causes a Unicode private-use codepoint
//! (U+F700-F74F, AppKit's `NSFunctionKey` family) to be inserted into
//! the input value as a tofu glyph. The codepoint reaches the value via
//! AppKit's responder chain default — `NSResponder.keyDown:` →
//! `interpretKeyEvents:` → `insertText:` — bypassing WebCore's edit
//! pipeline entirely (no `beforeinput`, no `input` event), which means
//! a JS-side guard cannot catch it.
//!
//! wry shipped a fix for this in PR #769 (2022): an override of
//! `keyDown:` on wry's `WryWebView` subclass that simply does NOT forward
//! arrow keycodes (123-126) to super. WKWebView's own keyboard pipeline
//! still produces JS `KeyboardEvent` and moves the cursor — only the
//! leaky AppKit default is bypassed.
//!
//! That fix was lost during wry's objc2 migration and has NOT been
//! reintroduced in any released version up to wry 0.55.0 (2026-03-26).
//! Tracking: tauri-apps/wry#1175, tauri-apps/tauri#10194 — both OPEN.
//!
//! Since we have `tauri/unstable` enabled (needed for child webviews
//! used by the in-app browser), we hit the regression. Until upstream
//! relands the fix, we install our own `keyDown:` IMP at startup.

#![cfg(target_os = "macos")]

use std::sync::Once;

use objc2::ffi::{
    class_addMethod, class_getSuperclass, objc_msgSendSuper, objc_super,
};
use objc2::runtime::{AnyClass, AnyObject, Imp, Sel};
use objc2::{msg_send, sel};

static INSTALL: Once = Once::new();

pub fn install_arrow_key_filter() {
    INSTALL.call_once(|| unsafe {
        install_inner();
    });
}

unsafe fn install_inner() {
    let cls = match find_wry_webview_class() {
        Some(c) => c,
        None => {
            log::warn!("[macos_arrow_filter] wry WKWebView subclass not found; arrow-key filter not installed (leak workaround inactive)");
            return;
        }
    };

    let sel: Sel = sel!(keyDown:);

    // Method type encoding: void (id self, SEL _cmd, id event)
    let types = c"v@:@";
    let imp_fn: extern "C" fn(*mut AnyObject, Sel, *mut AnyObject) = key_down_filter;
    let imp: Imp = std::mem::transmute(imp_fn);

    let added = class_addMethod(
        (cls as *const AnyClass) as *mut AnyClass,
        sel,
        imp,
        types.as_ptr(),
    );

    if added.as_bool() {
        log::info!("[macos_arrow_filter] WryWebView keyDown: filter installed (re-applies wry PR #769)");
    } else {
        // `class_addMethod` only fails when the class itself already has a
        // method for this selector. Inherited `WKWebView` / `NSResponder`
        // implementations do not block adding our override.
        log::info!("[macos_arrow_filter] WryWebView already has a direct keyDown: method; assuming upstream fix landed, skipping");
    }
}

fn find_wry_webview_class() -> Option<&'static AnyClass> {
    // wry <= 0.54.2 used an explicit ObjC class name.
    if let Some(cls) = AnyClass::get(c"WryWebView") {
        return Some(cls);
    }

    // wry 0.54.4 removed `#[name = "WryWebView"]`. objc2 then generates a
    // version-suffixed class name such as
    // `wry::wkwebview::class::wry_web_view::WryWebView0.54.4`.
    let mut found = None;
    let mut matches = Vec::new();
    for cls in AnyClass::classes().iter().copied() {
        let name = cls.name().to_string_lossy();
        if is_wry_webview_class_name(&name) {
            matches.push(name.into_owned());
            found = Some(cls);
        }
    }

    if matches.len() > 1 {
        log::warn!(
            "[macos_arrow_filter] multiple WryWebView-like classes found: {}; using last registered match",
            matches.join(", ")
        );
    } else if let Some(name) = matches.first() {
        log::info!("[macos_arrow_filter] found generated WryWebView class: {name}");
    }

    found
}

fn is_wry_webview_class_name(name: &str) -> bool {
    let tail = name.rsplit("::").next().unwrap_or(name);
    if tail == "WryWebView" {
        return true;
    }
    let Some(version) = tail.strip_prefix("WryWebView") else {
        return false;
    };
    version
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_digit())
}

extern "C" fn key_down_filter(
    this: *mut AnyObject,
    _sel: Sel,
    event: *mut AnyObject,
) {
    unsafe {
        // Read the AppKit virtual keycode. NSEvent.keyCode is u16.
        let keycode: u16 = msg_send![&*event, keyCode];

        // macOS virtual keycodes for arrow keys:
        //   123 = kVK_LeftArrow
        //   124 = kVK_RightArrow
        //   125 = kVK_DownArrow
        //   126 = kVK_UpArrow
        //
        // For these four, we deliberately do NOT call super. WKWebView's
        // own keyboard pipeline (the one that produces JS KeyboardEvent
        // and drives caret movement) runs separately from
        // NSResponder.keyDown:, so cursor movement still works. What
        // we're skipping is AppKit's `interpretKeyEvents:` →
        // `insertText:` chain, which is what inserts the leaked
        // U+F700-F74F private-use codepoint at boundaries.
        if (123..=126).contains(&keycode) {
            return;
        }

        // For every other keycode, forward to the superclass's keyDown:
        // unchanged. The super of WryWebView is WKWebView, whose default
        // keyDown: handles non-arrow keys correctly.
        let cls: *const AnyClass = msg_send![this, class];
        let super_cls = class_getSuperclass(cls);
        let super_struct = objc_super {
            receiver: this,
            super_class: super_cls,
        };

        // objc_msgSendSuper has signature `id (struct objc_super *, SEL, ...)`
        // but we want void return on a single id arg. Cast to the right
        // signature before calling.
        type SuperKeyDown =
            extern "C" fn(*const objc_super, Sel, *mut AnyObject);
        let send_super: SuperKeyDown =
            std::mem::transmute(objc_msgSendSuper as *const ());
        send_super(&super_struct, sel!(keyDown:), event);
    }
}

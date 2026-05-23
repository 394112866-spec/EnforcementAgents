import { describe, expect, it } from 'vitest';

import { describeTerminalReason, shouldSurfaceTerminalReason } from './terminalReason';

describe('describeTerminalReason — banner suppression', () => {
  it('returns null for non-disruptive reasons (completed + any aborted_*)', () => {
    expect(describeTerminalReason('completed')).toBeNull();
    expect(describeTerminalReason('aborted_streaming')).toBeNull();
    expect(describeTerminalReason('aborted_tools')).toBeNull();
    // Prefix match by design — a future aborted_* value is auto-suppressed.
    expect(describeTerminalReason('aborted_init')).toBeNull();
  });

  it('returns null for missing / non-string input (no crash, no banner)', () => {
    expect(describeTerminalReason(undefined)).toBeNull();
    expect(describeTerminalReason(null)).toBeNull();
    expect(describeTerminalReason('')).toBeNull();
    expect(describeTerminalReason(42)).toBeNull();
  });
});

describe('describeTerminalReason — known reasons map to severity', () => {
  it('maps known error reasons with severity "error"', () => {
    expect(describeTerminalReason('prompt_too_long')).toMatchObject({ severity: 'error' });
    expect(describeTerminalReason('blocking_limit')).toMatchObject({ severity: 'error' });
    expect(describeTerminalReason('stop_hook_prevented')).toMatchObject({ severity: 'error' });
  });

  it('maps notice-level reasons with severity "notice"', () => {
    expect(describeTerminalReason('max_turns')).toMatchObject({ severity: 'notice' });
    expect(describeTerminalReason('rapid_refill_breaker')).toMatchObject({ severity: 'notice' });
  });

  it('gives every known reason a non-empty label + detail', () => {
    const info = describeTerminalReason('model_error');
    expect(info?.label).toBeTruthy();
    expect(info?.detail).toBeTruthy();
  });
});

describe('describeTerminalReason — unknown values degrade gracefully', () => {
  it('returns a notice placeholder embedding the raw value (no exhaustive-switch crash)', () => {
    const info = describeTerminalReason('some_future_reason');
    expect(info).not.toBeNull();
    expect(info?.severity).toBe('notice');
    expect(info?.label).toContain('some_future_reason');
  });
});

describe('shouldSurfaceTerminalReason', () => {
  it('is the inverse of "describe returned null"', () => {
    expect(shouldSurfaceTerminalReason('completed')).toBe(false);
    expect(shouldSurfaceTerminalReason('aborted_streaming')).toBe(false);
    expect(shouldSurfaceTerminalReason(undefined)).toBe(false);
    expect(shouldSurfaceTerminalReason('prompt_too_long')).toBe(true);
    expect(shouldSurfaceTerminalReason('some_future_reason')).toBe(true);
  });
});

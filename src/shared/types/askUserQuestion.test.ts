import { describe, it, expect } from 'vitest';
import { withQuestionTextAnswerKeys, type AskUserQuestion } from './askUserQuestion';

const q = (question: string, extra: Partial<AskUserQuestion> = {}): AskUserQuestion => ({
  question,
  header: 'H',
  options: [
    { label: 'A', description: '' },
    { label: 'B', description: '' },
  ],
  multiSelect: false,
  ...extra,
});

describe('withQuestionTextAnswerKeys', () => {
  // The bug: SDK 0.3.158's builtin AskUserQuestion tool looks each answer up by
  // question TEXT (answers[question.question]); the renderer keyed by index, so
  // the model was told "The user did not answer the questions."
  it('aliases index-keyed builtin answers by question text (0.2.119→0.3.158 regression)', () => {
    const questions = [q('请你定一件事：要不要配图？')];
    const out = withQuestionTextAnswerKeys(questions, { '0': '纯文字纸墨版' });
    // SDK binary reads answers[question.text] — must be present now.
    expect(out['请你定一件事：要不要配图？']).toBe('纯文字纸墨版');
    // Original index key is preserved so the Codex runtime still matches.
    expect(out['0']).toBe('纯文字纸墨版');
  });

  it('aliases every question in a multi-question form', () => {
    const questions = [q('Q1?'), q('Q2?')];
    const out = withQuestionTextAnswerKeys(questions, { '0': 'A', '1': 'B' });
    expect(out['Q1?']).toBe('A');
    expect(out['Q2?']).toBe('B');
  });

  it('keeps id-keyed (Codex) answers and adds the text alias for the SDK binary', () => {
    const questions = [q('Pick one?', { id: 'codex-123' })];
    const out = withQuestionTextAnswerKeys(questions, { 'codex-123': 'A' });
    expect(out['codex-123']).toBe('A'); // Codex reads this
    expect(out['Pick one?']).toBe('A'); // SDK binary reads this
  });

  it('preserves comma-joined multi-select values verbatim', () => {
    const questions = [q('Which features?', { multiSelect: true })];
    const out = withQuestionTextAnswerKeys(questions, { '0': 'A,B' });
    expect(out['Which features?']).toBe('A,B');
  });

  it('does not invent an answer for a skipped optional question', () => {
    const questions = [q('Required?'), q('Optional?', { required: false })];
    // Renderer omits the optional question's key when nothing was selected.
    const out = withQuestionTextAnswerKeys(questions, { '0': 'A' });
    expect(out['Required?']).toBe('A');
    expect('Optional?' in out).toBe(false);
  });

  it('returns answers unchanged when questions are missing', () => {
    expect(withQuestionTextAnswerKeys(undefined, { '0': 'A' })).toEqual({ '0': 'A' });
  });

  it('is idempotent when answers are already keyed by text', () => {
    const questions = [q('Q1?')];
    const out = withQuestionTextAnswerKeys(questions, { 'Q1?': 'A' });
    expect(out).toEqual({ 'Q1?': 'A' });
  });
});

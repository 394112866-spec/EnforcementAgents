/**
 * CalendarWidget — 执行日历
 * Shows current month with deadline indicators.
 */
import { memo, useMemo } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';

interface Deadline {
  date: string;  // YYYY-MM-DD
  event: string;
  caseName: string;
  urgent: boolean;
}

interface Props {
  deadlines: Deadline[];
}

const WEEKDAYS = ['一', '二', '三', '四', '五', '六', '日'];

export default memo(function CalendarWidget({ deadlines }: Props) {
  const today = useMemo(() => new Date(), []);
  const [year, month] = [today.getFullYear(), today.getMonth()];

  const deadlineMap = useMemo(() => {
    const map = new Map<string, Deadline[]>();
    deadlines.forEach((d) => {
      const list = map.get(d.date) || [];
      list.push(d);
      map.set(d.date, list);
    });
    return map;
  }, [deadlines]);

  const days = useMemo(() => {
    const firstDay = new Date(year, month, 1);
    const lastDay = new Date(year, month + 1, 0);
    const startPad = (firstDay.getDay() + 6) % 7; // Monday start
    const result: (number | null)[] = [];
    for (let i = 0; i < startPad; i++) result.push(null);
    for (let d = 1; d <= lastDay.getDate(); d++) result.push(d);
    return result;
  }, [year, month]);

  const todayStr = useMemo(() => {
    const y = today.getFullYear();
    const m = String(today.getMonth() + 1).padStart(2, '0');
    const d = String(today.getDate()).padStart(2, '0');
    return `${y}-${m}-${d}`;
  }, [today]);

  const monthLabel = `${year}年${month + 1}月`;

  return (
    <div className="rounded-xl border border-[var(--line)] bg-[var(--paper-elevated)] p-4">
      {/* Header */}
      <div className="mb-3 flex items-center justify-between">
        <h3 className="text-[13px] font-semibold text-[var(--ink)]">执行日历</h3>
        <div className="flex items-center gap-2">
          <span className="text-[13px] text-[var(--ink-secondary)]">{monthLabel}</span>
          <div className="flex gap-0.5">
            <button type="button" className="rounded p-0.5 text-[var(--ink-subtle)] hover:text-[var(--ink)]">
              <ChevronLeft className="h-3.5 w-3.5" />
            </button>
            <button type="button" className="rounded p-0.5 text-[var(--ink-subtle)] hover:text-[var(--ink)]">
              <ChevronRight className="h-3.5 w-3.5" />
            </button>
          </div>
        </div>
      </div>

      {/* Weekday headers */}
      <div className="mb-1 grid grid-cols-7 text-center">
        {WEEKDAYS.map((w) => (
          <div key={w} className="text-[10px] font-medium text-[var(--ink-subtle)]">
            {w}
          </div>
        ))}
      </div>

      {/* Day grid */}
      <div className="grid grid-cols-7 text-center">
        {days.map((day, i) => {
          if (day === null) {
            return <div key={`empty-${i}`} className="aspect-square" />;
          }
          const dateStr = `${year}-${String(month + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}`;
          const hasDeadline = deadlineMap.has(dateStr);
          const isToday = dateStr === todayStr;
          const items = deadlineMap.get(dateStr);

          return (
            <div
              key={day}
              className="relative flex aspect-square items-center justify-center"
              title={items?.map((d) => d.event).join('\n')}
            >
              <span
                className={`inline-flex h-6 w-6 items-center justify-center rounded-full text-[12px] ${
                  isToday
                    ? 'bg-[var(--accent)] font-semibold text-white'
                    : hasDeadline
                      ? 'font-medium text-[var(--ink)]'
                      : 'text-[var(--ink-muted)]'
                }`}
              >
                {day}
              </span>
              {hasDeadline && !isToday && (
                <span
                  className={`absolute bottom-0 left-1/2 h-1 w-1 -translate-x-1/2 rounded-full ${
                    items!.some((d) => d.urgent) ? 'bg-red-500' : 'bg-amber-400'
                  }`}
                />
              )}
              {hasDeadline && isToday && (
                <span className="absolute bottom-0 left-1/2 h-1 w-1 -translate-x-1/2 rounded-full bg-white" />
              )}
            </div>
          );
        })}
      </div>

      {/* Upcoming deadlines summary */}
      {deadlines.length > 0 && (
        <div className="mt-3 border-t border-[var(--line-subtle)] pt-3">
          <div className="space-y-1.5">
            {deadlines.slice(0, 3).map((d, i) => (
              <div key={i} className="flex items-center gap-2 text-[12px]">
                <span
                  className={`inline-block h-1.5 w-1.5 shrink-0 rounded-full ${
                    d.urgent ? 'bg-red-500' : 'bg-amber-400'
                  }`}
                />
                <span className="text-[var(--ink-muted)]">{d.date.slice(5)}</span>
                <span className="truncate text-[var(--ink)]">{d.event}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
});

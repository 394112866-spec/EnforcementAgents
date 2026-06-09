/**
 * EnforcementDashboard — 执行仪表盘
 * Layout: stats row → right panel (calendar + deadlines) | left panel (recent + skills)
 */
import { memo, useMemo } from 'react';
import { Scale, Clock, Banknote, AlertTriangle, ChevronRight, FileText, Search, ArrowRight } from 'lucide-react';
import CalendarWidget from './CalendarWidget';

interface DashboardProps {
  onOpenCase?: (caseName: string) => void;
}

const STATS = [
  { label: '冻结到期', value: '3', icon: Clock, color: 'text-red-500', bg: 'bg-red-50' },
  { label: '待办事项', value: '7', icon: AlertTriangle, color: 'text-amber-600', bg: 'bg-amber-50' },
];

const RECENT_ACTIVITIES = [
  { type: '流水分析', case: '张三申请执行李四案', time: '2小时前', icon: FileText },
  { type: '财产调查', case: '王五申请执行赵六案', time: '昨天', icon: Search },
  { type: '案件分析', case: 'A公司申请执行B公司案', time: '昨天', icon: Scale },
];

// Deadlines for both calendar display and the deadlines list
const DEADLINES = [
  { date: '2026-06-15', event: '冻结银行存款到期', caseName: '张三案', urgent: true },
  { date: '2026-06-20', event: '查封不动产到期', caseName: '王五案', urgent: true },
  { date: '2026-07-01', event: '恢复执行审查', caseName: 'A公司案', urgent: false },
  { date: '2026-07-10', event: '冻结股权到期', caseName: '张三案', urgent: true },
  { date: '2026-06-25', event: '拍卖公告截止', caseName: '王五案', urgent: false },
];

export default memo(function EnforcementDashboard({ onOpenCase }: DashboardProps) {
  // Sort deadlines by date, urgent first
  const sortedDeadlines = useMemo(
    () => [...DEADLINES].sort((a, b) => a.date.localeCompare(b.date)),
    [],
  );

  return (
    <div className="flex w-full max-w-[800px] flex-col gap-4 px-4">
      {/* Brand header — compact */}
      <div className="text-center">
        <h1 className="text-[1.75rem] font-semibold tracking-tight text-[var(--ink)]">
          执行工作台
        </h1>
        <p className="mt-0.5 text-[13px] text-[var(--ink-muted)]">
          从收案到回款，每一步都有人帮你
        </p>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-2 gap-3">
        {STATS.map((stat) => (
          <div
            key={stat.label}
            className="flex items-center gap-3 rounded-xl border border-[var(--line)] bg-[var(--paper-elevated)] p-3 transition-shadow hover:shadow-sm"
          >
            <div className={`flex h-9 w-9 items-center justify-center rounded-lg ${stat.bg}`}>
              <stat.icon className={`h-[18px] w-[18px] ${stat.color}`} />
            </div>
            <div>
              <div className="text-[11px] text-[var(--ink-muted)]">{stat.label}</div>
              <div className="text-lg font-semibold text-[var(--ink)]">{stat.value}</div>
            </div>
          </div>
        ))}
      </div>

      {/* Main content: Left (recent + skills) | Right (calendar + deadlines) */}
      <div className="grid grid-cols-5 gap-4">
        {/* Left: 3/5 — Recent activity + Quick skills */}
        <div className="col-span-3 flex flex-col gap-4">
          {/* Recent activity */}
          <div className="rounded-xl border border-[var(--line)] bg-[var(--paper-elevated)] p-4">
            <div className="mb-3 flex items-center justify-between">
              <h3 className="text-[13px] font-semibold text-[var(--ink)]">最近分析</h3>
              <button
                type="button"
                className="flex items-center gap-1 text-[11px] text-[var(--ink-muted)] transition-colors hover:text-[var(--accent)]"
              >
                全部 <ChevronRight className="h-3 w-3" />
              </button>
            </div>
            <div className="space-y-1">
              {RECENT_ACTIVITIES.map((a, i) => (
                <button
                  key={i}
                  type="button"
                  className="flex w-full items-center gap-3 rounded-lg px-2 py-2 text-left transition-colors hover:bg-[var(--hover-bg)]"
                  onClick={() => onOpenCase?.(a.case)}
                >
                  <a.icon className="h-4 w-4 shrink-0 text-[var(--ink-muted)]" />
                  <div className="min-w-0 flex-1">
                    <div className="truncate text-[13px] text-[var(--ink)]">{a.case}</div>
                    <div className="text-[11px] text-[var(--ink-subtle)]">
                      {a.type} · {a.time}
                    </div>
                  </div>
                  <ArrowRight className="h-3 w-3 shrink-0 text-[var(--ink-subtle)]" />
                </button>
              ))}
            </div>
          </div>

          {/* Quick skills */}
          <div className="flex flex-wrap gap-1.5">
            {[
              { label: '整理材料', icon: '📂' },
              { label: '分析案情', icon: '📋' },
              { label: '财产调查', icon: '🔍' },
              { label: '流水分析', icon: '💳' },
              { label: '查档指引', icon: '📍' },
              { label: '案件评估', icon: '📊' },
            ].map((s) => (
              <button
                key={s.label}
                type="button"
                className="rounded-full border border-[var(--line)] bg-[var(--paper-elevated)] px-2.5 py-1 text-[12px] text-[var(--ink-muted)] transition-colors hover:border-[var(--line-strong)] hover:text-[var(--ink)]"
              >
                {s.icon} {s.label}
              </button>
            ))}
          </div>
        </div>

        {/* Right: 2/5 — Calendar + Deadlines */}
        <div className="col-span-2 flex flex-col gap-4">
          {/* Calendar */}
          <CalendarWidget deadlines={DEADLINES} />

          {/* Upcoming deadlines */}
          <div className="rounded-xl border border-[var(--line)] bg-[var(--paper-elevated)] p-4">
            <div className="mb-3 flex items-center justify-between">
              <h3 className="text-[13px] font-semibold text-[var(--ink)]">近期到期</h3>
              <button
                type="button"
                className="flex items-center gap-1 text-[11px] text-[var(--ink-muted)] transition-colors hover:text-[var(--accent)]"
              >
                全部 <ChevronRight className="h-3 w-3" />
              </button>
            </div>
            <div className="space-y-1.5">
              {sortedDeadlines.slice(0, 5).map((d, i) => (
                <div key={i} className="flex items-center gap-2 rounded-lg px-2 py-1.5">
                  <span
                    className={`h-2 w-2 shrink-0 rounded-full ${
                      d.urgent ? 'bg-red-500' : 'bg-amber-400'
                    }`}
                  />
                  <div className="min-w-0 flex-1">
                    <div className="truncate text-[13px] text-[var(--ink)]">{d.event}</div>
                    <div className="text-[11px] text-[var(--ink-subtle)]">
                      {d.caseName} · {d.date}
                    </div>
                  </div>
                  {d.urgent && (
                    <span className="shrink-0 rounded-full bg-red-50 px-1.5 py-0.5 text-[10px] font-medium text-red-600">
                      紧急
                    </span>
                  )}
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
});

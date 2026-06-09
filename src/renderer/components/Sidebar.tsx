/**
 * Sidebar — 执行工作台左侧导航栏
 * Persistent across all views (Launcher / Chat / Settings / TaskCenter).
 */
import { memo } from 'react';
import { LayoutDashboard, FolderOpen, Sparkles, Wrench } from 'lucide-react';

interface NavItem {
  id: string;
  label: string;
  icon: typeof LayoutDashboard;
  event: string;
}

const NAV_ITEMS: NavItem[] = [
  { id: 'dashboard', label: '仪表盘', icon: LayoutDashboard, event: 'NAV_DASHBOARD' },
  { id: 'cases', label: '案件管理', icon: FolderOpen, event: 'NAV_CASES' },
  { id: 'skills', label: 'Skills', icon: Sparkles, event: 'NAV_SKILLS' },
  { id: 'mcp', label: 'MCP 工具', icon: Wrench, event: 'NAV_MCP' },
];

interface SidebarProps {
  activeNav?: string;
  onNavigate?: (id: string) => void;
}

export default memo(function Sidebar({ activeNav = 'dashboard', onNavigate }: SidebarProps) {
  return (
    <nav className="flex w-[56px] flex-shrink-0 flex-col items-center gap-1 border-r border-[var(--line)] bg-[var(--paper-elevated)] py-3">
      {/* App icon */}
      <div className="mb-2 flex h-8 w-8 items-center justify-center rounded-lg bg-[var(--accent)] text-[13px] font-bold text-white">
        执
      </div>

      {/* Nav items */}
      {NAV_ITEMS.map((item) => {
        const isActive = activeNav === item.id;
        return (
          <button
            key={item.id}
            type="button"
            onClick={() => onNavigate?.(item.id)}
            className={`group flex flex-col items-center gap-0.5 rounded-lg px-1.5 py-1.5 transition-colors ${
              isActive
                ? 'text-[var(--accent)]'
                : 'text-[var(--ink-subtle)] hover:text-[var(--ink-muted)]'
            }`}
            title={item.label}
          >
            <div
              className={`flex h-8 w-8 items-center justify-center rounded-lg transition-colors ${
                isActive
                  ? 'bg-[var(--accent-warm-subtle)]'
                  : 'group-hover:bg-[var(--hover-bg)]'
              }`}
            >
              <item.icon className="h-[18px] w-[18px]" />
            </div>
            <span className="text-[9px] leading-tight">{item.label}</span>
          </button>
        );
      })}
    </nav>
  );
});

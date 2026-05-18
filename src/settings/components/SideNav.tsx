import { cn } from "@shared/lib/cn";

export type SectionKey = "general" | "hotkey" | "provider" | "appearance" | "about";

interface Props {
  active: SectionKey;
  onChange: (key: SectionKey) => void;
  sections: { key: SectionKey; label: string }[];
}

export function SideNav({ active, onChange, sections }: Props) {
  return (
    <nav className="flex h-full w-56 shrink-0 flex-col gap-1 border-r border-[var(--color-border)] bg-[var(--color-bg)]/40 p-3">
      <div className="px-2 py-2 text-[10px] uppercase tracking-wider text-[var(--color-text-dim)]">
        Sections
      </div>
      {sections.map((s) => (
        <button
          key={s.key}
          onClick={() => onChange(s.key)}
          className={cn(
            "rounded-lg px-3 py-2 text-left text-sm transition",
            active === s.key
              ? "bg-[var(--color-bg-soft)] text-[var(--color-text)]"
              : "text-[var(--color-text-muted)] hover:bg-[var(--color-bg-soft)]/60 hover:text-[var(--color-text)]"
          )}
        >
          {s.label}
        </button>
      ))}
    </nav>
  );
}

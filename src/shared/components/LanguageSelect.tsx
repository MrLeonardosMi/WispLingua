import * as Select from "@radix-ui/react-select";
import { Check, ChevronDown, Search } from "lucide-react";
import { useMemo, useState } from "react";
import Fuse from "fuse.js";
import { cn } from "@shared/lib/cn";
import type { LanguageOption } from "@shared/types";

interface Props {
  value: string;
  onChange: (code: string) => void;
  languages: LanguageOption[];
  className?: string;
  includeAuto?: boolean;
  placeholder?: string;
}

export function LanguageSelect({
  value,
  onChange,
  languages,
  className,
  includeAuto = false,
  placeholder = "Select language",
}: Props) {
  const [query, setQuery] = useState("");

  const options = useMemo(
    () => (includeAuto ? languages : languages.filter((l) => l.code !== "auto")),
    [languages, includeAuto]
  );

  const fuse = useMemo(
    () => new Fuse(options, { keys: ["name", "native_name", "code"], threshold: 0.35 }),
    [options]
  );

  const filtered = useMemo(() => {
    if (!query.trim()) return options;
    return fuse.search(query).map((r) => r.item);
  }, [options, fuse, query]);

  const current = options.find((l) => l.code === value);

  return (
    <Select.Root value={value} onValueChange={onChange}>
      <Select.Trigger
        className={cn(
          "no-drag inline-flex h-9 items-center gap-1.5 rounded-lg border border-[var(--color-border)] bg-[var(--color-bg-soft)] px-3 text-sm font-medium text-[var(--color-text)] outline-none transition hover:border-[var(--color-border-strong)] focus-visible:ring-2 focus-visible:ring-[var(--color-accent)]/70",
          className
        )}
      >
        <Select.Value placeholder={placeholder}>
          {current ? (
            <span className="flex items-center gap-1.5">
              <span className="font-mono text-[11px] text-[var(--color-text-muted)] uppercase">
                {current.code}
              </span>
              <span>{current.native_name}</span>
            </span>
          ) : (
            placeholder
          )}
        </Select.Value>
        <Select.Icon>
          <ChevronDown size={14} className="text-[var(--color-text-muted)]" />
        </Select.Icon>
      </Select.Trigger>

      <Select.Portal>
        <Select.Content
          position="popper"
          sideOffset={6}
          className="z-50 overflow-hidden rounded-xl border border-[var(--color-border)] bg-[var(--color-bg-elevated)] shadow-[var(--shadow-popup)]"
        >
          <div className="flex items-center gap-2 border-b border-[var(--color-border)] px-3 py-2">
            <Search size={14} className="text-[var(--color-text-muted)]" />
            <input
              autoFocus
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search…"
              className="text-selectable w-44 bg-transparent text-sm text-[var(--color-text)] outline-none placeholder:text-[var(--color-text-dim)]"
            />
          </div>
          <Select.Viewport className="scroll-thin max-h-72 p-1">
            {filtered.map((lang) => (
              <Select.Item
                key={lang.code}
                value={lang.code}
                className="relative flex cursor-default items-center gap-2 rounded-md px-2.5 py-1.5 text-sm text-[var(--color-text)] outline-none data-[highlighted]:bg-[var(--color-bg-soft)] data-[state=checked]:text-[var(--color-accent)]"
              >
                <span className="w-9 font-mono text-[10px] uppercase text-[var(--color-text-dim)]">
                  {lang.code}
                </span>
                <Select.ItemText>{lang.native_name}</Select.ItemText>
                <span className="ml-auto text-xs text-[var(--color-text-dim)]">
                  {lang.name !== lang.native_name ? lang.name : ""}
                </span>
                <Select.ItemIndicator className="absolute right-2">
                  <Check size={13} className="text-[var(--color-accent)]" />
                </Select.ItemIndicator>
              </Select.Item>
            ))}
            {filtered.length === 0 && (
              <div className="px-2 py-3 text-center text-xs text-[var(--color-text-dim)]">
                No matches
              </div>
            )}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
  );
}

import type { ReactNode } from "react";
import { cn } from "@shared/lib/cn";

interface Props {
  label: string;
  description?: string;
  children: ReactNode;
  align?: "row" | "stack";
  className?: string;
}

export function Field({ label, description, children, align = "row", className }: Props) {
  return (
    <div
      className={cn(
        "flex gap-6 py-4",
        align === "row" ? "items-center justify-between" : "flex-col gap-2",
        className
      )}
    >
      <div className={cn("flex flex-col gap-1", align === "row" ? "max-w-md" : "")}>
        <div className="text-sm font-medium text-[var(--color-text)]">{label}</div>
        {description && (
          <div className="text-xs leading-relaxed text-[var(--color-text-muted)]">
            {description}
          </div>
        )}
      </div>
      <div className={align === "row" ? "shrink-0" : ""}>{children}</div>
    </div>
  );
}

export function FieldGroup({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="rounded-2xl border border-[var(--color-border)] bg-[var(--color-bg-elevated)]/40 px-5 py-2">
      <h2 className="border-b border-[var(--color-border)]/60 py-3 text-[11px] uppercase tracking-wider text-[var(--color-text-muted)]">
        {title}
      </h2>
      <div className="divide-y divide-[var(--color-border)]/40">{children}</div>
    </section>
  );
}

import { forwardRef, type ButtonHTMLAttributes, type ReactNode } from "react";
import { cn } from "@shared/lib/cn";

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
  label: string;
  active?: boolean;
  children: ReactNode;
}

export const IconButton = forwardRef<HTMLButtonElement, Props>(function IconButton(
  { label, active, className, children, ...rest },
  ref
) {
  return (
    <button
      ref={ref}
      title={label}
      aria-label={label}
      className={cn(
        "no-drag inline-flex h-7 w-7 items-center justify-center rounded-md text-[var(--color-text-muted)] transition",
        "hover:bg-[var(--color-bg-soft)] hover:text-[var(--color-text)]",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--color-accent)]/70",
        active && "bg-[var(--color-accent-soft)]/30 text-[var(--color-accent)]",
        className
      )}
      {...rest}
    >
      {children}
    </button>
  );
});

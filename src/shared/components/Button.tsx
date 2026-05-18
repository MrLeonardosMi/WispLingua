import { forwardRef, type ButtonHTMLAttributes } from "react";
import { cn } from "@shared/lib/cn";

type Variant = "primary" | "secondary" | "ghost" | "danger";
type Size = "sm" | "md" | "lg";

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
  size?: Size;
}

const variants: Record<Variant, string> = {
  primary:
    "bg-[var(--color-accent)] text-[oklch(0.12_0.01_270)] hover:brightness-110 active:brightness-95",
  secondary:
    "bg-[var(--color-bg-soft)] text-[var(--color-text)] hover:bg-[var(--color-border)]",
  ghost:
    "bg-transparent text-[var(--color-text-muted)] hover:bg-[var(--color-bg-soft)] hover:text-[var(--color-text)]",
  danger:
    "bg-[var(--color-danger)] text-white hover:brightness-110",
};

const sizes: Record<Size, string> = {
  sm: "h-7 px-2.5 text-xs gap-1.5 rounded-md",
  md: "h-9 px-3.5 text-sm gap-2 rounded-lg",
  lg: "h-11 px-5 text-base gap-2.5 rounded-xl",
};

export const Button = forwardRef<HTMLButtonElement, Props>(function Button(
  { variant = "secondary", size = "md", className, ...rest },
  ref
) {
  return (
    <button
      ref={ref}
      className={cn(
        "inline-flex items-center justify-center font-medium transition select-none outline-none focus-visible:ring-2 focus-visible:ring-[var(--color-accent)]/70 disabled:opacity-40 disabled:pointer-events-none no-drag",
        variants[variant],
        sizes[size],
        className
      )}
      {...rest}
    />
  );
});

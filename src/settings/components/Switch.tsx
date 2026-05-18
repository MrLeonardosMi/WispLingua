import * as Sw from "@radix-ui/react-switch";
import { cn } from "@shared/lib/cn";

interface Props {
  checked: boolean;
  onChange: (v: boolean) => void;
  className?: string;
}

export function Switch({ checked, onChange, className }: Props) {
  return (
    <Sw.Root
      checked={checked}
      onCheckedChange={onChange}
      className={cn(
        "relative h-6 w-10 rounded-full bg-[var(--color-bg-soft)] outline-none transition",
        "data-[state=checked]:bg-[var(--color-accent)]",
        "focus-visible:ring-2 focus-visible:ring-[var(--color-accent)]/70",
        className
      )}
    >
      <Sw.Thumb className="block h-5 w-5 translate-x-0.5 rounded-full bg-white shadow transition data-[state=checked]:translate-x-[18px]" />
    </Sw.Root>
  );
}

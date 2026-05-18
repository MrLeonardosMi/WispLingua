import { FieldGroup, Field } from "../components/Field";

export function AboutSection() {
  return (
    <div className="flex flex-col gap-6">
      <FieldGroup title="About">
        <Field label="Version" description="WispLingua 0.1.0">
          <span className="font-mono text-xs text-[var(--color-text-muted)]">0.1.0</span>
        </Field>
        <Field label="Source" description="Open source — pull requests welcome.">
          <a
            href="https://github.com/"
            target="_blank"
            rel="noreferrer"
            className="text-sm text-[var(--color-accent)] hover:underline"
          >
            github →
          </a>
        </Field>
        <Field label="License" description="MIT">
          <span className="font-mono text-xs text-[var(--color-text-muted)]">MIT</span>
        </Field>
      </FieldGroup>

      <FieldGroup title="Tips">
        <div className="space-y-3 py-4 text-sm text-[var(--color-text-muted)]">
          <p>• Hotkey doesn't fire? Make sure the app has accessibility / input monitoring permissions on macOS, or runs unprivileged on Wayland.</p>
          <p>• On Linux X11 it works out of the box. Wayland: needs the global-shortcut portal — install <code className="font-mono text-[12px] text-[var(--color-text)]">xdg-desktop-portal</code> matching your compositor.</p>
          <p>• Double Ctrl+C is the most reliable trigger across apps. If a target app blocks Ctrl+C, switch to a dedicated combo like Ctrl+Alt+T.</p>
        </div>
      </FieldGroup>
    </div>
  );
}

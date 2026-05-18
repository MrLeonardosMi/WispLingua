import { Field, FieldGroup } from "../components/Field";
import { Switch } from "../components/Switch";
import { LanguageSelect } from "@shared/components/LanguageSelect";
import { DEFAULT_LANGUAGES } from "@shared/lib/languages";
import { useConfig } from "@shared/store/config";
import { api } from "@shared/api/invoke";

const STYLES: { value: string; label: string; description: string }[] = [
  { value: "Native", label: "Native", description: "Idiomatic, sounds like a native speaker" },
  { value: "Formal", label: "Formal", description: "Professional and polished" },
  { value: "Casual", label: "Casual", description: "Conversational tone" },
  { value: "Technical", label: "Technical", description: "Preserves jargon and structure" },
  { value: "Literal", label: "Literal", description: "Word-for-word fidelity" },
];

export function GeneralSection() {
  const { config, update } = useConfig();
  if (!config) return null;

  return (
    <div className="flex flex-col gap-6">
      <FieldGroup title="Translation">
        <Field label="Target language" description="Translate selected text into this language by default.">
          <LanguageSelect
            value={config.target_language}
            onChange={(code) => update({ target_language: code })}
            languages={DEFAULT_LANGUAGES}
          />
        </Field>
        <Field
          label="Fallback target"
          description="Used when the source text is already in your target language (e.g. you copied English while target is English — translate to this instead)."
        >
          <LanguageSelect
            value={config.fallback_target_language}
            onChange={(code) => update({ fallback_target_language: code })}
            languages={DEFAULT_LANGUAGES}
          />
        </Field>
        <Field label="Auto-detect source" description="Let the engine detect the source language. Off = always pass 'auto' to the model.">
          <Switch checked={config.detect_source} onChange={(v) => update({ detect_source: v })} />
        </Field>
        <Field label="Style" description="How the translation should read.">
          <select
            value={config.style}
            onChange={(e) => update({ style: e.target.value as typeof config.style })}
            className="no-drag h-9 rounded-lg border border-[var(--color-border)] bg-[var(--color-bg-soft)] px-3 text-sm outline-none transition hover:border-[var(--color-border-strong)] focus-visible:ring-2 focus-visible:ring-[var(--color-accent)]/70"
          >
            {STYLES.map((s) => (
              <option key={s.value} value={s.value}>
                {s.label} — {s.description}
              </option>
            ))}
          </select>
        </Field>
      </FieldGroup>

      <FieldGroup title="System">
        <Field
          label="Start with system"
          description="Launch WispLingua at login. Useful since the app lives in the tray."
        >
          <Switch
            checked={config.autostart}
            onChange={async (v) => {
              await update({ autostart: v });
              await api.applyAutostart();
            }}
          />
        </Field>
        <Field
          label="Preserve clipboard"
          description="Restore the clipboard contents after capturing the selection. Off = the selected text stays in your clipboard."
        >
          <Switch
            checked={config.preserve_clipboard}
            onChange={(v) => update({ preserve_clipboard: v })}
          />
        </Field>
      </FieldGroup>
    </div>
  );
}

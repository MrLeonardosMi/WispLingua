import { useState } from "react";
import { Field, FieldGroup } from "../components/Field";
import { Button } from "@shared/components/Button";
import { useConfig } from "@shared/store/config";
import { useHotkeyRecorder, formatTrigger } from "@shared/hooks/useHotkeyRecorder";
import { api } from "@shared/api/invoke";
import type { HotkeyTrigger } from "@shared/types";

const PRESET_TRIGGERS: { label: string; trigger: HotkeyTrigger; hint: string }[] = [
  {
    label: "Double Ctrl+C",
    trigger: { mode: "Combo", combo: "Ctrl+C", presses: 2 },
    hint: "DeepL-style. Press Ctrl+C twice quickly.",
  },
  {
    label: "Triple Ctrl",
    trigger: { mode: "Modifier", modifier: "Ctrl", presses: 3 },
    hint: "Tap Ctrl three times. No copy required (uses cursor selection only on supported apps).",
  },
  {
    label: "Ctrl+Alt+T",
    trigger: { mode: "Combo", combo: "Ctrl+Alt+T", presses: 1 },
    hint: "Classic single combo.",
  },
  {
    label: "Ctrl+Shift+Space",
    trigger: { mode: "Combo", combo: "Ctrl+Shift+Space", presses: 1 },
    hint: "Quick and rarely conflicts.",
  },
];

export function HotkeySection() {
  const { config, update } = useConfig();
  const recorder = useHotkeyRecorder();
  const [presses, setPresses] = useState<number>(config?.hotkey.trigger.presses ?? 2);

  if (!config) return null;

  async function applyTrigger(trigger: HotkeyTrigger) {
    await update({ hotkey: { ...config!.hotkey, trigger } });
    await api.applyHotkey();
  }

  async function applyChordWindow(ms: number) {
    await update({ hotkey: { ...config!.hotkey, chord_window_ms: ms } });
    await api.applyHotkey();
  }

  async function commitRecording() {
    const trig = recorder.build(presses);
    if (trig) await applyTrigger(trig);
  }

  return (
    <div className="flex flex-col gap-6">
      <FieldGroup title="Trigger">
        <Field label="Current shortcut" description="Press this combo to translate the current selection.">
          <div className="font-mono text-sm rounded-lg border border-[var(--color-border)] bg-[var(--color-bg-soft)] px-3 py-1.5">
            {formatTrigger(config.hotkey.trigger)}
          </div>
        </Field>

        <Field
          label="Record new"
          description="Click record, then press your desired keys. For chord shortcuts (multi-press), set the count below."
          align="stack"
        >
          <div className="flex flex-wrap items-center gap-3">
            <Button
              variant={recorder.recording ? "danger" : "primary"}
              onClick={() => (recorder.recording ? recorder.stop() : recorder.start())}
            >
              {recorder.recording ? "Recording… press keys" : "Record shortcut"}
            </Button>
            {(recorder.modifiers.length > 0 || recorder.key) && (
              <div className="font-mono text-sm rounded-lg border border-[var(--color-accent)]/50 bg-[var(--color-accent-soft)]/15 px-3 py-1.5">
                {[...recorder.modifiers, recorder.key].filter(Boolean).join("+")}
                {presses > 1 ? ` ×${presses}` : ""}
              </div>
            )}
            <div className="flex items-center gap-2 text-sm text-[var(--color-text-muted)]">
              Presses:
              <select
                value={presses}
                onChange={(e) => setPresses(Number(e.target.value))}
                className="no-drag h-8 rounded-md border border-[var(--color-border)] bg-[var(--color-bg-soft)] px-2 text-sm outline-none"
              >
                <option value={1}>Single</option>
                <option value={2}>Double</option>
                <option value={3}>Triple</option>
              </select>
            </div>
            {(recorder.modifiers.length > 0 || recorder.key) && (
              <Button variant="primary" onClick={commitRecording}>
                Apply
              </Button>
            )}
          </div>
        </Field>

        <Field
          label="Chord window"
          description="Maximum delay (ms) between consecutive presses when using a double or triple chord."
        >
          <div className="flex items-center gap-2">
            <input
              type="range"
              min={150}
              max={700}
              step={25}
              value={config.hotkey.chord_window_ms}
              onChange={(e) => applyChordWindow(Number(e.target.value))}
              className="no-drag accent-[var(--color-accent)]"
            />
            <span className="w-12 text-right font-mono text-xs text-[var(--color-text-muted)]">
              {config.hotkey.chord_window_ms}ms
            </span>
          </div>
        </Field>
      </FieldGroup>

      <FieldGroup title="Presets">
        {PRESET_TRIGGERS.map((p) => (
          <Field key={p.label} label={p.label} description={p.hint}>
            <Button variant="secondary" onClick={() => applyTrigger(p.trigger)}>
              Use
            </Button>
          </Field>
        ))}
      </FieldGroup>
    </div>
  );
}

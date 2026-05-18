import { Field, FieldGroup } from "../components/Field";
import { Switch } from "../components/Switch";
import { useConfig } from "@shared/store/config";

export function AppearanceSection() {
  const { config, update } = useConfig();
  if (!config) return null;

  return (
    <div className="flex flex-col gap-6">
      <FieldGroup title="Popup">
        <Field label="Width" description="Width of the popup window in pixels.">
          <div className="flex items-center gap-2">
            <input
              type="range"
              min={320}
              max={620}
              step={10}
              value={config.popup.width}
              onChange={(e) =>
                update({ popup: { ...config.popup, width: Number(e.target.value) } })
              }
              className="no-drag accent-[var(--color-accent)]"
            />
            <span className="w-12 text-right font-mono text-xs text-[var(--color-text-muted)]">
              {config.popup.width}px
            </span>
          </div>
        </Field>
        <Field label="Max height" description="Maximum popup height before scrolling.">
          <div className="flex items-center gap-2">
            <input
              type="range"
              min={160}
              max={520}
              step={10}
              value={config.popup.max_height}
              onChange={(e) =>
                update({ popup: { ...config.popup, max_height: Number(e.target.value) } })
              }
              className="no-drag accent-[var(--color-accent)]"
            />
            <span className="w-12 text-right font-mono text-xs text-[var(--color-text-muted)]">
              {config.popup.max_height}px
            </span>
          </div>
        </Field>
        <Field
          label="Follow cursor"
          description="Open the popup near the mouse cursor. Off = open near screen center."
        >
          <Switch
            checked={config.popup.follow_cursor}
            onChange={(v) => update({ popup: { ...config.popup, follow_cursor: v } })}
          />
        </Field>
        <Field
          label="Prefer below cursor"
          description="When there's room, open below the cursor. Otherwise open above."
        >
          <Switch
            checked={config.popup.prefer_below}
            onChange={(v) => update({ popup: { ...config.popup, prefer_below: v } })}
          />
        </Field>
        <Field label="Text size" description="Scale the translation text.">
          <div className="flex items-center gap-2">
            <input
              type="range"
              min={0.8}
              max={1.4}
              step={0.05}
              value={config.popup.font_scale}
              onChange={(e) =>
                update({ popup: { ...config.popup, font_scale: Number(e.target.value) } })
              }
              className="no-drag accent-[var(--color-accent)]"
            />
            <span className="w-12 text-right font-mono text-xs text-[var(--color-text-muted)]">
              {Math.round(config.popup.font_scale * 100)}%
            </span>
          </div>
        </Field>
      </FieldGroup>
    </div>
  );
}

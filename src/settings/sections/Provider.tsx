import { useEffect, useMemo, useState } from "react";
import { Eye, EyeOff, ExternalLink, Loader2, RefreshCw, Search } from "lucide-react";
import { Field, FieldGroup } from "../components/Field";
import { Button } from "@shared/components/Button";
import { useConfig } from "@shared/store/config";
import { api } from "@shared/api/invoke";
import type { ModelInfo } from "@shared/types";
import Fuse from "fuse.js";

export function ProviderSection() {
  const { config, update } = useConfig();
  const [apiKey, setApiKey] = useState("");
  const [hasKey, setHasKey] = useState(false);
  const [showKey, setShowKey] = useState(false);
  const [savingKey, setSavingKey] = useState(false);
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [modelsLoading, setModelsLoading] = useState(false);
  const [modelError, setModelError] = useState<string | null>(null);
  const [testResult, setTestResult] = useState<string | null>(null);
  const [testing, setTesting] = useState(false);
  const [query, setQuery] = useState("");

  useEffect(() => {
    api.getApiKey().then((k) => {
      if (k) {
        setHasKey(true);
        setApiKey("");
      } else {
        setHasKey(false);
      }
    });
  }, []);

  async function saveKey() {
    if (!apiKey.trim()) return;
    setSavingKey(true);
    try {
      await api.setApiKey(apiKey.trim());
      setHasKey(true);
      setApiKey("");
    } finally {
      setSavingKey(false);
    }
  }

  async function clearKey() {
    await api.clearApiKey();
    setHasKey(false);
    setApiKey("");
  }

  async function loadModels() {
    setModelsLoading(true);
    setModelError(null);
    try {
      const list = await api.fetchModels();
      setModels(list);
    } catch (e) {
      setModelError(String(e));
    } finally {
      setModelsLoading(false);
    }
  }

  async function runTest() {
    setTesting(true);
    setTestResult(null);
    try {
      const out = await api.testProvider();
      setTestResult(out);
    } catch (e) {
      setTestResult(`Error: ${e}`);
    } finally {
      setTesting(false);
    }
  }

  const fuse = useMemo(
    () => new Fuse(models, { keys: ["id", "name", "description"], threshold: 0.35 }),
    [models]
  );

  const filtered = useMemo(() => {
    if (!query.trim()) return models;
    return fuse.search(query).map((r) => r.item);
  }, [fuse, models, query]);

  if (!config) return null;
  const or = config.provider.openrouter;

  return (
    <div className="flex flex-col gap-6">
      <FieldGroup title="OpenRouter">
        <Field
          label="API key"
          description="Stored in the OS keychain (Credential Manager / Keychain / Secret Service). Never in plain text."
          align="stack"
        >
          {hasKey && !apiKey ? (
            <div className="flex flex-wrap items-center gap-3">
              <div className="font-mono text-sm text-[var(--color-text-muted)] rounded-lg border border-[var(--color-border)] bg-[var(--color-bg-soft)] px-3 py-1.5">
                ••••••••••••••••
              </div>
              <Button variant="secondary" onClick={() => setHasKey(false)}>
                Replace
              </Button>
              <Button variant="danger" onClick={clearKey}>
                Remove
              </Button>
            </div>
          ) : (
            <div className="flex flex-wrap items-center gap-3">
              <div className="relative flex-1 min-w-[280px]">
                <input
                  type={showKey ? "text" : "password"}
                  value={apiKey}
                  onChange={(e) => setApiKey(e.target.value)}
                  placeholder="sk-or-v1-..."
                  className="text-selectable no-drag h-9 w-full rounded-lg border border-[var(--color-border)] bg-[var(--color-bg-soft)] px-3 pr-9 font-mono text-sm outline-none transition focus-visible:ring-2 focus-visible:ring-[var(--color-accent)]/70"
                />
                <button
                  onClick={() => setShowKey(!showKey)}
                  className="absolute right-2 top-1/2 -translate-y-1/2 text-[var(--color-text-dim)] hover:text-[var(--color-text)]"
                >
                  {showKey ? <EyeOff size={14} /> : <Eye size={14} />}
                </button>
              </div>
              <Button variant="primary" onClick={saveKey} disabled={!apiKey.trim() || savingKey}>
                {savingKey ? "Saving…" : "Save"}
              </Button>
              <a
                href="https://openrouter.ai/keys"
                target="_blank"
                rel="noreferrer"
                className="inline-flex items-center gap-1 text-xs text-[var(--color-accent)] hover:underline"
              >
                Get a key <ExternalLink size={12} />
              </a>
            </div>
          )}
        </Field>

        <Field
          label="Model"
          description={`Currently: ${or.model || "—"}`}
          align="stack"
        >
          <div className="flex flex-wrap items-center gap-3">
            <Button variant="secondary" onClick={loadModels} disabled={modelsLoading}>
              {modelsLoading ? (
                <>
                  <Loader2 size={13} className="animate-spin" /> Loading…
                </>
              ) : (
                <>
                  <RefreshCw size={13} /> Refresh model list
                </>
              )}
            </Button>
            {modelError && (
              <div className="text-xs text-[var(--color-danger)]">{modelError}</div>
            )}
          </div>

          {models.length > 0 && (
            <div className="mt-3 flex flex-col gap-2">
              <div className="relative">
                <Search
                  size={14}
                  className="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-[var(--color-text-dim)]"
                />
                <input
                  value={query}
                  onChange={(e) => setQuery(e.target.value)}
                  placeholder="Search models…"
                  className="text-selectable no-drag h-9 w-full rounded-lg border border-[var(--color-border)] bg-[var(--color-bg-soft)] pl-8 pr-3 text-sm outline-none transition focus-visible:ring-2 focus-visible:ring-[var(--color-accent)]/70"
                />
              </div>
              <div className="scroll-thin max-h-72 overflow-y-auto rounded-xl border border-[var(--color-border)] bg-[var(--color-bg)]/40">
                {filtered.slice(0, 200).map((m) => {
                  const active = or.model === m.id;
                  return (
                    <button
                      key={m.id}
                      onClick={() => update({ provider: { ...config.provider, openrouter: { ...or, model: m.id } } })}
                      className={`w-full px-3 py-2 text-left transition ${
                        active
                          ? "bg-[var(--color-accent-soft)]/20"
                          : "hover:bg-[var(--color-bg-soft)]"
                      }`}
                    >
                      <div className="flex items-center justify-between gap-3">
                        <div className="min-w-0 flex-1">
                          <div className={`truncate text-sm font-medium ${active ? "text-[var(--color-accent)]" : ""}`}>
                            {m.name}
                          </div>
                          <div className="truncate font-mono text-[11px] text-[var(--color-text-dim)]">
                            {m.id}
                          </div>
                        </div>
                        <div className="shrink-0 text-right text-[11px] text-[var(--color-text-muted)]">
                          {m.context_length ? `${(m.context_length / 1000).toFixed(0)}k ctx` : ""}
                          {m.pricing_prompt != null && (
                            <div className="font-mono">
                              ${(m.pricing_prompt * 1_000_000).toFixed(2)}/Mt
                            </div>
                          )}
                        </div>
                      </div>
                    </button>
                  );
                })}
                {filtered.length === 0 && (
                  <div className="px-3 py-6 text-center text-xs text-[var(--color-text-dim)]">
                    No models match “{query}”
                  </div>
                )}
              </div>
            </div>
          )}
        </Field>

        <Field
          label="Temperature"
          description="Higher = more creative; lower = more literal."
        >
          <div className="flex items-center gap-2">
            <input
              type="range"
              min={0}
              max={1.5}
              step={0.05}
              value={or.temperature}
              onChange={(e) =>
                update({ provider: { ...config.provider, openrouter: { ...or, temperature: Number(e.target.value) } } })
              }
              className="no-drag accent-[var(--color-accent)]"
            />
            <span className="w-12 text-right font-mono text-xs text-[var(--color-text-muted)]">
              {or.temperature.toFixed(2)}
            </span>
          </div>
        </Field>

        <Field label="Test" description="Run a quick translation through the active provider.">
          <div className="flex items-center gap-3">
            <Button variant="primary" onClick={runTest} disabled={testing}>
              {testing ? (
                <>
                  <Loader2 size={13} className="animate-spin" /> Testing…
                </>
              ) : (
                "Send test"
              )}
            </Button>
            {testResult && (
              <div className="rounded-lg border border-[var(--color-border)] bg-[var(--color-bg-soft)] px-3 py-1.5 text-xs text-[var(--color-text)]">
                {testResult}
              </div>
            )}
          </div>
        </Field>
      </FieldGroup>
    </div>
  );
}

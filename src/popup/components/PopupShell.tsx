import { useEffect, useState } from "react";
import { ArrowLeftRight, Copy, CornerDownLeft, Pin, PinOff, RefreshCw, Settings, Sparkles, X } from "lucide-react";
import { Button } from "@shared/components/Button";
import { IconButton } from "@shared/components/IconButton";
import { LanguageSelect } from "@shared/components/LanguageSelect";
import { DEFAULT_LANGUAGES } from "@shared/lib/languages";
import { api } from "@shared/api/invoke";
import { useTranslation } from "../store";

export function PopupShell() {
  const {
    pinned,
    loading,
    done,
    error,
    sourceText,
    sourceLanguage,
    targetLanguage,
    model,
    output,
    elapsedMs,
    close,
    togglePin,
    copyOutput,
    retranslate,
    setTargetLanguage,
    swapLanguages,
  } = useTranslation();

  const [copied, setCopied] = useState(false);

  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if (e.key === "Escape" && !pinned) {
        close();
      }
      if ((e.ctrlKey || e.metaKey) && e.key === "c" && output) {
        copyOutput();
        setCopied(true);
      }
    }
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [pinned, close, copyOutput, output]);

  useEffect(() => {
    if (!copied) return;
    const t = setTimeout(() => setCopied(false), 1300);
    return () => clearTimeout(t);
  }, [copied]);

  async function doCopy() {
    await copyOutput();
    setCopied(true);
  }

  return (
    <div
      className="flex h-full w-full flex-col overflow-hidden rounded-2xl border border-[var(--color-border)] bg-[var(--color-bg-elevated)]/95 backdrop-blur-xl shadow-[var(--shadow-popup)]"
      onMouseDown={(e) => e.stopPropagation()}
    >
      <header className="drag flex items-center gap-2 border-b border-[var(--color-border)]/70 px-3 py-2">
        <div className="flex items-center gap-1.5 text-[11px] uppercase tracking-wider text-[var(--color-text-dim)]">
          <Sparkles size={12} className="text-[var(--color-accent)]" />
          WispLingua
        </div>
        <div className="ml-2 flex items-center gap-1.5">
          <span className="rounded-md bg-[var(--color-bg-soft)] px-1.5 py-0.5 font-mono text-[10px] uppercase text-[var(--color-text-muted)]">
            {sourceLanguage ?? "auto"}
          </span>
          <IconButton label="Swap languages" onClick={swapLanguages} className="h-6 w-6">
            <ArrowLeftRight size={11} />
          </IconButton>
          <LanguageSelect
            value={targetLanguage}
            onChange={setTargetLanguage}
            languages={DEFAULT_LANGUAGES}
            className="h-7 px-2 py-0 text-xs"
          />
        </div>
        <div className="ml-auto flex items-center gap-0.5">
          <IconButton label="Retranslate" onClick={() => retranslate()} disabled={loading || !sourceText}>
            <RefreshCw size={13} className={loading ? "animate-spin" : ""} />
          </IconButton>
          <IconButton label={copied ? "Copied!" : "Copy"} onClick={doCopy} disabled={!output}>
            <Copy size={13} />
          </IconButton>
          <IconButton label={pinned ? "Unpin" : "Pin"} active={pinned} onClick={togglePin}>
            {pinned ? <PinOff size={13} /> : <Pin size={13} />}
          </IconButton>
          <IconButton label="Settings" onClick={() => api.openSettings()}>
            <Settings size={13} />
          </IconButton>
          <IconButton label="Close" onClick={close}>
            <X size={13} />
          </IconButton>
        </div>
      </header>

      <section className="scroll-thin flex-1 overflow-y-auto px-3.5 py-2.5">
        {sourceText && (
          <div className="mb-2 text-[12px] leading-snug text-[var(--color-text-dim)] line-clamp-3 italic">
            “{sourceText}”
          </div>
        )}
        {error ? (
          <div className="rounded-lg border border-[var(--color-danger)]/40 bg-[var(--color-danger)]/10 px-3 py-2 text-sm text-[var(--color-danger)]">
            {error}
          </div>
        ) : (
          <div
            className={`text-selectable whitespace-pre-wrap break-words text-[14px] leading-relaxed text-[var(--color-text)] ${
              loading && !output ? "shimmer-bg rounded-md min-h-6" : ""
            } ${loading && output ? "caret-blink" : ""}`}
          >
            {output}
          </div>
        )}
      </section>

      <footer className="flex items-center justify-between gap-3 border-t border-[var(--color-border)]/70 px-3 py-1.5">
        <div className="flex min-w-0 items-center gap-2 text-[10px] text-[var(--color-text-dim)]">
          <span className="truncate font-mono">{model || "—"}</span>
          <span className="shrink-0">
            {done && elapsedMs != null && !error
              ? `${(elapsedMs / 1000).toFixed(1)}s`
              : loading
              ? "translating…"
              : ""}
          </span>
        </div>
        <Button
          size="sm"
          variant="primary"
          onClick={async () => {
            if (!output || loading) return;
            await api.replaceSelection(output);
          }}
          disabled={!done || !!error || !output}
          title="Replace the originally selected text with this translation"
        >
          <CornerDownLeft size={11} />
          Replace
        </Button>
      </footer>
    </div>
  );
}

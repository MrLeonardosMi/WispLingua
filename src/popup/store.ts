import { create } from "zustand";
import { api } from "@shared/api/invoke";
import { events } from "@shared/api/events";
import type { UnlistenFn } from "@tauri-apps/api/event";

interface PopupState {
  visible: boolean;
  pinned: boolean;
  loading: boolean;
  done: boolean;
  error: string | null;
  requestId: string | null;
  sourceText: string;
  sourceLanguage: string | null;
  targetLanguage: string;
  model: string;
  output: string;
  elapsedMs: number | null;
  unlisteners: UnlistenFn[];

  init: () => Promise<void>;
  dispose: () => void;
  togglePin: () => Promise<void>;
  close: () => Promise<void>;
  copyOutput: () => Promise<void>;
  retranslate: (target?: string) => Promise<void>;
  setTargetLanguage: (code: string) => Promise<void>;
  swapLanguages: () => Promise<void>;
}

export const useTranslation = create<PopupState>((set, get) => ({
  visible: false,
  pinned: false,
  loading: false,
  done: false,
  error: null,
  requestId: null,
  sourceText: "",
  sourceLanguage: null,
  targetLanguage: "en",
  model: "",
  output: "",
  elapsedMs: null,
  unlisteners: [],

  async init() {
    const off1 = await events.onTranslationStart((p) => {
      set({
        visible: true,
        loading: true,
        done: false,
        error: null,
        requestId: p.request_id,
        sourceText: p.source_text,
        sourceLanguage: p.source_language,
        targetLanguage: p.target_language,
        model: p.model,
        output: "",
        elapsedMs: null,
      });
    });
    const off2 = await events.onTranslationChunk((p) => {
      if (get().requestId !== p.request_id) return;
      set((s) => ({ output: s.output + p.delta }));
    });
    const off3 = await events.onTranslationDone((p) => {
      if (get().requestId !== p.request_id) return;
      set({ loading: false, done: true, output: p.total, elapsedMs: p.elapsed_ms });
    });
    const off4 = await events.onTranslationError((p) => {
      if (get().requestId !== p.request_id) return;
      set({ loading: false, done: true, error: p.message });
    });
    set({ unlisteners: [off1, off2, off3, off4] });

    try {
      const cfg = await api.getConfig();
      set({ targetLanguage: cfg.target_language });
    } catch {}
  },

  dispose() {
    get().unlisteners.forEach((u) => u());
    set({ unlisteners: [] });
  },

  async togglePin() {
    const next = !get().pinned;
    await api.pinPopup(next);
    set({ pinned: next });
  },

  async close() {
    set({ visible: false });
    await new Promise((r) => setTimeout(r, 150));
    await api.closePopup();
    set({
      pinned: false,
      output: "",
      sourceText: "",
      error: null,
      done: false,
      requestId: null,
    });
  },

  async copyOutput() {
    const text = get().output;
    if (!text) return;
    try {
      await navigator.clipboard.writeText(text);
    } catch {}
  },

  async retranslate(target?: string) {
    const { sourceText } = get();
    if (!sourceText.trim()) return;
    if (target) {
      const cfg = await api.getConfig();
      await api.saveConfig({ ...cfg, target_language: target });
      set({ targetLanguage: target });
    }
    await api.translateText(sourceText);
  },

  async setTargetLanguage(code: string) {
    set({ targetLanguage: code });
    await get().retranslate(code);
  },

  async swapLanguages() {
    const updated = await api.swapLanguages();
    set({ targetLanguage: updated.target_language });
    await get().retranslate();
  },
}));

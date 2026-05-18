import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  TranslationChunkPayload,
  TranslationDonePayload,
  TranslationErrorPayload,
  TranslationStartPayload,
} from "@shared/types";

export const events = {
  onTranslationStart: (cb: (p: TranslationStartPayload) => void): Promise<UnlistenFn> =>
    listen<TranslationStartPayload>("translation://start", (e) => cb(e.payload)),
  onTranslationChunk: (cb: (p: TranslationChunkPayload) => void): Promise<UnlistenFn> =>
    listen<TranslationChunkPayload>("translation://chunk", (e) => cb(e.payload)),
  onTranslationDone: (cb: (p: TranslationDonePayload) => void): Promise<UnlistenFn> =>
    listen<TranslationDonePayload>("translation://done", (e) => cb(e.payload)),
  onTranslationError: (cb: (p: TranslationErrorPayload) => void): Promise<UnlistenFn> =>
    listen<TranslationErrorPayload>("translation://error", (e) => cb(e.payload)),
};

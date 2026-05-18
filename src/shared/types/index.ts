export interface AppConfig {
  hotkey: HotkeyConfig;
  target_language: string;
  fallback_target_language: string;
  style: TranslationStyle;
  provider: ProviderConfig;
  theme: Theme;
  autostart: boolean;
  popup: PopupConfig;
  preserve_clipboard: boolean;
  detect_source: boolean;
}

export interface HotkeyConfig {
  trigger: HotkeyTrigger;
  chord_window_ms: number;
}

export type HotkeyTrigger =
  | { mode: "Combo"; combo: string; presses: number }
  | { mode: "Modifier"; modifier: string; presses: number };

export type TranslationStyle = "Formal" | "Casual" | "Technical" | "Literal" | "Native";

export interface ProviderConfig {
  active: string;
  openrouter: OpenRouterConfig;
}

export interface OpenRouterConfig {
  model: string;
  base_url: string;
  temperature: number;
  max_tokens: number | null;
  referrer: string;
  app_name: string;
}

export type Theme = "dark" | "light" | "system";

export interface PopupConfig {
  width: number;
  max_height: number;
  follow_cursor: boolean;
  prefer_below: boolean;
  font_scale: number;
}

export interface LanguageOption {
  code: string;
  name: string;
  native_name: string;
}

export interface ModelInfo {
  id: string;
  name: string;
  context_length: number | null;
  pricing_prompt: number | null;
  pricing_completion: number | null;
  description: string | null;
}

export interface TranslationStartPayload {
  request_id: string;
  source_text: string;
  source_language: string | null;
  target_language: string;
  model: string;
  cursor_x: number;
  cursor_y: number;
}

export interface TranslationChunkPayload {
  request_id: string;
  delta: string;
}

export interface TranslationDonePayload {
  request_id: string;
  total: string;
  elapsed_ms: number;
}

export interface TranslationErrorPayload {
  request_id: string;
  message: string;
}

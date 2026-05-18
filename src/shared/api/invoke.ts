import { invoke } from "@tauri-apps/api/core";
import type { AppConfig, LanguageOption, ModelInfo } from "@shared/types";

export const api = {
  getConfig: () => invoke<AppConfig>("get_config"),
  saveConfig: (config: AppConfig) => invoke<void>("save_config", { config }),

  listLanguages: () => invoke<LanguageOption[]>("list_languages"),

  getApiKey: () => invoke<string | null>("get_api_key"),
  setApiKey: (key: string) => invoke<void>("set_api_key", { key }),
  clearApiKey: () => invoke<void>("clear_api_key"),

  fetchModels: () => invoke<ModelInfo[]>("fetch_models"),

  translateText: (text: string) => invoke<void>("translate_text", { text }),
  translateSelection: () => invoke<void>("translate_selection"),
  cancelTranslation: (requestId: string) =>
    invoke<void>("cancel_translation", { requestId }),

  closePopup: () => invoke<void>("close_popup"),
  pinPopup: (pinned: boolean) => invoke<void>("pin_popup", { pinned }),
  openSettings: () => invoke<void>("open_settings"),

  applyHotkey: () => invoke<void>("apply_hotkey"),
  applyAutostart: () => invoke<void>("apply_autostart"),

  testProvider: () => invoke<string>("test_provider"),
  swapLanguages: () => invoke<AppConfig>("swap_languages"),
  replaceSelection: (text: string) => invoke<void>("replace_selection", { text }),
};

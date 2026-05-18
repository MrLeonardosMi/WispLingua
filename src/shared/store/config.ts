import { create } from "zustand";
import { api } from "@shared/api/invoke";
import type { AppConfig } from "@shared/types";

interface ConfigState {
  config: AppConfig | null;
  loading: boolean;
  error: string | null;
  load: () => Promise<void>;
  update: (patch: Partial<AppConfig>) => Promise<void>;
  set: (config: AppConfig) => Promise<void>;
}

export const useConfig = create<ConfigState>((set, get) => ({
  config: null,
  loading: false,
  error: null,
  async load() {
    set({ loading: true, error: null });
    try {
      const config = await api.getConfig();
      set({ config, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },
  async update(patch) {
    const current = get().config;
    if (!current) return;
    const next = { ...current, ...patch };
    await api.saveConfig(next);
    set({ config: next });
  },
  async set(config) {
    await api.saveConfig(config);
    set({ config });
  },
}));

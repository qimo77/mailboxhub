import { create } from 'zustand';
import * as api from '../lib/tauri';
import type { AppSettings, AppSettingsPatch } from '../types/settings';

interface SettingsState {
  settings: AppSettings;
  databasePath: string;
  loading: boolean;
  loadSettings: () => Promise<void>;
  updateSettings: (patch: AppSettingsPatch) => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: {
    poll_interval_seconds: 5,
    theme: 'light',
    notifications_enabled: true,
    launch_auto_connect: true,
  },
  databasePath: '',
  loading: false,
  async loadSettings() {
    set({ loading: true });
    try {
      const [settings, databasePath] = await Promise.all([api.getSettings(), api.getDatabasePath()]);
      set({ settings, databasePath, loading: false });
    } catch {
      set({ loading: false });
    }
  },
  async updateSettings(patch) {
    const settings = await api.updateSettings(patch);
    set({ settings: { ...get().settings, ...settings } });
  },
}));

export interface AppSettings {
  poll_interval_seconds: number;
  theme: string;
  notifications_enabled: boolean;
  launch_auto_connect: boolean;
}

export interface AppSettingsPatch {
  poll_interval_seconds?: number;
  theme?: string;
  notifications_enabled?: boolean;
  launch_auto_connect?: boolean;
}

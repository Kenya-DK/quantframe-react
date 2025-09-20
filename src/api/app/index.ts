import { TauriClient } from "..";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";

export class AppModule {
  constructor(private readonly client: TauriClient) {}

  get_settings() {
    return useQuery({
      queryKey: ["app_get_settings"],
      queryFn: () => this.client.sendInvoke<TauriTypes.Settings>("app_get_settings"),
      retry: false,
      enabled: false, // Disable automatic fetching
    });
  }

  get_app_info() {
    return useQuery({
      queryKey: ["app_get_info"],
      queryFn: () => this.client.sendInvoke<TauriTypes.AppInfo>("app_get_app_info"),
      retry: false,
      enabled: false, // Disable automatic fetching
    });
  }
  updateSettings(settings: TauriTypes.Settings): Promise<TauriTypes.Settings> {
    return this.client.sendInvoke<TauriTypes.Settings>("app_update_settings", { settings });
  }
  exit() {
    return this.client.sendInvoke<void>("app_exit");
  }
  accept_tos(id: string): Promise<void> {
    return this.client.sendInvoke<void>("app_accept_tos", { id });
  }
  notify_reset(id: string): Promise<TauriTypes.NotificationSetting> {
    return this.client.sendInvoke<TauriTypes.NotificationSetting>("app_notify_reset", { id });
  }
}

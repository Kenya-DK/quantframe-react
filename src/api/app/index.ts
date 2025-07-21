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
    });
  }

  get_app_info() {
    return useQuery({
      queryKey: ["app_get_info"],
      queryFn: () => this.client.sendInvoke<TauriTypes.AppInfo>("app_get_app_info"),
      retry: false,
    });
  }
  updateSettings(settings: TauriTypes.Settings): Promise<TauriTypes.Settings> {
    return this.client.sendInvoke<TauriTypes.Settings>("app_update_settings", { settings });
  }
}

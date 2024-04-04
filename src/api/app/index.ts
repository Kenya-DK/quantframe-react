import { TauriClient } from "..";
import { DeepPartial } from "../../types";
import { Settings } from "./types";

export class AppModule {

  constructor(private readonly client: TauriClient) { }

  async getSettings(): Promise<Settings> {
    return this.client.sendInvoke('get_settings');
  }

  updateSettings(settings: DeepPartial<Settings>) {
    return this.client.sendInvoke('update_settings', settings);
  }

  exportLogs() {
    return this.client.sendInvoke('export_logs');
  }
}

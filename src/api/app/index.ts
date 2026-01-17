import { TauriClient } from "..";
import { InitializeResponds, Settings } from "@api/types";

export class AppModule {
  constructor(private readonly client: TauriClient) {}

  async init(): Promise<InitializeResponds> {
    const [err, res] = await this.client.sendInvoke<InitializeResponds>("app_init");
    if (err) throw err;
    return res;
  }

  async updateSettings(settings: Settings) {
    const [err] = await this.client.sendInvoke("app_update_settings", { settings });
    if (err) throw err;
  }

  async exit() {
    const [err] = await this.client.sendInvoke("app_exit");
    if (err) throw err;
  }
}

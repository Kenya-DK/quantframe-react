import { TauriClient } from "..";
import { TauriTypes } from "$types";

export class AppModule {
  constructor(private readonly client: TauriClient) {}

  async init(): Promise<TauriTypes.InitializeResponds> {
    const [err, res] = await this.client.sendInvoke<TauriTypes.InitializeResponds>("app_init");
    if (err) throw err;
    return res;
  }

  async updateSettings(settings: TauriTypes.Settings) {
    const [err] = await this.client.sendInvoke("app_update_settings", { settings });
    if (err) throw err;
  }

  async exit() {
    const [err] = await this.client.sendInvoke("app_exit");
    if (err) throw err;
  }
}

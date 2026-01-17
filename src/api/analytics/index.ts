import { TauriClient } from "..";
export class AnalyticsModule {
  constructor(private readonly client: TauriClient) {}

  async add_metric(key: string, value: number | string): Promise<void> {
    return await this.client.sendInvoke("analytics_add_metric", { key, value });
  }

  async setLastUserActivity(): Promise<void> {
    await this.client.sendInvoke<void>("analytics_set_last_user_activity");
  }
}

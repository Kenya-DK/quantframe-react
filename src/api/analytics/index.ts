import { TauriClient } from "..";

export class AnalyticsModule {
  constructor(private readonly client: TauriClient) { }

  async setLastUserActivity(): Promise<void> {
    await this.client.sendInvoke<void>('analytics_set_last_user_activity');
  }

  async sendMetric(key: string, value: string): Promise<void> {
    await this.client.sendInvoke<number>('analytics_send_metric', { key, value });
  }
}

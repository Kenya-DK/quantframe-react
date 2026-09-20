import { TauriClient } from "..";
export class AnalyticsModule {
  constructor(private readonly client: TauriClient) {}

  async add_metric(key: string, value: Record<string, string>): Promise<void> {
    return await this.client.sendInvoke("track_event", { key, value });
  }
}

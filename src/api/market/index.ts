import { TauriClient } from "..";
import { QuantframeApiTypes } from "../../types";
type ResponseType = { labels: string[]; registered_users_chart: number[]; total_users_chart: number[] };
export class MarketModule {
  constructor(private readonly client: TauriClient) {}

  async get_user_activity(query: QuantframeApiTypes.WfmControllerGetUserActiveHistoryParams): Promise<ResponseType> {
    const items = await this.client.sendInvoke<ResponseType>("get_user_activity", { query });
    return items;
  }
}

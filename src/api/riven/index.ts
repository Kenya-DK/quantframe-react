import { TauriClient } from "..";
import { QuantframeApiTypes } from "../../types";

export class RivenModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(query: QuantframeApiTypes.RivenPriceControllerGetListParams): Promise<QuantframeApiTypes.RivenPriceControllerGetListData> {
    query.from_date = query.from_date ? encodeURIComponent(query.from_date) : undefined;
    query.to_date = query.to_date ? encodeURIComponent(query.to_date) : undefined;
    return this.client.sendInvoke<QuantframeApiTypes.RivenPriceControllerGetListData>(`riven_prices_lookup`, { query });
  }
  exportJson = async (query: QuantframeApiTypes.RivenPriceControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_riven_price_data", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}

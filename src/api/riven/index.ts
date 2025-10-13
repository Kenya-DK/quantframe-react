import { TauriClient } from "..";
import { QuantframeApiTypes } from "../../types";

export class RivenModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(query: QuantframeApiTypes.RivenPriceControllerGetListParams): Promise<QuantframeApiTypes.RivenPriceControllerGetListData> {
    return this.client.sendInvoke<QuantframeApiTypes.RivenPriceControllerGetListData>(`riven_prices_lookup`, { query });
  }
  exportJson = async (query: QuantframeApiTypes.RivenPriceControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_riven_price_data", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}

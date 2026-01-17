import { TauriClient } from "..";
import { QuantframeApiTypes } from "../../types";

export class ItemModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(query: QuantframeApiTypes.ItemPriceControllerGetListParams): Promise<QuantframeApiTypes.ItemPriceControllerGetListData> {
    return this.client.sendInvoke<QuantframeApiTypes.ItemPriceControllerGetListData>(`item_prices_lookup`, { query });
  }
  exportJson = async (query: QuantframeApiTypes.ItemPriceControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_item_price_data", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}

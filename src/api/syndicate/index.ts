import { TauriClient } from "..";
import { QuantframeApiTypes } from "../../types";

export class SyndicateModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(
    query: QuantframeApiTypes.SyndicateItemPriceControllerGetListParams,
  ): Promise<QuantframeApiTypes.SyndicateItemPriceControllerGetListData> {
    return this.client.sendInvoke<QuantframeApiTypes.SyndicateItemPriceControllerGetListData>(`syndicate_item_prices_lookup`, { query });
  }
  exportJson = async (query: QuantframeApiTypes.SyndicateItemPriceControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_syndicate_item_price_data", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}

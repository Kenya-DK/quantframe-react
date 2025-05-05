import { TauriClient } from "..";
import { QuantframeApiTypes } from "$types";
export class ItemModule {
  constructor(private readonly client: TauriClient) {}

  getItemPriceOverview = async (): Promise<QuantframeApiTypes.ItemControllerGetPriceOverviewData> => {
    return this.client.get<QuantframeApiTypes.ItemControllerGetPriceOverviewData>(`items/price/overview`);
  };
  async getAll(query: QuantframeApiTypes.ItemControllerGetListParams): Promise<QuantframeApiTypes.ItemControllerGetListData> {
    return this.client.get<QuantframeApiTypes.ItemControllerGetListData>(`items/prices`, this.client.objectToParameters(query));
  }
}

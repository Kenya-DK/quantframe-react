import { TauriClient } from "..";
import { QFApiTypes } from "../types";

export class ItemModule {
  constructor(private readonly client: TauriClient) {}

  getItemPriceOverview = async () => {
    return this.client.get<QFApiTypes.ItemPriceOverviewDto>(`items/price/overview`);
  };
}

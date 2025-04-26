import { TauriClient } from "..";
export class ItemModule {
  constructor(private readonly client: TauriClient) {}

  getItemPriceOverview = async () => {
    return this.client.get<any>(`items/price/overview`);
  };
}

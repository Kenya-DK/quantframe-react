import { TauriClient } from "..";
import { QuantframeApiTypes, TauriTypes } from "../../types";

export class RivenModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(query: QuantframeApiTypes.RivenPriceControllerGetListParams): Promise<QuantframeApiTypes.RivenPriceControllerGetListData> {
    query.from_date = query.from_date ? encodeURIComponent(query.from_date) : undefined;
    query.to_date = query.to_date ? encodeURIComponent(query.to_date) : undefined;
    return this.client.sendInvoke<QuantframeApiTypes.RivenPriceControllerGetListData>(`riven_prices_lookup`, { query });
  }
  async pricerAvailable(): Promise<boolean> {
    return this.client.sendInvoke<boolean>("riven_pricer_available");
  }
  async predictPrice(input: TauriTypes.RivenPriceInput): Promise<TauriTypes.RivenPriceEstimate | null> {
    return this.client.sendInvoke<TauriTypes.RivenPriceEstimate | null>("riven_price_predict", { input });
  }
  async reloadPricer(): Promise<boolean> {
    return this.client.sendInvoke<boolean>("riven_pricer_reload");
  }
  async getKnownRivenWeapons(): Promise<string[]> {
    return this.client.sendInvoke<string[]>("get_known_riven_weapons");
  }
  exportJson = async (query: QuantframeApiTypes.RivenPriceControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_riven_price_data", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}

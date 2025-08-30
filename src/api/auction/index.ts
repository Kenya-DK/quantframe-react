import { TauriClient } from "..";
import { WFMarketTypes } from "$types";
export class AuctionModule {
  constructor(private readonly client: TauriClient) {}
  async getPagination(query: WFMarketTypes.WfmAuctionControllerGetListParams): Promise<WFMarketTypes.WfmAuctionControllerGetListData> {
    return await this.client.sendInvoke<WFMarketTypes.WfmAuctionControllerGetListData>("get_wfm_auctions_pagination", { query });
  }

  async refreshAuctions(): Promise<any> {
    return await this.client.sendInvoke<any>("auction_refresh");
  }

  async deleteAllAuctions(): Promise<any> {
    return await this.client.sendInvoke<any>("auction_delete_all");
  }
  async deleteById(id: string): Promise<any> {
    return await this.client.sendInvoke<any>("auction_delete_by_id", { id });
  }
}

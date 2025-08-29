import { TauriClient } from "..";
import { WFMarketTypes } from "$types";
export class OrderModule {
  constructor(private readonly client: TauriClient) {}
  async getPagination(query: WFMarketTypes.WfmOrderControllerGetListParams): Promise<WFMarketTypes.WfmOrderControllerGetListData> {
    return await this.client.sendInvoke<WFMarketTypes.WfmOrderControllerGetListData>("get_wfm_orders_pagination", { query });
  }

  async getStatusCounts(query: WFMarketTypes.WfmOrderControllerGetListParams): Promise<{ [key: string]: number[] }> {
    return await this.client.sendInvoke<{ [key: string]: number[] }>("get_wfm_orders_status_counts", {
      query: this.client.convertToTauriQuery(query),
    });
  }
  async refreshOrders(): Promise<any> {
    return await this.client.sendInvoke<any>("order_refresh");
  }

  async deleteAllOrders(): Promise<any> {
    return await this.client.sendInvoke<any>("order_delete_all");
  }
  async deleteById(id: string): Promise<any> {
    return await this.client.sendInvoke<any>("order_delete_by_id", { id });
  }
}

import { TauriClient } from "..";
import { TauriTypes } from "$types";
export class WfInventoryModule {
  constructor(private readonly client: TauriClient) {}
  async getRivensPagination(query: TauriTypes.WFItemControllerGetListParams): Promise<TauriTypes.WFInvRivenControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.WFInvRivenControllerGetListData>("wf_inventory_get_rivens", {
      query: this.client.convertToTauriQuery(query),
    });
  }
}

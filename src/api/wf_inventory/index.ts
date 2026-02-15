import { TauriClient } from "..";
import { TauriTypes } from "../../types";
export class WfInventoryModule {
  constructor(private readonly client: TauriClient) {}
  async getVeiledRivenPagination(query: TauriTypes.VeiledRivenControllerGetListParams): Promise<TauriTypes.VeiledRivenControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.VeiledRivenControllerGetListData>("wf_inventory_get_veiled_rivens", {
      query: this.client.convertToTauriQuery(query),
    });
  }
  async getUnveiledRivens(): Promise<TauriTypes.RivenSummary[]> {
    return await this.client.sendInvoke<TauriTypes.RivenSummary[]>("wf_inventory_get_unveiled_rivens", {});
  }
}

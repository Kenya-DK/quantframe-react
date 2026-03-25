import { TauriClient } from "..";
import { ItemBase, TauriTypes } from "$types";
export class WfInventoryModule {
  constructor(private readonly client: TauriClient) {}
  async getRivensPagination(query: TauriTypes.WFItemControllerGetListParams): Promise<TauriTypes.WFInvRivenControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.WFInvRivenControllerGetListData>("wf_inventory_get_rivens", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getSyndicatesPagination(
    query: TauriTypes.WFItemControllerGetListParams,
  ): Promise<ItemBase<{ max_standing: number; min_standing: number; total: number; colour: string; background_colour: string }>[]> {
    return await this.client.sendInvoke<
      ItemBase<{ max_standing: number; min_standing: number; total: number; colour: string; background_colour: string }>[]
    >("wf_inventory_get_syndicates", {
      query: this.client.convertToTauriQuery(query),
    });
  }
}

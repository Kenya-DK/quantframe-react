import { TauriClient } from "..";
import { QuantframeApiTypes } from "$types";
export class RivenModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(query: QuantframeApiTypes.RivenControllerGetRivenListParams): Promise<QuantframeApiTypes.RivenControllerGetRivenListData> {
    return this.client.get<QuantframeApiTypes.RivenControllerGetRivenListData>(`rivens/prices`, this.client.objectToParameters(query));
  }
}

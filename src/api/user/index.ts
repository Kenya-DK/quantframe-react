import { TauriClient } from "..";
import { QuantframeApiTypes } from "$types";

export class UserModule {
  constructor(private readonly client: TauriClient) {}
  async getAll(query: QuantframeApiTypes.WfmControllerGetUserActiveHistoryParams): Promise<QuantframeApiTypes.WfmControllerGetUserActiveHistoryData> {
    return this.client.get<QuantframeApiTypes.WfmControllerGetUserActiveHistoryData>(
      `wfm/users_active_history`,
      this.client.objectToParameters(query)
    );
  }
}

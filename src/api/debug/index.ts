import { TauriClient } from "..";
import { TauriTypes } from "../../types";

export class DebugModule {
  constructor(private readonly client: TauriClient) {}

  async get_wfm_state() {
    return await this.client.sendInvoke<{ [key: string]: any }>("debug_get_wfm_state");
  }
  async get_ee_logs(query: TauriTypes.EELogControllerGetListParams): Promise<TauriTypes.EELogControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.EELogControllerGetListData>("debug_get_ee_logs", { query });
  }
  async ee_logs_exportJson(query: TauriTypes.EELogControllerGetListParams): Promise<string> {
    return await this.client.sendInvoke<string>("debug_export_ee_logs", { query });
  }
}

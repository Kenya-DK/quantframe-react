import { TauriClient } from "..";

export class DebugModule {
  constructor(private readonly client: TauriClient) {}

  async get_wfm_state() {
    return await this.client.sendInvoke<{ [key: string]: any }>("debug_get_wfm_state");
  }
}

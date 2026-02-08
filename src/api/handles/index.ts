import { TauriClient } from "..";
import { TauriTypes } from "$types";

export class HandlesModule {
  constructor(private readonly client: TauriClient) {}
  async handle_items(items: TauriTypes.HandleItem[]): Promise<number> {
    return await this.client.sendInvoke<number>("handles_handle_items", { items });
  }
}

import { TauriClient } from "..";

export class ChatModule {
  constructor(private readonly client: TauriClient) { }

  async refresh(): Promise<void> {
    await this.client.sendInvoke<void>('chat_refresh');
  }
}

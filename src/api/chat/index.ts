import { TauriClient } from "..";

export class ChatModule {
  constructor(private readonly client: TauriClient) { }

  async refresh(): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('chat_refresh');
    if (err)
      throw err;
    return res;
  }
}

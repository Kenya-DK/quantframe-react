import { TauriClient } from "..";

export class ChatModule {
  constructor(private readonly client: TauriClient) { }

  async getChannels() {
    return await this.client.sendInvoke("get_channels");
  }

  async getMessagesFromChannel(channel: string) {
    return await this.client.sendInvoke("get_messages_from_channel", { channel });
  }

  async deleteChannel(channel: string) {
    return await this.client.sendInvoke("delete_channel", { channel });
  }
}

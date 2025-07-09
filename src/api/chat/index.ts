import { TauriClient } from "..";
import { WFMarketTypes } from "$types/index";

export class ChatModule {
  constructor(private readonly client: TauriClient) {}

  async refresh(): Promise<WFMarketTypes.ChatData[]> {
    const [err, res] = await this.client.sendInvoke<WFMarketTypes.ChatData[]>("chat_refresh");
    if (err) throw err;
    return res;
  }

  async delete(id: string): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("chat_delete", { id });
    if (err) throw err;
    return res;
  }

  async deleteAll(): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("chat_delete_all");
    if (err) throw err;
    return res;
  }

  async getChatMessages(id: string): Promise<WFMarketTypes.ChatMessage[]> {
    const [err, res] = await this.client.sendInvoke<WFMarketTypes.ChatMessage[]>("chat_get_messages", { id });
    if (err) throw err;
    return res;
  }

  async on_message(msg: WFMarketTypes.ChatMessage) {
    const [err, res] = await this.client.sendInvoke<WFMarketTypes.ChatMessage[]>("chat_on_message", { msg });
    if (err) throw err;
    return res;
  }

  async set_active(id: string, unread: number): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("chat_set_active", { id, unread });
    if (err) throw err;
    return res;
  }

  async send_message(id: string, msg: string): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("chat_send_message", { id, msg });
    if (err) throw err;
    return res;
  }
}

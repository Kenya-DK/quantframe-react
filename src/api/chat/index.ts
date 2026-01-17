import { TauriClient } from "..";
import { WFMarketTypes } from "$types/index";

export class ChatModule {
  constructor(private readonly client: TauriClient) {}

  async refresh(): Promise<WFMarketTypes.ChatData[]> {
    return await this.client.sendInvoke<WFMarketTypes.ChatData[]>("chat_refresh");
  }

  async getPagination(query: WFMarketTypes.WfmChatDataControllerGetListParams): Promise<WFMarketTypes.WfmChatDataControllerGetListData> {
    return await this.client.sendInvoke<WFMarketTypes.WfmChatDataControllerGetListData>("get_chat_pagination", { query });
  }

  async delete(id: string): Promise<void> {
    return await this.client.sendInvoke<void>("chat_delete", { id });
  }

  async getChatMessages(id: string): Promise<WFMarketTypes.ChatMessage[]> {
    return await this.client.sendInvoke<WFMarketTypes.ChatMessage[]>("chat_get_messages_by_id", { id });
  }

  async set_active(id: string | undefined): Promise<void> {
    return await this.client.sendInvoke<void>("chat_set_active", { id });
  }

  async send_message(id: string, msg: string): Promise<void> {
    return await this.client.sendInvoke<void>("chat_send_message", { id, msg });
  }
  async chat_set_active_chat(id: string): Promise<void> {
    return await this.client.sendInvoke<void>("chat_set_active_chat", { id });
  }
}

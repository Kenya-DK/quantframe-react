import { TauriClient } from "..";

export class NotificationModule {
  constructor(private client: TauriClient) {}

  async sendSystemNotification(title: string, message: string) {
    return this.client.sendInvoke("send_system_notification", { title, message });
  }
}

import { TauriClient } from "..";

export class NotificationModule {
  constructor(private readonly client: TauriClient) { }

  async sendSystemNotification(title: string, message: string, icon?: string, sound?: string) {
    return await this.client.sendInvoke("send_system_notification", { title, message, icon, sound });
  }

  async sendDiscordNotification(title: string, message: string) {
    return await this.client.sendInvoke("send_discord_notification", { title, message });
  }
}
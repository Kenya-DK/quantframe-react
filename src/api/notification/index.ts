import { TauriClient } from "..";

export class NotificationModule {
  constructor(private readonly client: TauriClient) { }

  sendNotification(_title: string, _message: string, _icon?: string, _sound?: string) {

  }
}

export * from './types';
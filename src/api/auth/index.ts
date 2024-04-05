import { TauriClient } from "..";

export class AuthModule {
  constructor(private readonly client: TauriClient) { }

  logIn(username: string, password: string) {
    return this.client.sendInvoke("log_in", { username, password });
  }
}

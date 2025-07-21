import { TauriClient } from "..";
import { TauriTypes } from "$types";
export class AuthModule {
  constructor(private readonly client: TauriClient) {}

  async me(): Promise<TauriTypes.User> {
    return await this.client.sendInvoke<TauriTypes.User>("auth_me");
  }

  async login(email: string, password: string): Promise<TauriTypes.User | undefined> {
    return await this.client.sendInvoke<TauriTypes.User>("auth_login", { email, password });
  }
  async logout(): Promise<TauriTypes.User> {
    return await this.client.sendInvoke<TauriTypes.User>("auth_logout");
  }
}

import { TauriClient } from "..";
import { TauriTypes } from "$types";
export class AuthModule {
  private permissionsCache: { [key in TauriTypes.PermissionsFlags]?: boolean } = {};
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
  async hasPermission(flag: TauriTypes.PermissionsFlags): Promise<boolean> {
    if (this.permissionsCache[flag] !== undefined) return this.permissionsCache[flag]!;

    const result = await this.client.sendInvoke<boolean>("auth_has_permission", { flag });
    this.permissionsCache[flag] = result;
    return result;
  }
}

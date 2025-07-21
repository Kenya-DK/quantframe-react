import { TauriClient } from "..";
import { TauriTypes, UserStatus } from "$types";
export class UserModule {
  constructor(private readonly client: TauriClient) {}

  async set_status(status: UserStatus): Promise<void> {
    await this.client.sendInvoke<TauriTypes.User>("user_set_status", { status });
  }
}

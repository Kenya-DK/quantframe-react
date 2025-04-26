import { TauriClient } from "..";
import { UserStatus, TauriTypes } from "$types";
import wfmSocket from "@models/wfmSocket";
import { WFMarketTypes } from "../../types";
export class AuthModule {
  constructor(private readonly client: TauriClient) {}

  async login(email: string, password: string): Promise<TauriTypes.User> {
    const [err, rep] = await this.client.sendInvoke<TauriTypes.User>("auth_login", { email, password });
    if (err) throw err;
    return rep;
  }

  async update_status(status: UserStatus) {
    wfmSocket.emit({ type: WFMarketTypes.SocketEvent.UpdateUserStatus, payload: status });
  }

  async set_status(status: UserStatus) {
    const [err] = await this.client.sendInvoke("auth_set_status", { status });
    if (err) throw err;
  }

  async logout() {
    const [err] = await this.client.sendInvoke("auth_logout");
    if (err) throw err;
  }
}

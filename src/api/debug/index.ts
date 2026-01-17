import { TauriClient } from "..";

export class DebugModule {
  constructor(private readonly client: TauriClient) {}

  async reset(target: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>("debug_db_reset", { target });
    if (err) throw err;
    return res;
  }
}

import { TauriClient } from "..";

export class DebugModule {
  constructor(private readonly client: TauriClient) { }

  async reset(target: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>('debug_db_reset', { target });
    if (err)
      throw err;
    return res;
  }
  async migrate(target: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>('debug_migrate_data_base', { target });
    if (err)
      throw err;
    return res;
  }
}

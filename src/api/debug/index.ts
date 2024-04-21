import { TauriClient } from "..";

export class DebugModule {
  constructor(private readonly client: TauriClient) { }

  async reset(target: string): Promise<boolean> {
    const [, rep] = await this.client.sendInvoke<boolean>('debug_db_reset', { target });
    if (!rep)
      throw new Error("Failed to create stock item");
    return rep;
  }
  async migrate(target: string): Promise<boolean> {
    const [, rep] = await this.client.sendInvoke<boolean>('debug_migrate_data_base', { target });
    if (!rep)
      throw new Error("Failed to create stock item");
    return rep;
  }
}

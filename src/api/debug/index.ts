import { TauriClient } from "..";

export class DebugModule {
  constructor(private readonly client: TauriClient) {}

  async reset(target: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>("debug_db_reset", { target });
    if (err) throw err;
    return res;
  }
  async migrate(target: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>("debug_migrate_data_base", { target });
    if (err) throw err;
    return res;
  }
  async importAlgoTrader(db_path: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>("debug_import_algo_trader", { db_path });
    if (err) throw err;
    return res;
  }
  async debug_method(name: string, payload: any): Promise<any> {
    const [err, res] = await this.client.sendInvoke<any>("debug_method", { name, payload });
    if (err) throw err;
    return res;
  }
}

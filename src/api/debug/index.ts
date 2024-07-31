import { TauriClient } from "..";

export class DebugModule {
  constructor(private readonly client: TauriClient) { }

  async reset(target: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>('debug_db_reset', { target });
    await this.client.analytics.sendMetric('Debug_ResetDatabase', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }
  async migrate(target: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>('debug_migrate_data_base', { target });
    await this.client.analytics.sendMetric('Debug_MigrateDatabase', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }
  async importAlgoTrader(db_path: string): Promise<boolean> {
    const [err, res] = await this.client.sendInvoke<boolean>('debug_import_algo_trader', { db_path });
    await this.client.analytics.sendMetric('Debug_ImportAlgoTrader', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }
}

import { TauriClient } from "..";

export class LogParserModule {
  constructor(private readonly client: TauriClient) {}

  async getLogEELines(): Promise<string[]> {
    const [err, res] = await this.client.sendInvoke<string[]>("get_cache_lines", {});
    if (err) throw err;
    return res;
  }
  async getLastReadDate(): Promise<string> {
    const [err, res] = await this.client.sendInvoke<string>("get_last_read_date", {});
    if (err) throw err;
    return res;
  }
  async clearLogCache(): Promise<void> {
    const [err] = await this.client.sendInvoke<void>("clear_cache_lines", {});
    if (err) throw err;
  }
  async dumpLogCache(): Promise<string> {
    const [err, rep] = await this.client.sendInvoke<string>("dump_cache_lines", {});
    if (err) throw err;
    return rep;
  }
}

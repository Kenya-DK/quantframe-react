import { TauriClient } from "..";

export class LogModule {
  constructor(private readonly client: TauriClient) { }

  async open(): Promise<void> {
    const [err] = await this.client.sendInvoke<void>("log_open_folder");
    if (err)
      throw err;
  }

  async export(): Promise<void> {
    const [err] = await this.client.sendInvoke<void>("log_export");
    if (err)
      throw err;
  }
}

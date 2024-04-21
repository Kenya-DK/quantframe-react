import { TauriClient } from "..";

export class LiveScraperModule {
  constructor(private readonly client: TauriClient) { }

  async start() {
    return await this.runningState(true)
  }

  async stop() {
    return await this.runningState(false)
  }

  private async runningState(enable: boolean) {
    return this.client.sendInvoke('live_scraper_set_running_state', { enable })
  }
}

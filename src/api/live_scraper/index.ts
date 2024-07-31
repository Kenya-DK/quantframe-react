import { TauriClient } from "..";

export class LiveScraperModule {
  constructor(private readonly client: TauriClient) { }

  async start() {
    const rep = await this.runningState(true)
    return rep;
  }

  async stop() {
    const rep = await this.runningState(false)
    return rep;
  }

  private async runningState(enable: boolean) {
    const [err, res] = await this.client.sendInvoke('live_scraper_set_running_state', { enable })
    this.client.analytics.sendMetric(`LiveScraper_SetState_${enable ? 'Start' : 'Stop'}`, err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }
}

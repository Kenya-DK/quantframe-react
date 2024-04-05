import { TauriClient } from "..";

export class LiveScraperModule {
  constructor(private readonly client: TauriClient) { }

  async startLiveScraper() {
    return await this.client.sendInvoke("start_live_scraper");
  }

  async stopLiveScraper() {
    return await this.client.sendInvoke("stop_live_scraper");
  }
}

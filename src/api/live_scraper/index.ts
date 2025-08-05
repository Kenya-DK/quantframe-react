import { TauriClient } from "..";
import { TauriTypes, UserStatus } from "$types";
export class LiveScraperModule {
  constructor(private readonly _client: TauriClient) {}

  async toggleLiveScraper(): Promise<void> {
    this._client.sendInvoke("live_scraper_toggle");
  }
}

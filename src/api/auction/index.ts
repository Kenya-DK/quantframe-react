import { TauriClient } from "..";

export class AuctionModule {
  constructor(private readonly client: TauriClient) { }

  async refresh(): Promise<void> {
    await this.client.sendInvoke<void>('auction_refresh');
  }
}

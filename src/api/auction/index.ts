import { TauriClient } from "..";

export class AuctionModule {
  constructor(private readonly client: TauriClient) { }

  async refresh(): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('auction_refresh');
    if (err)
      throw err;
    return res;
  }
}

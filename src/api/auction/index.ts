import { TauriClient } from "..";
import { Wfm } from "../../types";

export class AuctionModule {
  constructor(private readonly client: TauriClient) { }

  async delete(id: string): Promise<Wfm.Auction<string>> {
    const [err, auction] = await this.client.sendInvoke<Wfm.Auction<string>>('auction_delete', { id: id });
    await this.client.analytics.sendMetric('WFM_AuctionDelete', err ? 'failed' : 'success');
    if (err)
      throw err;
    return auction;
  }

  async refresh(): Promise<number> {
    const [err, res] = await this.client.sendInvoke<number>('auction_refresh');
    await this.client.analytics.sendMetric('WFM_AuctionRefresh', err ? 'failed' : 'success');
    if (err)
      throw err;

    return res;
  }

  async deleteAll(): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('auction_delete_all');
    await this.client.analytics.sendMetric('WFM_AuctionDeleteAll', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }
}

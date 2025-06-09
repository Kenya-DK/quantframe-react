import { TauriClient } from "..";
import { TauriTypes } from "$types";

export class SummaryModule {
  constructor(private readonly client: TauriClient) {}

  async overview(): Promise<TauriTypes.TradingSummaryDto> {
    const [err, overview] = await this.client.sendInvoke<TauriTypes.TradingSummaryDto>("summary_overview");
    if (err) throw err;
    return overview;
  }
}

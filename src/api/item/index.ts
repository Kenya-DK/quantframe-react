import { TauriClient } from "..";
import { Paginated, PaginatedWithInclude, SubType, SyndicatesPrice, ItemPrice, ItemPriceChart } from "../types";

export class ItemModule {
  constructor(private readonly client: TauriClient) {}
  async getSyndicatesPrices(page: number, limit: number): Promise<Paginated<SyndicatesPrice>> {
    const [err, res] = await this.client.sendInvoke<Paginated<SyndicatesPrice>>("item_get_syndicates_prices", {
      page: page,
      limit: limit,
    });
    if (err) throw err;
    return res;
  }
  async getItemsPrices(
    page: number,
    limit: number,
    from_date: Date,
    to_date: Date,
    order_type?: string,
    wfm_url?: string,
    sub_type?: SubType,
    include?: string,
    group_by?: string
  ): Promise<PaginatedWithInclude<ItemPrice, ItemPriceChart>> {
    const [err, res] = await this.client.sendInvoke<PaginatedWithInclude<ItemPrice, ItemPriceChart>>("item_get_prices", {
      page: page,
      limit: limit,
      from_date: from_date,
      to_date: to_date,
      order_type: order_type,
      wfm_url: wfm_url,
      sub_type: sub_type,
      include: include,
      group_by: group_by,
    });
    if (err) throw err;
    return res;
  }
}

import { TauriClient } from "..";
import { ComplexFilter, Sort } from "../../types";
import { Paginated, SyndicatesPrice } from "../types";

export class ItemModule {
  constructor(private readonly client: TauriClient) {}
  async getSyndicatesPrices(page: number, limit: number, filter?: ComplexFilter, sort?: Sort): Promise<Paginated<SyndicatesPrice>> {
    const [err, res] = await this.client.sendInvoke<Paginated<SyndicatesPrice>>("item_get_syndicates_prices", {
      page: page,
      limit: limit,
      filter: filter,
      sort: sort,
    });
    if (err) throw err;
    return res;
  }
}

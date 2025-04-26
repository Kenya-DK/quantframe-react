import { TauriClient } from "..";
import { WFMarketTypes } from "$types/index";

export class OrderModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(): Promise<WFMarketTypes.OrderDto[]> {
    const [err, orders] = await this.client.sendInvoke<WFMarketTypes.OrderDto[]>("order_get");
    if (err) throw err;
    return orders;
  }

  async delete(orderId: string): Promise<WFMarketTypes.OrderDto> {
    const [err, order] = await this.client.sendInvoke<WFMarketTypes.OrderDto>("order_delete", { id: orderId });
    if (err) throw err;
    return order;
  }

  async refresh(): Promise<number> {
    const [err, res] = await this.client.sendInvoke<number>("order_refresh");
    if (err) throw err;
    return res;
  }

  async deleteAll(): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("order_delete_all");
    if (err) throw err;
    return res;
  }
}

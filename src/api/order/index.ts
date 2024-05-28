import { TauriClient } from "..";
import { Wfm } from "$types/index";

export class OrderModule {
  constructor(private readonly client: TauriClient) { }

  async getAll(): Promise<Wfm.OrderDto[]> {
    const [err, orders] = await this.client.sendInvoke<Wfm.OrderDto[]>('order_get');
    if (err)
      throw err;
    return orders;
  }

  async delete(orderId: string): Promise<Wfm.OrderDto> {
    const [err, order] = await this.client.sendInvoke<Wfm.OrderDto>('order_delete', orderId);
    if (err)
      throw err;
    return order;
  }

  async refresh(): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('order_refresh');
    if (err)
      throw err;
    return res;
  }

  async deleteAll(): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('order_delete_all');
    if (err)
      throw err;
    return res;
  }
}

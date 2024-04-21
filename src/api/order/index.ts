import { TauriClient } from "..";
import { Wfm } from "$types/index";

export class OrderModule {
  constructor(private readonly client: TauriClient) { }

  async getAll(): Promise<Wfm.OrderDto[]> {
    const [, orders] = await this.client.sendInvoke<Wfm.OrderDto[]>('order_get');
    if (!orders)
      return [];
    return orders;
  }

  async delete(orderId: string): Promise<Wfm.OrderDto> {
    const [, order] = await this.client.sendInvoke<Wfm.OrderDto>('order_delete', orderId);
    if (!order)
      throw new Error("Failed to delete order");
    return order;
  }

  async refresh(): Promise<void> {
    await this.client.sendInvoke<void>('order_refresh');
  }

  async deleteAll(): Promise<void> {
    await this.client.sendInvoke<void>('order_delete_all');
  }
}

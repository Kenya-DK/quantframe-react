import { TauriClient } from "..";
import { Wfm } from "$types/index";

export class OrderModule {
  constructor(private readonly client: TauriClient) { }

  async getAll(): Promise<Wfm.OrderDto[]> {
    const [err, orders] = await this.client.sendInvoke<Wfm.OrderDto[]>('order_get');
    await this.client.analytics.sendMetric('WFM_OrderGetAll', err ? 'failed' : 'success');
    if (err)
      throw err;
    return orders;
  }

  async delete(orderId: string): Promise<Wfm.OrderDto> {
    const [err, order] = await this.client.sendInvoke<Wfm.OrderDto>('order_delete', { id: orderId });
    await this.client.analytics.sendMetric('WFM_OrderDelete', err ? 'failed' : 'success');
    if (err)
      throw err;
    return order;
  }

  async refresh(): Promise<number> {
    const [err, res] = await this.client.sendInvoke<number>('order_refresh');
    await this.client.analytics.sendMetric('WFM_OrderRefresh', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }

  async deleteAll(): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('order_delete_all');
    await this.client.analytics.sendMetric('WFM_OrderDeleteAll', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }
}

import { TauriClient } from "..";
export class OrderModule {
  constructor(private readonly client: TauriClient) {}

  async refreshOrders(): Promise<any> {
    return await this.client.sendInvoke<any>("order_refresh");
  }

  async deleteAllOrders(): Promise<any> {
    return await this.client.sendInvoke<any>("order_delete_all");
  }
}

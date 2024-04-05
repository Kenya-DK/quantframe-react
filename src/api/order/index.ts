import { TauriClient } from "..";
import { Order, OrderCreate, OrderUpdate } from "../types";

export class OrderModule {
  constructor(private readonly client: TauriClient) { }

  async getOrders(): Promise<Order[]> {
    return await this.client.sendInvoke("get_orders");
  }

  async createOrder(data: OrderCreate) {
    return await this.client.sendInvoke("create_order", data);
  }

  async updateOrder(id: string, data: OrderUpdate) {
    return await this.client.sendInvoke("update_order", { id, data });
  }

  async deleteOrder(id: string) {
    return await this.client.sendInvoke("delete_order", { id });
  }

  async getOrder(id: string): Promise<Order> {
    return await this.client.sendInvoke("get_order", { id });
  }

  async getOrdersByUser(userId: string): Promise<Order[]> {
    return await this.client.sendInvoke("get_orders_by_user", { userId });
  }
}

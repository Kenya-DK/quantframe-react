import { TauriClient } from "../..";
import { CreateWishListItem, WishListItem, UpdateWishListItem } from "@api/types";

export class WishListModule {
  constructor(private readonly client: TauriClient) {}

  async reload(): Promise<void> {
    const [] = await this.client.sendInvoke<void>("wish_list_reload");
  }

  async create(entry: CreateWishListItem): Promise<WishListItem> {
    const [err, stockItem] = await this.client.sendInvoke<WishListItem>("wish_list_create", entry);
    if (err) throw err;
    return stockItem;
  }

  async update(entry: UpdateWishListItem): Promise<WishListItem> {
    const [err, stockItem] = await this.client.sendInvoke<WishListItem>("wish_list_update", entry);
    if (err) throw err;
    return stockItem;
  }

  async updateBulk(ids: number[], entry: UpdateWishListItem): Promise<number> {
    const [err, stockItem] = await this.client.sendInvoke<number>("wish_list_update_bulk", { ...entry, ids });
    if (err) throw err;
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("wish_list_delete", { id });
    if (err) throw err;
    return res;
  }

  async deleteBulk(ids: number[]): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("wish_list_delete_bulk", { ids });
    if (err) throw err;
    return res;
  }
}

import { TauriClient } from "..";
import { TauriTypes } from "../../types";
export class WishListModule {
  constructor(private readonly client: TauriClient) {}

  async getPagination(query: TauriTypes.WishListControllerGetListParams): Promise<TauriTypes.WishListControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.WishListControllerGetListData>("get_wish_list_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getFinancialReport(query: TauriTypes.WishListControllerGetListParams): Promise<TauriTypes.FinancialReport> {
    return await this.client.sendInvoke<TauriTypes.FinancialReport>("get_wish_list_financial_report", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getStatusCounts(query: TauriTypes.WishListControllerGetListParams): Promise<{ [key: string]: number }> {
    return await this.client.sendInvoke<{ [key: string]: number }>("get_wish_list_status_counts", { query: this.client.convertToTauriQuery(query) });
  }

  async create(input: TauriTypes.CreateWishListItem) {
    return await this.client.sendInvoke<TauriTypes.WishListItem>("wish_list_create", { input });
  }

  async update(input: TauriTypes.UpdateWishListItem): Promise<TauriTypes.WishListItem> {
    return await this.client.sendInvoke<TauriTypes.WishListItem>("wish_list_update", { input });
  }

  async delete(id: number): Promise<void> {
    return await this.client.sendInvoke<void>("wish_list_delete", { id });
  }
  async bought(entry: TauriTypes.BoughtWishListItem): Promise<TauriTypes.WishListItem> {
    return await this.client.sendInvoke<TauriTypes.WishListItem>("wish_list_bought", { ...entry });
  }

  async getById(id: number): Promise<TauriTypes.WishListItemDetails> {
    return await this.client.sendInvoke<TauriTypes.WishListItemDetails>("wish_list_get_by_id", { id });
  }
  exportJson = async (query: TauriTypes.WishListControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_wish_list_json", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}

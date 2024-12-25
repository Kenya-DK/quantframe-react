import { TauriClient } from "..";
import { StockItemModule } from "./item";
import { StockRivenModule } from "./riven";
import { WishListModule } from "./wish_list";
export class StockModule {
  constructor(client: TauriClient) {
    this.item = new StockItemModule(client);
    this.riven = new StockRivenModule(client);
    this.wishList = new WishListModule(client);
  }

  item: StockItemModule;
  riven: StockRivenModule;
  wishList: WishListModule;
}

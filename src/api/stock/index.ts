import { TauriClient } from "..";
import { StockItemModule } from "./item";
import { StockRivenModule } from "./riven";
export class StockModule {
  constructor(client: TauriClient) {
    this.item = new StockItemModule(client);
    this.riven = new StockRivenModule(client);
  }

  item: StockItemModule;
  riven: StockRivenModule;
}

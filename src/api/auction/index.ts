import { TauriClient } from "..";
import { AuctionCreate, AuctionUpdate } from "../types";

export class AuctionModule {
  constructor(private readonly client: TauriClient) { }

  getAuctions() {
    return this.client.sendInvoke("get_auctions");
  }

  createAuction(data: AuctionCreate) {
    return this.client.sendInvoke("create_auction", data);
  }

  updateAuction(id: string, data: AuctionUpdate) {
    return this.client.sendInvoke("update_auction", { id, data });
  }

  deleteAuction(id: string) {
    return this.client.sendInvoke("delete_auction", { id });
  }
}

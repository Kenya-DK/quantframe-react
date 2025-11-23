import { TauriClient } from "..";
import { WFMarketTypes } from "$types";
import { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import faPolarityZenuri from "../../icons/faPolarityZenuri";
import faPolarityUnairu from "../../icons/faPolarityUnairu";
import faPolarityUmbra from "../../icons/faPolarityUmbra";
import faPolarityPenjaga from "../../icons/faPolarityPenjaga";
import faPolarityNaramon from "../../icons/faPolarityNaramon";
import faPolarityMadurai from "../../icons/faPolarityMadurai";
import faPolarityAura from "../../icons/faPolarityAura";
import faPolarityVazarin from "../../icons/faPolarityVazarin";
import { faCross } from "@fortawesome/free-solid-svg-icons";
export class AuctionModule {
  constructor(private readonly client: TauriClient) {}
  async getPagination(query: WFMarketTypes.WfmAuctionControllerGetListParams): Promise<WFMarketTypes.WfmAuctionControllerGetListData> {
    return await this.client.sendInvoke<WFMarketTypes.WfmAuctionControllerGetListData>("get_wfm_auctions_pagination", { query });
  }
  async getOverview(query: WFMarketTypes.WfmAuctionControllerGetListParams): Promise<number[]> {
    return await this.client.sendInvoke<number[]>("get_wfm_auctions_overview", { query });
  }

  async refreshAuctions(): Promise<any> {
    return await this.client.sendInvoke<any>("auction_refresh");
  }

  async deleteAllAuctions(): Promise<any> {
    return await this.client.sendInvoke<any>("auction_delete_all");
  }
  async deleteById(id: string): Promise<any> {
    return await this.client.sendInvoke<any>("auction_delete_by_id", { id });
  }

  async importById(id: string, bought: number): Promise<any> {
    return await this.client.sendInvoke<any>("auction_import_by_id", { id, bought });
  }

  polarityToIcon(polarity: string): IconDefinition {
    switch (polarity) {
      case "zenuri":
        return faPolarityZenuri;
      case "unairu":
        return faPolarityUnairu;
      case "umbra":
        return faPolarityUmbra;
      case "penjaga":
        return faPolarityPenjaga;
      case "naramon":
        return faPolarityNaramon;
      case "madurai":
        return faPolarityMadurai;
      case "aura":
        return faPolarityAura;
      case "vazarin":
        return faPolarityVazarin;
      default:
        return faCross;
    }
  }
}

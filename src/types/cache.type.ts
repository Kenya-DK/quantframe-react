export namespace Cache {
  export interface CacheState {
    tradable_items: TradableItem[];
  }
  
  export interface TradableItem {
    name: string;
    uniqueName: string;
    description: string;
    wfm_id: string;
    wfm_url_name: string;
    trade_tax: number;
    mr_requirement: number;
    tags: string[];
    wiki_url: string;
    image_url: string;
    sub_types: any[];
  }

}
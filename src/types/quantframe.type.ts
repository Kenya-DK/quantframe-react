import { PaginatedDto } from ".";

export namespace QuantframeApiTypes {
  export interface AlertDto {
    /** The ID of the alert entry. */
    id: string;
    /**
     * The date and time when the alert was created.
     * @format date-time
     */
    created_at: string;
    /**
     * The date and time when the alert was last updated.
     * @format date-time
     */
    updated_at: string;
    /** The type of the alert. */
    type: string;
    /** The context of the alert. */
    context: string;
    /** If the alert is enabled or not. */
    enabled: boolean;
    /** The properties of the alert, if any. */
    properties?: object;
  }
  export interface AlertControllerGetListParams {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 1
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min 0
     * @max 100
     * @default 25
     */
    limit: number;
    sort_by?: number;
    /** Sort direction used when sorting by a specific field. */
    sort_direction?: "asc" | "desc";
    /** A search query to filter the users by name or email. */
    query?: string;
    /** Filter by enabled status. */
    enabled?: boolean;
  }

  /** PaginatedResponseOfAlertDto */
  export type AlertControllerGetListData = PaginatedDto & {
    results?: AlertDto[];
  };
  export interface ItemSubTypeDto {
    /** The rank of the item, if applicable. */
    rank?: number;
    /** The variant of the item, if applicable. */
    variant?: string;
    /** The number of amber stars, if applicable. */
    amber_stars?: number;
    /** The number of cyan stars, if applicable. */
    cyan_stars?: number;
  }
  export interface ItemPriceDto {
    /** The WFM ID of the item. */
    wfm_id: string;
    /** The URL of the item on Warframe Market. */
    wfm_url: string;
    /** The price of the item. */
    name?: string;
    /** The price of the item. */
    tags?: string[];
    /**
     * The date and time of the price record.
     * @format date-time
     */
    datetime: string;
    /** The trading volume of the item. */
    volume: number;
    /** The minimum price of the item. */
    min_price: number;
    /** The maximum price of the item. */
    max_price: number;
    /** The opening price of the item. */
    open_price: number;
    /** The closing price of the item. */
    closed_price: number;
    /** The average price of the item. */
    avg_price: number;
    /** The weighted average price of the item. */
    wa_price: number;
    /** The median price of the item. */
    median: number;
    /** The moving average price of the item, if applicable. */
    moving_avg?: number;
    /** The Donchian channel top value. */
    donch_top: number;
    /** The Donchian channel bottom value. */
    donch_bot: number;
    /** The type of order (e.g., buy, sell). */
    order_type: string;
    /** The trading tax for the item. */
    trading_tax: number;
    supply: number;
    demand: number;
    /** Details about the sub-type of the item, if applicable. */
    sub_type?: ItemSubTypeDto;
  }
  export interface ItemPriceControllerGetListParams {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 1
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min 0
     * @max 100
     * @default 25
     */
    limit: number;
    sort_by?: string;
    /** Sort direction used when sorting by a specific field. */
    sort_direction?: "asc" | "desc";
    /**
     * Select the start date.
     * @format date-time
     * @example "2025-05-18"
     */
    from_date?: string;
    /**
     * Select the end date.
     * @format date-time
     * @example "2025-05-18"
     */
    to_date?: string;
    /** Search for orders that contain this query. */
    query?: string;

    /**
     * Filter orders by item type. Use the item type name as the value. For example, "pistol,aa".
     * @example ["pistol","rifle"]
     */
    tags?: string[];
  }
  /** PaginatedResponseOfItemPriceDto */
  export type ItemPriceControllerGetListData = PaginatedDto & {
    results?: ItemPriceDto[];
  };
}

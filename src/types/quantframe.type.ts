export namespace QuantframeApiTypes {
  /* eslint-disable */
  /* tslint:disable */
  // @ts-nocheck
  /*
   * ---------------------------------------------------------------
   * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
   * ##                                                           ##
   * ## AUTHOR: acacode                                           ##
   * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
   * ---------------------------------------------------------------
   */

  export interface LatestReleaseDto {
    version: string;
    notes: string;
    pub_date: string;
    platforms: Record<string, PlatformDto>;
  }

  export interface PlatformDto {
    signature: string;
    url: string;
  }

  export interface AuthUserDto {
    /** The unique identifier of the user. */
    id: string;
    /** The name of the user. */
    name: string;
    /** Whether the user is banned or not. */
    banned: boolean;
    /** The reason the user was banned. */
    banned_reason?: string;
    /**
     * The date the user was banned until.
     * @format date-time
     */
    banned_until?: string;
    /** The permissions assigned to the user. */
    permissions?: string;
    /** The tier of the user on Patreon. */
    patreon_tier?: string;
  }

  export interface AuthLoginDto {
    /** The username or email of the user. */
    username: string;
    /** The password of the user. */
    password: string;
  }

  export interface PaginatedDto {
    /** The total number of items in the database */
    total: number;
    /** The number of items returned in this request */
    limit: number;
    /** The current page */
    page: number;
  }

  export interface CreateUserDto {
    /** The username of the user. */
    username: string;
    /** The password of the user. */
    password: string;
  }

  export interface UserDto {
    /** Unique identifier for the user */
    id: string;
    /** In-game name of the user */
    ingame_name: string;
    /** Indicates if the user is banned */
    banned: boolean;
    /** Reason for banning the user, if applicable */
    banned_reason: string;
    /** Current version of the user record */
    current_version: string;
    /**
     * Timestamp of the user's last activity
     * @format date-time
     */
    last_activity: string;
    /** Role of the user */
    role_name: string;
  }

  export interface OkMessageResponse {
    /** The message of the response. */
    message: string;
    /** The i18n key of the response. */
    i18n_key?: string;
    /** The i18n values of the response. */
    i18n_values?: object;
  }

  export interface UpdateUserDto {
    /** The unique identifier of the user. */
    id: string;
  }

  export interface UserBanDto {
    /** The reason the user was banned. */
    reason?: string;
    /**
     * The date the user was banned until.
     * @format date-time
     */
    until?: string;
  }

  export interface LogDto {
    /** The ID of the log entry. */
    id: string;
    /**
     * The date and time when the log entry was created.
     * @format date-time
     */
    created_at: string;
    /** The type of the log entry. */
    type: string;
    /** The context of the log entry. */
    context: string;
    /** The message of the log entry. */
    message: string;
  }

  export interface PaginationLogsDto {
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
    sort_by?: "type" | "context" | "created_at" | "message";
    /** Sort direction used when sorting by a specific field. */
    sort_direction?: "asc" | "desc";
    /** A search query to filter the users by name or email. */
    query?: string;
  }

  export interface ClearLogsBodyDto {
    /**
     * Delete logs from this date.
     * @format date-time
     * @example "2025-05-03"
     */
    from_date?: string;
    /**
     * Delete logs until this date.
     * @format date-time
     * @example "2025-05-03"
     */
    to_date?: string;
    /**
     * The type of logs to delete.
     * @example ["error","log","warn"]
     */
    types?: string[];
    /** The context of logs to delete. */
    context?: string[];
  }

  export interface LogDetailsDto {
    /** The ID of the log entry. */
    id: string;
    /**
     * The date and time when the log entry was created.
     * @format date-time
     */
    created_at: string;
    /** The type of the log entry. */
    type: string;
    /** The context of the log entry. */
    context: string;
    /** The message of the log entry. */
    message: string;
    /** The properties of the log entry, if any. */
    properties: object;
  }

  export interface SupplyDemandChartDto {
    /** An array of labels for the chart chart. */
    labels: string[];
    /** An array of numbers representing the supply values. */
    supply: number[];
    /** An array of numbers representing the demand values. */
    demand: number[];
  }

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
    /** Details about the sub-type of the item, if applicable. */
    sub_type?: ItemSubTypeDto;
  }

  export interface PaginationPriceQueryDto {
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
    sort_by?: "volume" | "min_price" | "max_price" | "open_price";
    /** Sort direction used when sorting by a specific field. */
    sort_direction?: "asc" | "desc";
    /**
     * Select the start date.
     * @format date-time
     * @example "2025-05-03"
     */
    from_date: string;
    /**
     * Select the end date.
     * @format date-time
     * @example "2025-05-03"
     */
    to_date: string;
    /** Search for orders that contain this query. */
    query?: string;
    /** Filter orders by type (e.g., buy, sell, or closed). */
    order_type?: "buy" | "sell" | "closed";
    /**
     * Filter orders by item type. Use the item type name as the value. For example, "pistol,aa".
     * @example ["pistol","rifle"]
     */
    tags?: string[];
  }

  export interface ChartDto {
    /** An array of numbers representing the chart values. */
    values: string[];
    /** An array of labels for the chart chart. */
    labels: string[];
  }

  export interface ItemPriceOverviewDto {
    /** The URL of the item on Warframe Market. */
    most_traded: ChartDto;
    /** The URL of the item on Warframe Market. */
    profit_margin: ChartDto;
    /** The URL of the item on Warframe Market. */
    return_on_investment: ChartDto;
    /** The URL of the item on Warframe Market. */
    supply_and_demand: SupplyDemandChartDto;
  }

  export interface RoleDto {
    /** The unique identifier of the role. */
    id?: string;
    /**
     * The date the role was created.
     * @format date-time
     */
    created_at: string;
    /**
     * The date the role was last updated.
     * @format date-time
     */
    updated_at: string;
    /** The name of the role. */
    name: string;
    /** Whether the role can be managed or not. */
    can_managed: boolean;
    /** The permissions assigned to the role. */
    permissions: string;
  }

  export interface CreateRoleDto {
    /** The name of the role. */
    name: string;
    /** Whether the role can be managed or not. */
    can_managed: boolean;
    /** The permissions assigned to the role. */
    permissions: string;
  }

  export interface UpdateRoleDto {
    /** The name of the role. */
    name: string;
    /** Whether the role can be managed or not. */
    can_managed: boolean;
    /** The permissions assigned to the role. */
    permissions: string;
  }

  export interface CreateAlertDto {
    /** Type of the alert */
    type: "error" | "warning" | "success" | "info";
    /** Unique name of the item involved in the transaction */
    context: string;
    /** If the alert is enabled or not. */
    enabled: boolean;
  }

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

  export interface UpdateAlertDto {
    /** The ID of the alert entry. */
    id: string;
    /** The type of the alert. */
    type: string;
    /** The context of the alert. */
    context: string;
    /** If the alert is enabled or not. */
    enabled: boolean;
    /** The properties of the alert, if any. */
    properties?: object;
  }

  export type AppControllerCheckReleaseData = LatestReleaseDto;

  export type AppControllerCheckPreReleaseData = LatestReleaseDto;

  export type AuthControllerLoginData = AuthUserDto;

  export type AuthControllerLogoutData = any;

  export interface AuthControllerLinkAccParams {
    state: string;
    code: string;
  }

  export type AuthControllerLinkAccData = any;

  export type AuthControllerCurrentUserData = AuthUserDto;

  export type MetricsControllerUpdateLastActivePayload = string[];

  export type MetricsControllerUpdateLastActiveData = any;

  export interface UserControllerGetListParams {
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
    /** A search query to filter the users by name or email. */
    query?: string;
    /** A search query to filter the users by banned. */
    banned?: boolean;
    /** A search query to filter the users by role. */
    role?: string;
    /** A search query to filter the users by active for the last 5 min. */
    is_active?: boolean;
  }

  /** PaginatedResponseOfUserDto */
  export type UserControllerGetListData = PaginatedDto & {
    results?: UserDto[];
  };

  export type UserControllerCreateUserData = AuthUserDto;

  export type UserControllerUpdateUserData = OkMessageResponse;

  export type UserControllerDeleteUserData = OkMessageResponse;

  export type UserControllerBanUserByIdData = OkMessageResponse;

  export type UserControllerUnBanUserByIdData = OkMessageResponse;

  export interface WfmControllerGetUserActiveHistoryParams {
    /**
     * The item name.
     * @format date-time
     * @example "2025-05-03"
     */
    from_date: string;
    /**
     * To date.
     * @format date-time
     * @example "2025-05-03"
     */
    to_date: string;
    /** The weapon type. */
    group_by: "day" | "hour";
  }

  export type WfmControllerGetUserActiveHistoryData = any;

  export interface LogControllerGetListParams {
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
    sort_by?: "type" | "context" | "created_at" | "message";
    /** Sort direction used when sorting by a specific field. */
    sort_direction?: "asc" | "desc";
    /** A search query to filter the users by name or email. */
    query?: string;
  }

  /** PaginatedResponseOfLogDto */
  export type LogControllerGetListData = PaginatedDto & {
    results?: LogDto[];
  };

  export type LogControllerClearLogsData = OkMessageResponse;

  export type LogControllerDeleteLogData = OkMessageResponse;

  export type LogControllerGetLogData = LogDetailsDto;

  export type ItemControllerDownloadOldCacheData = any;

  export type ItemControllerGetCacheOldMd5Data = string;

  export type ItemControllerGetPriceOverviewData = ItemPriceOverviewDto;

  export interface ItemControllerGetListParams {
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
    sort_by?: "volume" | "min_price" | "max_price" | "open_price";
    /** Sort direction used when sorting by a specific field. */
    sort_direction?: "asc" | "desc";
    /**
     * Select the start date.
     * @format date-time
     * @example "2025-05-03"
     */
    from_date: string;
    /**
     * Select the end date.
     * @format date-time
     * @example "2025-05-03"
     */
    to_date: string;
    /** Search for orders that contain this query. */
    query?: string;
    /** Filter orders by type (e.g., buy, sell, or closed). */
    order_type?: "buy" | "sell" | "closed";
    /**
     * Filter orders by item type. Use the item type name as the value. For example, "pistol,aa".
     * @example ["pistol","rifle"]
     */
    tags?: string[];
  }

  /** PaginatedResponseOfItemPriceDto */
  export type ItemControllerGetListData = PaginatedDto & {
    results?: ItemPriceDto[];
  };

  export interface RoleControllerGetListParams {
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
    sort_by?: "name" | "permissions" | "created_at";
    /** Sort direction used when sorting by a specific field. */
    sort_direction?: "asc" | "desc";
    /** A search query to filter the roles by name. */
    query?: string;
  }

  /** PaginatedResponseOfRoleDto */
  export type RoleControllerGetListData = PaginatedDto & {
    results?: RoleDto[];
  };

  export type RoleControllerCreateRoleData = OkMessageResponse;

  export type RoleControllerDeleteRoleData = OkMessageResponse;

  export type RoleControllerGetRoleData = RoleDto;

  export type RoleControllerUpdateRoleData = OkMessageResponse;

  export interface RivenControllerGetRivenListParams {
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
    /**
     * Start date for filtering data.
     * @format date-time
     * @example "2025-05-03"
     */
    from_date: string;
    /**
     * End date for filtering data.
     * @format date-time
     * @example "2025-05-03"
     */
    to_date: string;
    /**
     * The weapon url.
     * @example "torid"
     */
    weapon_url?: string;
    /**
     * If the riven is rolled or not.
     * @example true
     */
    rolled?: boolean;
  }

  /** PaginatedResponseOfRivenPriceDto */
  export type RivenControllerGetRivenListData = PaginatedDto & {
    results?: any[];
  };

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

  export type AlertControllerCreateData = any;

  export type AlertControllerDeleteLogData = AlertDto;

  export type AlertControllerGetLogData = AlertDto;

  export type AlertControllerUpdateLogData = AlertDto;

  export type CacheControllerDownloadCacheData = any;

  export type CacheControllerGetCacheMd5Data = string;
}

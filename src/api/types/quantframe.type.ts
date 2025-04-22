export namespace QFApiTypes {
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
    /** The properties of the log entry, if any. */
    properties?: object;
  }

  export interface PaginationLogsDto {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 0
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min -1
     * @max 100
     * @default 25
     */
    limit: number;
    /** A search query to filter the users by name or email. */
    query?: string;
  }

  export interface DeleteLogBatchDto {
    /** An array of IDs of the logs to delete. */
    logs: string[];
  }

  export interface SyndicatesPriceDto {
    /**
     * The standing value associated with the object
     * @example 100000
     */
    standing: number;
    /**
     * The name of the object or area
     * @example "Central Mall Backroom"
     */
    name: string;
    /**
     * The unique identifier for WFM
     * @example "675c61997b18977f6e64540c"
     */
    wfm_id: string;
    /**
     * The URL-friendly identifier for WFM
     * @example "central_mall_backroom"
     */
    wfm_url_name: string;
    /**
     * The minimum price recorded
     * @example 50
     */
    min_price: number;
    /**
     * The maximum price recorded
     * @example 60
     */
    max_price: number;
    /**
     * The average price calculated
     * @example 55
     */
    avg_price: number;
    /**
     * The date and time the data was recorded
     * @example "2025-01-26T00:00:00.000+00:00"
     */
    datetime: string;
    /**
     * The sub-type of the object
     * @example {"rank":5}
     */
    sub_type?: object;
  }

  export interface SupplyDemandChartDto {
    /** An array of labels for the chart chart. */
    labels: string[];
    /** An array of numbers representing the supply values. */
    supply: number[];
    /** An array of numbers representing the chart values. */
    values: number[];
    /** An array of numbers representing the demand values. */
    demand: number[];
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
    permissions: (
      | "all"
      | "dashboard_access"
      | "user_delete"
      | "user_edit"
      | "user_view"
      | "user_ban"
      | "user_metrics"
      | "role_add"
      | "role_delete"
      | "role_edit"
      | "role_view"
      | "log_view"
      | "log_delete"
      | "metrics_view"
      | "transaction_view"
      | "api_key_create"
      | "riven_sniper_get_user"
      | "riven_sniper_view"
      | "riven_sniper_edit"
      | "riven_sniper_delete"
      | "riven_sniper_add"
      | "alert_delete"
      | "alert_create"
      | "item_generate_price"
      | "riven_price_history"
      | "wfm_users_active_history"
    )[];
  }

  export interface UpdateRoleDto {
    /** The name of the role. */
    name: string;
    /** Whether the role can be managed or not. */
    can_managed: boolean;
    /** The permissions assigned to the role. */
    permissions: (
      | "all"
      | "dashboard_access"
      | "user_delete"
      | "user_edit"
      | "user_view"
      | "user_ban"
      | "user_metrics"
      | "role_add"
      | "role_delete"
      | "role_edit"
      | "role_view"
      | "log_view"
      | "log_delete"
      | "metrics_view"
      | "transaction_view"
      | "api_key_create"
      | "riven_sniper_get_user"
      | "riven_sniper_view"
      | "riven_sniper_edit"
      | "riven_sniper_delete"
      | "riven_sniper_add"
      | "alert_delete"
      | "alert_create"
      | "item_generate_price"
      | "riven_price_history"
      | "wfm_users_active_history"
    )[];
    /** The unique identifier of the role. */
    id?: string;
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

  export interface MetricsOverviewDto {
    /** Chat showing the total number of metrics */
    metrics_chart: ChartDto;
    /** The total number of unique users */
    unique_users: number;
  }

  export interface MetricsDetailDto {
    /** Quantity of the metric events */
    quantity: number;
    /** Chat showing the total number of metrics */
    metrics_chart: ChartDto;
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

  export interface UserControllerGetListParams {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 0
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min -1
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
     * @example "2025-04-22"
     */
    from_date: string;
    /**
     * To date.
     * @format date-time
     * @example "2025-04-22"
     */
    to_date: string;
    /** The weapon type. */
    group_by: "day" | "hour";
  }

  export type WfmControllerGetUserActiveHistoryData = any;

  export interface LogControllerGetListParams {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 0
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min -1
     * @max 100
     * @default 25
     */
    limit: number;
    /** A search query to filter the users by name or email. */
    query?: string;
  }

  /** PaginatedResponseOfLogDto */
  export type LogControllerGetListData = PaginatedDto & {
    results?: LogDto[];
  };

  export type LogControllerDeleteLogsData = LogDto;

  export type LogControllerDeleteLogData = LogDto;

  export type LogControllerGetLogData = LogDto;

  export type ItemControllerDownloadOldCacheData = any;

  export type ItemControllerGetCacheOldMd5Data = string;

  export type ItemControllerGetPriceOverviewData = ItemPriceOverviewDto;

  export interface RoleControllerGetListParams {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 0
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min -1
     * @max 100
     * @default 25
     */
    limit: number;
    /** A search query to filter the roles by name. */
    query?: string;
  }

  /** PaginatedResponseOfRoleDto */
  export type RoleControllerGetListData = PaginatedDto & {
    results?: RoleDto[];
  };

  export type RoleControllerCreateRoleData = RoleDto;

  export type RoleControllerDeleteRoleData = RoleDto;

  export type RoleControllerGetRoleData = RoleDto;

  export type RoleControllerUpdateRoleData = RoleDto;

  export interface RivenControllerGetRivenListParams {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 0
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min -1
     * @max 100
     * @default 25
     */
    limit: number;
    /**
     * Start date for filtering data.
     * @format date-time
     * @example "2025-04-22"
     */
    from_date: string;
    /**
     * End date for filtering data.
     * @format date-time
     * @example "2025-04-22"
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
     * @min 0
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min -1
     * @max 100
     * @default 25
     */
    limit: number;
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

  export interface MetricsControllerGetListParams {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 0
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min -1
     * @max 100
     * @default 25
     */
    limit: number;
    /** A search query to filter the users by name or email. */
    query?: string;
  }

  /** PaginatedResponseOfMetricsDto */
  export type MetricsControllerGetListData = PaginatedDto & {
    results?: any[];
  };

  export type MetricsControllerUpdateLastActivePayload = string[];

  export type MetricsControllerUpdateLastActiveData = any;

  export type MetricsControllerGetOverviewData = MetricsOverviewDto;

  export type MetricsControllerGetMetricsDetailsData = MetricsDetailDto;

  export type MetricsControllerCombieData = any;
}

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
}

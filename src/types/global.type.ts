// export type DeepPartial<T> = T extends object
//   ? {
//       [P in keyof T]?: DeepPartial<T[P]>;
//     }
//   : T;
// export type ErrOrResult<RES> = [ResponseError, null] | [null, RES] | [ResponseError, undefined] | [undefined, RES];

export interface ResponseError extends Error {
  component: string;
  message: string;
  location: string;
  cause?: string;
  context: Record<string, any>;
  log_level: string;
}
export interface SubType {
  rank?: number;
  variant?: string;
  amber_stars?: number;
  cyan_stars?: number;
}
export interface PaginatedDto {
  /** The total number of items in the database */
  total: number;
  /** The number of items returned in this request */
  limit: number;
  /** The current page */
  page: number;
}
export enum UserStatus {
  Online = "online",
  Invisible = "invisible",
  Ingame = "ingame",
}
export interface MinMaxDto {
  min: number;
  max?: number;
}
export interface Paginated<T> {
  total: number;
  limit: number;
  page: number;
  results: T[];
}

export interface PriceHistory {
  created_at: Date;
  name: string;
  price: number;
  user_id: string;
}
export interface RivenAttribute {
  positive: boolean;
  url_name: string;
  value: number;
  effect?: string;
}

export type ErrOrResult<RES> = [ResponseError, null] | [null, RES] | [ResponseError, undefined] | [undefined, RES];

export interface ResponseError extends Error {
  backtrace: string;
  cause: string;
  component: string;
  extra_data: Record<string, any>;
  log_level: string;
}

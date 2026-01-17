
export * from "./wfm.type";
export * from "./sorting.type";

export type DeepPartial<T> = T extends object ? {
  [P in keyof T]?: DeepPartial<T[P]>;
} : T;

export * from "./settings.type";
export * from "./wfm.type";
export * from "./database.type";
export * from "./sorting.type";
export * from "./search.type";
export * from "./statistic.type";
export * from "./progressReport.type";

export type DeepPartial<T> = T extends object ? {
  [P in keyof T]?: DeepPartial<T[P]>;
} : T;
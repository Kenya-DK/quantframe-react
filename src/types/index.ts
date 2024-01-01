
export * from "./settings.type";
export * from "./wfm.type";
export * from "./database.type";
export * from "./sorting.type";
export * from "./search.type";
export * from "./statistic.type";

export interface WeeklyRiven {
  itemType: string;
  compatibility: null;
  rerolled: boolean;
  avg: number;
  stddev: number;
  min: number;
  max: number;
  pop: number;
  median: number;
}

export type DeepPartial<T> = T extends object ? {
  [P in keyof T]?: DeepPartial<T[P]>;
} : T;

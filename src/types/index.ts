
export * from "./settings.type";
export * from "./wfm.type";
export * from "./database.type";
export * from "./sorting.type";


export interface ChartWithValuesDto {
  values: Array<number>;
}
export interface ChartWithLabelsDto {
  labels: Array<string>;
}
export interface ChartDto extends ChartWithValuesDto, ChartWithLabelsDto {

}

// Handle Items Statistic
export interface StatisticTransactionItemRevenue {
  wfm_id: string;
  url: string;
  item_type: string;
  name: string;
  tags: string[];
  total_bought: number;
  total_sold: number;
  quantity: number;
  price: number;
  turnover: number;
}

export interface StatisticTransactionPopularItems {
  buy: StatisticTransactionItemRevenue[];
  sell: StatisticTransactionItemRevenue[];
}
//End Handle Items Statistic
export interface StatisticPreviousPresent extends ChartWithLabelsDto {
  previous: StatisticTotalTransactionBuyAndSell;
  present: StatisticTotalTransactionBuyAndSell;
}


export interface StatisticTransactionRevenue {
  average: number;
  quantity: number;
  revenue: number;
}
export interface StatisticTransactionRevenueWithChart extends StatisticTransactionRevenue {
  quantity_chart: number[];
  revenue_chart: number[];
}

export interface StatisticTotalTransactionBuyAndSell extends ChartWithLabelsDto {
  sales: StatisticTransactionRevenueWithChart;
  buy: StatisticTransactionRevenueWithChart;
  popular_items: StatisticTransactionPopularItems;
}

export interface StatisticTotalTransaction extends StatisticPreviousPresent {
  // Remove revenue_chart and quantity_chart from
  sales: StatisticTransactionRevenue,
  buy: StatisticTransactionRevenue;
}
export interface StatisticTodayTransaction extends StatisticTotalTransactionBuyAndSell {

}

export interface StatisticRecentDaysTransaction extends StatisticTotalTransactionBuyAndSell {
  days: number;
}

export interface StatisticDto {
  total: StatisticTotalTransaction;
  today: StatisticTodayTransaction;
  recent_days: StatisticRecentDaysTransaction;
  turnover: number;
}
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
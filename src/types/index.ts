
export * from "./settings.type";
export * from "./wfm.type";
export * from "./constants";
export * from "./database.type";



export interface ChartWithValuesDto {
  values: Array<number>;
}
export interface ChartWithLabelsDto {
  labels: Array<string>;
}
export interface ChartDto extends ChartWithValuesDto, ChartWithLabelsDto {

}



export interface StatisticPreviousPresent extends ChartWithLabelsDto {
  previous: StatisticTotalTransactionBuyAndSell;
  present: StatisticTotalTransactionBuyAndSell;
}


export interface StatisticTransactionRevenue {
  average: number;
  quantity: number;
  revenue: number;
  best_sellers: Array<StatisticTransactionBestSeller>;
}
export interface StatisticTransactionRevenueWithChart extends StatisticTransactionRevenue {
  quantity_chart: number[];
  revenue_chart: number[];
}


export interface StatisticTransactionBestSeller {
  item_id: string;
  item_url: string;
  item_type: string;
  item_name: string;
  quantity: number;
  revenue: number;
}
export interface StatisticTotalTransactionBuyAndSell extends ChartWithLabelsDto {
  sales: StatisticTransactionRevenueWithChart;
  buy: StatisticTransactionRevenueWithChart;
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

export type DeepPartial<T> = T extends object ? {
  [P in keyof T]?: DeepPartial<T[P]>;
} : T;
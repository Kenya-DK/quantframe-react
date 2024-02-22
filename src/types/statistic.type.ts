import { TransactionEntryDto } from ".";

// Base Statistic Type
export interface ChartWithValuesDto {
  values: Array<number>;
}
export interface ChartWithMultipleValuesDto {
  values: Array<Array<number>>;
}
export interface ChartWithLabelsDto {
  labels: Array<string>;
}
export interface ChartDto extends ChartWithValuesDto, ChartWithLabelsDto {

}
export interface ChartMultipleDto extends ChartWithMultipleValuesDto, ChartWithLabelsDto {

}
export interface StatisticTransactionPreviousPresent extends ChartWithLabelsDto {
  previous: StatisticProfitTransaction;
  present: StatisticProfitTransaction;
}
export interface StatisticProfitBase {
  profit: number;
  profit_margin: number;
  average_revenue: number;
  purchases: number;
  sales: number;
  expense: number;
  revenue: number;
}

export interface StatisticProfitTransaction extends StatisticProfitBase {
  number_of_trades: number;
  popular_items: StatisticProfitItem[];
}

export interface StatisticChartTransactionAndItem extends StatisticProfitTransaction {
  chart_profit: ChartMultipleDto;
  chart_items: ChartMultipleDto;
}

export interface StatisticProfitTransactionPreviousPresent extends ChartWithLabelsDto {
  previous: StatisticProfitTransaction & ChartWithMultipleValuesDto;
  present: StatisticProfitTransaction & ChartWithMultipleValuesDto;
}

export interface StatisticProfitItem extends StatisticProfitBase {
  wfm_id: string;
  url: string;
  item_type: string;
  name: string;
  tags: string[];
  quantity: number;
}
export interface StatisticItemCategoryProfit extends StatisticProfitBase {
  name: string;
  icon: string;
  quantity: number;
}

// Statistic Type 
export interface StatisticItemBestSeller {
  items: StatisticProfitItem[];
  items_chart: ChartMultipleDto;
  categorys: StatisticItemCategoryProfit[];
  category_chart: ChartMultipleDto
}
export interface StatisticProfitTransactionTotal extends StatisticProfitTransactionPreviousPresent, StatisticProfitBase {

}

export interface StatisticProfitTransactionToday extends StatisticChartTransactionAndItem {

}

// export interface StatisticItemCategoryProfit extends StatisticProfitBase {
export interface StatisticRecentTransactions extends StatisticProfitBase {
  transactions: TransactionEntryDto[];
}
export interface StatisticProfitTransactionRecentDays extends StatisticChartTransactionAndItem {
  days: number;
}
export interface CategoryItemProfitLink {
  name: string;
  icon: string;
  tags: string[];
  types: string[];
}
export interface StatisticDto {
  best_seller: StatisticItemBestSeller;
  total: StatisticProfitTransactionTotal;
  today: StatisticProfitTransactionToday;
  recent_days: StatisticProfitTransactionRecentDays;
  recent_transactions: StatisticRecentTransactions;
}
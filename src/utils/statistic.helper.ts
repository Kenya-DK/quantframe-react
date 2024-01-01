import dayjs from "dayjs";
import isBetween from 'dayjs/plugin/isBetween';
dayjs.extend(isBetween);
import { groupBy, getGroupByDate, GroupByDateSettings } from ".";
import { TransactionEntryDto, StatisticProfitItem, StatisticProfitTransaction, StatisticProfitTransactionTotal, StatisticProfitTransactionToday, StatisticProfitTransactionRecentDays, StatisticDto, ChartMultipleDto, CategoryItemProfitLink, StatisticItemCategoryProfit, StatisticItemBestSeller } from "../types";
import i18next from "i18next";

// This function splits the given array of transactions into two arrays: one for buy transactions and one for sell transactions.
// It takes an array of TransactionEntryDto objects as input and returns a tuple of two arrays.
export const SplitTransactionType = (transactions: TransactionEntryDto[]): [TransactionEntryDto[], TransactionEntryDto[]] => {
  // The 'filter' method is used to create a new array with all elements that pass the test implemented by the provided function.
  // Here, it's used to create two new arrays: one with all transactions where 'transaction_type' is "buy", and one where 'transaction_type' is "sell".
  return [transactions.filter(t => t.transaction_type == "buy"), transactions.filter(t => t.transaction_type == "sell")];
}

/**
 * This function calculates the profit for each item in a list of transactions and returns it in a format suitable for charting.
 * 
 * @param {TransactionEntryDto[]} transactions - The list of transactions to calculate profits from.
 * 
 * @returns {ChartMultipleDto} - An object containing labels for each item and a 2D array of values. 
 * The values array contains four sub-arrays, each corresponding to sales, purchases, quantity, and profit for each item, respectively.
 */
export const GetItemChartProfit = (transactions: TransactionEntryDto[]): ChartMultipleDto => {
  const items = GetItemsProfit(transactions);
  const labels = items.map((item: StatisticProfitItem) => item.name);
  const groups = groupBy("name", items);
  const sales_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticProfitItem) => item.quantity).reduce((acc, cur) => acc + cur, 0) : 0);
  const purchases_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticProfitItem) => item.quantity).reduce((acc, cur) => acc + cur, 0) : 0);
  const quantity_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticProfitItem) => item.quantity).reduce((acc, cur) => acc + cur, 0) : 0);
  const profit_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticProfitItem) => item.profit).reduce((acc, cur) => acc + cur, 0) : 0);
  return {
    labels: labels,
    values: [
      sales_chart,
      purchases_chart,
      quantity_chart,
      profit_chart
    ]
  };
};
/**
 * This function calculates the profit for each transaction in a list and returns it in a format suitable for charting.
 * 
 * @param {TransactionEntryDto[]} transactions - The list of transactions to calculate profits from.
 * @param {GroupByDateSettings} settings - The settings for grouping transactions by date.
 * 
 * @returns {ChartMultipleDto} - An object containing labels for each transaction and a 2D array of values. 
 * The values array contains four sub-arrays, each corresponding to sales, purchases, quantity, and profit for each transaction, respectively.
 */
export const GetTransactionChartProfit = (transactions: TransactionEntryDto[], settings: GroupByDateSettings): ChartMultipleDto => {
  const [groups, labels] = getGroupByDate<TransactionEntryDto>("created", transactions, settings);
  const purchases_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].filter(t => t.transaction_type == "buy").length : 0);
  const sales_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].filter(t => t.transaction_type == "sell").length : 0);
  const quantity_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].length : 0);
  const profit_chart = labels.map((label: string) => groups[`${label}`] ? GetTransactionProfit(groups[`${label}`]).profit : 0);

  return {
    labels: labels,
    values: [
      sales_chart,
      purchases_chart,
      quantity_chart,
      profit_chart
    ]
  };
};
/**
 * This function calculates various statistics related to the profit from a list of transactions.
 * 
 * @param {TransactionEntryDto[]} transactions - The list of transactions to calculate profits from.
 * 
 * @returns {StatisticProfitTransaction} - An object containing various statistics related to the transactions, including total profit, profit margin, 
 * average revenue per transaction, total number of trades, total expense, total revenue, number of purchases, number of sales, and a list of popular items.
 */
const GetTransactionProfit = (transactions: TransactionEntryDto[]): StatisticProfitTransaction => {
  const [buy_transactions, sell_transactions] = SplitTransactionType(transactions);
  const expense = buy_transactions.reduce((acc, cur) => acc + cur.price, 0);
  const revenue = sell_transactions.reduce((acc, cur) => acc + cur.price, 0);
  const profit = revenue - expense;
  return {
    profit: profit,
    profit_margin: profit / revenue,
    average_revenue: revenue / transactions.length,
    number_of_trades: transactions.length,
    expense: expense,
    revenue: revenue,
    purchases: buy_transactions.length,
    sales: sell_transactions.length,
    popular_items: GetItemsProfit(transactions)
  };
};
/**
 * This function calculates various statistics related to the profit from a list of transactions for each unique item.
 * 
 * @param {TransactionEntryDto[]} transactions - The list of transactions to calculate profits from.
 * 
 * @returns {StatisticProfitItem[]} - An array of objects, each containing various statistics related to the transactions for a unique item. 
 * These statistics include total profit, profit margin, average revenue per transaction, total number of trades, total expense, total revenue, 
 * number of purchases, number of sales, Warframe Market ID, URL, item type, name, tags, and total quantity.
 */
const GetItemsProfit = (transactions: TransactionEntryDto[]): StatisticProfitItem[] => {
  let items: StatisticProfitItem[] = [];
  let transactionsgroupBy = groupBy("url", transactions);
  for (let item in transactionsgroupBy) {
    // Get the order
    let transactionList = transactionsgroupBy[item];

    let firstTransaction = transactionList[0];
    const [buy_transactions, sell_transactions] = SplitTransactionType(transactionList);
    const expense = buy_transactions.reduce((acc, cur) => acc + cur.price, 0);
    const revenue = sell_transactions.reduce((acc, cur) => acc + cur.price, 0);
    const profit = revenue - expense;

    let trans = {
      profit: profit,
      profit_margin: profit / revenue,
      average_revenue: revenue / transactions.length,
      number_of_trades: transactions.length,
      expense: expense,
      revenue: revenue,
      purchases: buy_transactions.length,
      sales: sell_transactions.length,
      wfm_id: firstTransaction.wfm_id,
      url: firstTransaction.url,
      item_type: firstTransaction.item_type,
      name: firstTransaction.name,
      tags: firstTransaction.tags.split(","),
      quantity: transactionList.reduce((acc, cur) => acc + cur.quantity, 0),
    };
    items.push(trans);
  }
  return items.sort((a, b) => b.profit - a.profit);
};

/**
 * This function calculates profit statistics for each category of items from a list of transactions.
 * 
 * @param {TransactionEntryDto[]} transactionsIn - The list of transactions to calculate profits from.
 * @param {CategoryItemProfitLink[]} categorys - The list of categories to group items into.
 * 
 * @returns {StatisticItemCategoryProfit[]} - An array of objects, each containing profit statistics for a unique category. 
 * These statistics include total profit, profit margin, average revenue per transaction, total number of trades, total expense, total revenue, 
 * number of purchases, number of sales, and a list of popular items. Each object also includes the category's icon and name.
 */
export const GetItemCategoryProfit = (transactionsIn: TransactionEntryDto[], categorys: CategoryItemProfitLink[]): StatisticItemCategoryProfit[] => {
  let transactions = [...transactionsIn];
  let items: StatisticItemCategoryProfit[] = [];
  for (let index = 0; index < categorys.length; index++) {
    const category = categorys[index];
    // Get the order
    let transactionList = transactions.filter(t => t.tags.split(",").some(r => category.tags.includes(r)) || category.types.includes(t.item_type));

    let trans = {
      ...GetTransactionProfit(transactionList),
      ...category,
      quantity: transactionList.reduce((acc, cur) => acc + cur.quantity, 0),
    };
    items.push(trans);
    // Remove the transactions from the list
    transactions = transactions.filter(t => !transactionList.includes(t));
  }
  let trans = {
    ...GetTransactionProfit(transactions),
    icon: "other.png",
    name: "Other",
    quantity: transactions.reduce((acc, cur) => acc + cur.quantity, 0),
  };
  items.push(trans);
  return items.sort((a, b) => b.profit - a.profit);
};

export const GetItemCategoryChartProfit = (transactions: TransactionEntryDto[], categorys: CategoryItemProfitLink[]): ChartMultipleDto => {
  const items = GetItemCategoryProfit(transactions, categorys);
  const labels = items.map((item: StatisticItemCategoryProfit) => item.name);
  const groups = groupBy("name", items);
  const sales_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticItemCategoryProfit) => item.sales).reduce((acc, cur) => acc + cur, 0) : 0);
  const purchases_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticItemCategoryProfit) => item.purchases).reduce((acc, cur) => acc + cur, 0) : 0);
  const quantity_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticItemCategoryProfit) => item.sales).reduce((acc, cur) => acc + cur, 0) : 0);
  const profit_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticItemCategoryProfit) => item.profit).reduce((acc, cur) => acc + cur, 0) : 0);
  return {
    labels: labels,
    values: [
      sales_chart,
      purchases_chart,
      quantity_chart,
      profit_chart
    ]
  };
};


export const GetTotalProfit = (transactions: TransactionEntryDto[]): StatisticProfitTransactionTotal => {

  // Create Chart Data
  const thisYearLabels = [];
  const lastYearLabels = [];
  const year = new Date().getFullYear();
  for (let i = 0; i < 12; i++) {
    thisYearLabels.push(`${i} ${year}`);
    lastYearLabels.push(`${i} ${year - 1}`);
  }

  const thisYearTransactions = transactions.filter(t => dayjs(t.created).isSame(new Date(), "year"));
  const lastYearTransactions = transactions.filter(t => dayjs(t.created).isSame(dayjs().subtract(1, "year"), "year"));

  const byDateSettings: GroupByDateSettings = { year: true, month: true, day: false, hours: false }
  return {
    ...GetTransactionProfit(transactions),
    labels: i18next.t("general.months", { returnObjects: true }) as string[],
    present: {
      ...GetTransactionProfit(thisYearTransactions),
      values: GetTransactionChartProfit(thisYearTransactions, { ...byDateSettings, labels: thisYearLabels }).values,
    },
    previous: {
      ...GetTransactionProfit(lastYearTransactions),
      values: GetTransactionChartProfit(lastYearTransactions, { ...byDateSettings, labels: lastYearLabels }).values,
    },
  };
};

export const GetToDayProfit = (transactions: TransactionEntryDto[]): StatisticProfitTransactionToday => {
  let today = dayjs().startOf("day").toDate();
  let endToday = dayjs().endOf('day').toDate();
  const labels = [];
  for (let i = 0; i < 24; i++) labels.push(`${i}:00`);

  transactions = transactions.filter(t => dayjs(t.created).isBetween(today, endToday));
  return {
    ...GetTransactionProfit(transactions),
    chart_profit: { ...GetTransactionChartProfit(transactions, { year: false, month: false, day: false, hours: true, labels: labels }) },
    chart_items: GetItemChartProfit(transactions)
  };
};

export const GetRecentDaysProfit = (transactions: TransactionEntryDto[], days: number): StatisticProfitTransactionRecentDays => {
  let today = dayjs().subtract(days, "day").endOf('day').toDate();
  let endToday = dayjs().endOf('day').toDate();
  transactions = transactions.filter(t => dayjs(t.created).isBetween(today, endToday));
  const labels = [];
  const date = new Date();
  date.setDate(date.getDate() - (days - 1));
  for (let i = 0; i < days; i++) {
    labels.push(`${date.getDate()} ${date.getMonth()} ${date.getFullYear()}`);
    date.setDate(date.getDate() + 1);
  }
  return {
    ...GetTransactionProfit(transactions),
    chart_profit: { ...GetTransactionChartProfit(transactions, { month: true, day: true, year: true, labels: labels }) },
    chart_items: GetItemChartProfit(transactions),
    days
  };
};
export const GetBestSeller = (transactions: TransactionEntryDto[]): StatisticItemBestSeller => {
  const categorys = [
    {
      icon: "https://warframe.market/static/assets/5e9c9c6e/img/warframes/ash.png",
      name: "Mod",
      tags: ["mod"],
      types: []
    },
    {
      icon: "https://warframe.market/static/assets/5e9c9c6e/img/warframes/ash.png",
      name: "Arcane",
      tags: ["arcane_enhancement"],
      types: []
    },
    {
      icon: "https://warframe.market/static/assets/5e9c9c6e/img/warframes/ash.png",
      name: "Set",
      tags: ["set"],
      types: []
    },
    {
      icon: "https://warframe.market/static/assets/5e9c9c6e/img/warframes/ash.png",
      name: "Prime",
      tags: ["prime"],
      types: []
    },
    {
      icon: "https://warframe.market/static/assets/5e9c9c6e/img/warframes/ash.png",
      name: "Relic",
      tags: ["relic"],
      types: []
    },
    {
      icon: "https://warframe.market/static/assets/5e9c9c6e/img/warframes/ash.png",
      name: "Riven",
      tags: [],
      types: ["riven"]
    }
  ];

  return {
    items: GetItemsProfit(transactions),
    items_chart: { ...GetItemChartProfit(transactions) },
    categorys: GetItemCategoryProfit(transactions, categorys),
    category_chart: GetItemCategoryChartProfit(transactions, categorys),
  };
};
export const GetStatistic = (transactions: TransactionEntryDto[]): StatisticDto => {
  return {
    best_seller: GetBestSeller(transactions),
    total: GetTotalProfit(transactions),
    today: GetToDayProfit(transactions),
    recent_days: GetRecentDaysProfit(transactions, 7)
  };
}
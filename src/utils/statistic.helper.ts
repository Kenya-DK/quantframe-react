import dayjs from "dayjs";
import isBetween from 'dayjs/plugin/isBetween';
dayjs.extend(isBetween);
import i18next from "i18next";
import { groupBy, getGroupByDate } from ".";
import { StatisticDto, TransactionEntryDto, StatisticTotalTransaction, StatisticTransactionRevenueWithChart, StatisticTodayTransaction, StatisticTransactionRevenue, StatisticRecentDaysTransaction, StatisticTransactionItemRevenue, StatisticTransactionPopularItems } from "../types";


type GroupByItem = { wfm_id: string; url: string; item_type: string; name: string; tags: string[]; quantity: number; price: number; total: number; }
const GetGroupByItem = (transactions: TransactionEntryDto[]): GroupByItem[] => {
  // Initialize an empty array to hold the grouped products
  let items: Array<GroupByItem> = [];
  // Clone the orders array to avoid modifying the original
  let transactionsgroupBy = groupBy("url", transactions);
  // Loop through the orders
  for (let item in transactionsgroupBy) {
    // Get the order
    let transactionList = transactionsgroupBy[item];

    let firstTransaction = transactionList[0];
    let trans = {
      wfm_id: firstTransaction.wfm_id,
      url: firstTransaction.url,
      item_type: firstTransaction.item_type,
      name: firstTransaction.name,
      tags: firstTransaction.tags.split(","),
      quantity: transactionList.reduce((acc, cur) => acc + cur.quantity, 0),
      price: transactionList.reduce((acc, cur) => acc + cur.price, 0),
      total: transactionList.length,
    };
    items.push(trans);
  }
  if (items.length == 0) return [
    {
      wfm_id: "",
      url: "",
      item_type: "",
      name: "",
      tags: [],
      quantity: 0,
      price: 0,
      total: 0,
    }
  ];
  return items.sort((a, b) => b.quantity - a.quantity);
};

const GetRevenueForItems = (bought: GroupByItem[], sold: GroupByItem[]): StatisticTransactionItemRevenue[] => {
  return bought.map((item) => {
    const sell = sold.find((sell_item) => sell_item.url == item.url);
    let turnover = 0;
    let price = 0;
    if (sell) {
      turnover = item.price - sell.price;
      price = sell.price;
    }
    return {
      wfm_id: item.wfm_id,
      url: item.url,
      item_type: item.item_type,
      name: item.name,
      tags: item.tags,
      quantity: item.quantity,
      total_bought: sell ? sell.total : 0,
      total_sold: item.total,
      price: price,
      turnover: turnover,
    };
  })
};

const getRevenue = (transactions: TransactionEntryDto[]): StatisticTransactionRevenue => {
  const revenue = transactions.reduce((acc, cur) => acc + cur.price, 0);
  return {
    average: revenue == 0 ? 0 : revenue / transactions.length,
    quantity: transactions.length,
    revenue: revenue,
  };

};

const getRevenueWithChart = (labels: string[], transactions: TransactionEntryDto[], settings: { year?: boolean, month?: boolean, day?: boolean, hours?: boolean }): StatisticTransactionRevenueWithChart => {
  const groups = getGroupByDate<TransactionEntryDto>("datetime", transactions, settings);
  const quantity_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].length : 0);
  const revenue_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].reduce((previousValue, currentValue) => previousValue + currentValue.price, 0) : 0);


  return {
    ...getRevenue(transactions),
    quantity_chart: quantity_chart,
    revenue_chart: revenue_chart,
  };

};

const getTotalRevenue = (transactions: TransactionEntryDto[]): StatisticTotalTransaction => {

  // Initialize an empty array to hold the grouped products
  const sell_transactions = transactions.filter(t => t.transaction_type == "sell");
  const buy_transactions = transactions.filter(t => t.transaction_type == "buy");

  // Create Chart Data
  const thisYearLabels = [];
  const lastYearLabels = [];
  const year = new Date().getFullYear();
  for (let i = 0; i < 12; i++) {
    thisYearLabels.push(`${i} ${year}`);
    lastYearLabels.push(`${i} ${year - 1}`);
  }


  const thisYearTransactions = transactions.filter(t => dayjs(t.created).isSame(new Date(), "year"));
  const lastYearTransactions = transactions.filter(t => dayjs(t.created).isSame(new Date().getFullYear() - 1, "year"));

  return {
    labels: i18next.t("general.months", { returnObjects: true }) as string[],
    buy: getRevenue(buy_transactions),
    sales: getRevenue(sell_transactions),
    previous: {
      labels: lastYearLabels,
      sales: getRevenueWithChart(lastYearLabels, sell_transactions, { year: true, month: true, day: false, hours: false }),
      buy: getRevenueWithChart(lastYearLabels, buy_transactions, { year: true, month: true, day: false, hours: false }),
      popular_items: getBestItem(lastYearTransactions),
    },
    present: {
      labels: thisYearLabels,
      sales: getRevenueWithChart(thisYearLabels, sell_transactions, { year: true, month: true, day: false, hours: false }),
      buy: getRevenueWithChart(thisYearLabels, buy_transactions, { year: true, month: true, day: false, hours: false }),
      popular_items: getBestItem(thisYearTransactions),
    },
  };

};

const getBestItem = (transactions: TransactionEntryDto[]): StatisticTransactionPopularItems => {
  const groped_sell = GetGroupByItem(transactions.filter(t => t.transaction_type == "buy"));
  const groped_buy = GetGroupByItem(transactions.filter(t => t.transaction_type == "sell"));

  return {
    buy: GetRevenueForItems(groped_sell, groped_buy),
    sell: GetRevenueForItems(groped_buy, groped_sell),
  }

};


const getTodayRevenue = (transactions: TransactionEntryDto[]): StatisticTodayTransaction => {
  let today = dayjs().startOf("day").toDate();
  let endToday = dayjs().endOf('day').toDate();
  transactions = transactions.filter(t => dayjs(t.created).isBetween(today, endToday));
  const sell_transactions = transactions.filter(t => t.transaction_type == "sell");
  const buy_transactions = transactions.filter(t => t.transaction_type == "buy");

  const labels = [];
  for (let i = 0; i < 24; i++) labels.push(`${i}:00`);

  return {
    labels: labels,
    sales: getRevenueWithChart(labels, sell_transactions, { hours: true }),
    buy: getRevenueWithChart(labels, buy_transactions, { hours: true }),
    popular_items: getBestItem(transactions),
  };

};

const getRecentDays = (transactions: TransactionEntryDto[], days: number): StatisticRecentDaysTransaction => {
  let today = dayjs().subtract(days, "day").endOf('day').toDate();
  let endToday = dayjs().endOf('day').toDate();
  transactions = transactions.filter(t => dayjs(t.created).isBetween(today, endToday));
  const sell_transactions = transactions.filter(t => t.transaction_type == "sell");
  const buy_transactions = transactions.filter(t => t.transaction_type == "buy");

  const labels = [];
  const date = new Date();
  date.setDate(date.getDate() - (days - 1));
  for (let i = 0; i < days; i++) {
    labels.push(`${date.getDate()} ${date.getMonth()} ${date.getFullYear()}`);
    date.setDate(date.getDate() + 1);
  }

  return {
    days: days,
    labels: labels,
    sales: getRevenueWithChart(labels, sell_transactions, { month: true, day: true, year: true }),
    buy: getRevenueWithChart(labels, buy_transactions, { month: true, day: true, year: true }),
    popular_items: getBestItem(transactions),
  };

};

export const getStatistic = (transactions: TransactionEntryDto[]): StatisticDto => {

  const sell_transactions = transactions.filter(t => t.transaction_type == "sell");
  const buy_transactions = transactions.filter(t => t.transaction_type == "buy");

  const spend_plat = buy_transactions.reduce((acc, cur) => acc + cur.price, 0);

  const earned_plat = sell_transactions.reduce((acc, cur) => acc + cur.price, 0);

  return {
    total: getTotalRevenue(transactions),
    today: getTodayRevenue(transactions),
    recent_days: getRecentDays(transactions, 7),
    turnover: earned_plat - spend_plat,
  };
}





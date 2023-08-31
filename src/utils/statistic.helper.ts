import dayjs from "dayjs";
import isBetween from 'dayjs/plugin/isBetween';
dayjs.extend(isBetween);
import i18next from "i18next";
import { groupBy, getGroupByDate } from ".";
import { StatisticDto, TransactionEntryDto, StatisticTotalTransaction, StatisticTransactionRevenueWithChart, StatisticTransactionBestSeller, StatisticTodayTransaction, StatisticTransactionRevenue, StatisticRecentDaysTransaction } from "../types";



const getBestSellerItem = (transactions: TransactionEntryDto[]): StatisticTransactionBestSeller[] => {
  // Initialize an empty array to hold the grouped products
  let items: Array<StatisticTransactionBestSeller> = [];
  // Clone the orders array to avoid modifying the original
  let transactionsgroupBy = groupBy("item_url", transactions);
  // Loop through the orders
  for (let item in transactionsgroupBy) {
    // Get the order
    let transaction = transactionsgroupBy[item];

    let trans: StatisticTransactionBestSeller = {
      item_id: transaction[0].item_id,
      item_url: transaction[0].item_url,
      item_type: transaction[0].item_type,
      item_name: transaction[0].item_name,
      quantity: transaction.reduce((acc, cur) => acc + cur.quantity, 0),
      revenue: transaction.reduce((acc, cur) => acc + cur.price, 0),
    };
    items.push(trans);
  }
  return items;
};


const getRevenue = (transactions: TransactionEntryDto[]): StatisticTransactionRevenue => {
  const revenue = transactions.reduce((acc, cur) => acc + cur.price, 0);
  return {
    average: revenue == 0 ? 0 : revenue / transactions.length,
    quantity: transactions.length,
    revenue: revenue,
    best_sellers: getBestSellerItem(transactions).sort((a, b) => b.quantity - a.quantity).slice(0, 5),
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
  const currentMonth = new Date().getMonth() + 1;
  console.log(currentMonth, thisYearLabels, lastYearLabels);


  return {
    labels: i18next.t("general.months", { returnObjects: true }) as string[],
    buy: getRevenue(buy_transactions),
    sales: getRevenue(sell_transactions),
    previous: {
      labels: lastYearLabels,
      sales: getRevenueWithChart(lastYearLabels, sell_transactions, { year: true, month: true, day: false, hours: false }),
      buy: getRevenueWithChart(lastYearLabels, buy_transactions, { year: true, month: true, day: false, hours: false }),
    },
    present: {
      labels: thisYearLabels,
      sales: getRevenueWithChart(thisYearLabels, sell_transactions, { year: true, month: true, day: false, hours: false }),
      buy: getRevenueWithChart(thisYearLabels, buy_transactions, { year: true, month: true, day: false, hours: false }),
    },
  };

};

const getTodayRevenue = (transactions: TransactionEntryDto[]): StatisticTodayTransaction => {
  let today = dayjs().startOf("day").toDate();
  let endToday = dayjs().endOf('day').toDate();
  transactions = transactions.filter(t => dayjs(t.datetime).isBetween(today, endToday));
  const sell_transactions = transactions.filter(t => t.transaction_type == "sell");
  const buy_transactions = transactions.filter(t => t.transaction_type == "buy");

  const labels = [];
  for (let i = 0; i < 24; i++) labels.push(`${i}:00`);

  return {
    labels: labels,
    sales: getRevenueWithChart(labels, sell_transactions, { hours: true }),
    buy: getRevenueWithChart(labels, buy_transactions, { hours: true }),
  };

};

const getRecentDays = (transactions: TransactionEntryDto[], days: number): StatisticRecentDaysTransaction => {
  let today = dayjs().subtract(days, "day").endOf('day').toDate();
  let endToday = dayjs().endOf('day').toDate();
  transactions = transactions.filter(t => dayjs(t.datetime).isBetween(today, endToday));
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





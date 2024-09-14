import dayjs from "dayjs";
import isBetween from 'dayjs/plugin/isBetween';
dayjs.extend(isBetween);
import { TauriClient } from "..";
import { TransactionDto, TransactionType, ChartMultipleDto, StatisticProfitTransaction, StatisticProfitItem, CategoryItemProfitLink, StatisticItemCategoryProfit, StatisticProfitTransactionTotal, StatisticProfitTransactionToday, StatisticProfitTransactionRecentDays, StatisticRecentTransactions, StatisticItemBestSeller, StatisticDto } from "../types";
import { GroupByDateSettings, getGroupByDate, groupBy } from "@utils/helper";
import i18next from "i18next";

export class StatisticModule {
  private _statistic: StatisticDto | null = null;
  constructor(private readonly client: TauriClient) { }


  private SplitTransactionType(transactions: TransactionDto[]): [TransactionDto[], TransactionDto[]] {
    return [transactions.filter(t => t.transaction_type == TransactionType.Purchase), transactions.filter(t => t.transaction_type == TransactionType.Sale)];
  }

  private GetItemChartProfit(transactions: TransactionDto[]): ChartMultipleDto {
    const items = this.GetItemsProfit(transactions);
    const labels = items.map((item: StatisticProfitItem) => item.name);
    const groups = groupBy("name", items);
    const profit_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticProfitItem) => item.profit).reduce((acc, cur) => acc + cur, 0) : 0);
    return {
      labels: labels,
      profit_values: profit_chart
    };
  }

  private GetTransactionChartProfit(transactions: TransactionDto[], settings: GroupByDateSettings): ChartMultipleDto {
    const [groups, labels] = getGroupByDate<TransactionDto>("created_at", transactions, settings);
    const profit_chart = labels.map((label: string) => groups[`${label}`] ? this.GetTransactionProfit(groups[`${label}`]).profit : 0);
    return {
      labels: labels,
      profit_values: profit_chart
    };
  }

  private GetTransactionProfit(transactions: TransactionDto[]): StatisticProfitTransaction {
    const [buy_transactions, sell_transactions] = this.SplitTransactionType(transactions);
    const expense = buy_transactions.reduce((acc, cur) => acc + cur.price, 0);
    const revenue = sell_transactions.reduce((acc, cur) => acc + cur.price, 0);
    const profit = revenue - expense;
    return {
      profit: profit,
      profit_margin: (revenue == 0 ? 0 : profit / revenue),
      average_revenue: revenue / transactions.length,
      number_of_trades: transactions.length,
      expense: expense,
      revenue: revenue,
      purchases: buy_transactions.length,
      sales: sell_transactions.length,
      popular_items: this.GetItemsProfit(transactions)
    };
  }

  private GetItemsProfit(transactions: TransactionDto[]): StatisticProfitItem[] {
    let items: StatisticProfitItem[] = [];
    let transactionsGroupBy = groupBy("wfm_url", transactions);
    for (let item in transactionsGroupBy) {
      // Get the order
      let transactionList = transactionsGroupBy[item];

      let firstTransaction = transactionList[0];
      const [buy_transactions, sell_transactions] = this.SplitTransactionType(transactionList);
      const expense = buy_transactions.reduce((acc, cur) => acc + cur.price, 0);
      const revenue = sell_transactions.reduce((acc, cur) => acc + cur.price, 0);
      const profit = revenue - expense;

      let trans = {
        profit: profit,
        profit_margin: (revenue == 0 ? 0 : profit / revenue),
        average_revenue: revenue / transactions.length,
        number_of_trades: transactions.length,
        expense: expense,
        revenue: revenue,
        purchases: buy_transactions.length,
        sales: sell_transactions.length,
        wfm_id: firstTransaction.wfm_id,
        url: firstTransaction.wfm_url,
        item_type: firstTransaction.item_type,
        name: firstTransaction.item_name,
        tags: firstTransaction.tags.split(","),
        quantity: transactionList.reduce((acc, cur) => acc + cur.quantity, 0),
      };
      items.push(trans);
    }
    return items.sort((a, b) => b.profit - a.profit);
  }

  private GetItemCategoryProfit(transactionsIn: TransactionDto[], categoryIn: CategoryItemProfitLink[]): StatisticItemCategoryProfit[] {
    let transactions = [...transactionsIn];
    let items: StatisticItemCategoryProfit[] = [];
    for (let index = 0; index < categoryIn.length; index++) {
      const category = categoryIn[index];
      // Get the order
      let transactionList = transactions.filter(t => t.tags.split(",").some(r => category.tags.includes(r)) || category.types.includes(t.item_type));

      let trans = {
        ...this.GetTransactionProfit(transactionList),
        ...category,
        quantity: transactionList.reduce((acc, cur) => acc + cur.quantity, 0),
      };
      items.push(trans);
      // Remove the transactions from the list
      transactions = transactions.filter(t => !transactionList.includes(t));
    }
    let trans = {
      ...this.GetTransactionProfit(transactions),
      icon: "other.png",
      name: "Other",
      quantity: transactions.reduce((acc, cur) => acc + cur.quantity, 0),
    };
    items.push(trans);
    return items.sort((a, b) => b.profit - a.profit);
  }

  private GetItemCategoryChartProfit(transactions: TransactionDto[], category: CategoryItemProfitLink[]): ChartMultipleDto {
    const items = this.GetItemCategoryProfit(transactions, category);
    const labels = items.map((item: StatisticItemCategoryProfit) => item.name);
    const groups = groupBy("name", items);
    const profit_chart = labels.map((label: string) => groups[`${label}`] ? groups[`${label}`].map((item: StatisticItemCategoryProfit) => item.profit).reduce((acc, cur) => acc + cur, 0) : 0);
    return {
      labels: labels,
      profit_values: profit_chart,
    };
  }

  private GetTotalProfit(transactions: TransactionDto[]): StatisticProfitTransactionTotal {
    // Create Chart Data
    const thisYearLabels = [];
    const lastYearLabels = [];
    const year = new Date().getFullYear();
    for (let i = 0; i < 12; i++) {
      thisYearLabels.push(`${i} ${year}`);
      lastYearLabels.push(`${i} ${year - 1}`);
    }

    const thisYearTransactions = transactions.filter(t => dayjs(t.created_at).isSame(new Date(), "year"));
    const lastYearTransactions = transactions.filter(t => dayjs(t.created_at).isSame(dayjs().subtract(1, "year"), "year"));

    const byDateSettings: GroupByDateSettings = { year: true, month: true, day: false, hours: false }
    return {
      ...this.GetTransactionProfit(transactions),
      labels: i18next.t("months", { returnObjects: true }) as string[],
      present: {
        ...this.GetTransactionProfit(thisYearTransactions),
        ...this.GetTransactionChartProfit(thisYearTransactions, { ...byDateSettings, labels: thisYearLabels }),
      },
      previous: {
        ...this.GetTransactionProfit(lastYearTransactions),
        ...this.GetTransactionChartProfit(lastYearTransactions, { ...byDateSettings, labels: lastYearLabels }),
      },
    };
  }

  private GetToDayProfit(transactions: TransactionDto[]): StatisticProfitTransactionToday {
    let today = dayjs().startOf("day").toDate();
    let endToday = dayjs().endOf('day').toDate();
    const labels = [];
    for (let i = 0; i < 24; i++) labels.push(`${i}:00`);

    transactions = transactions.filter(t => dayjs(t.created_at).isBetween(today, endToday));
    return {
      ...this.GetTransactionProfit(transactions),
      chart_profit: { ...this.GetTransactionChartProfit(transactions, { year: false, month: false, day: false, hours: true, labels: labels }) },
      chart_items: this.GetItemChartProfit(transactions)
    };
  }

  private GetRecentDaysProfit(transactions: TransactionDto[], days: number): StatisticProfitTransactionRecentDays {
    let today = dayjs().subtract(days, "day").endOf('day').toDate();
    let endToday = dayjs().endOf('day').toDate();
    transactions = transactions.filter(t => dayjs(t.created_at).isBetween(today, endToday));
    const labels = [];
    const date = new Date();
    date.setDate(date.getDate() - (days - 1));
    for (let i = 0; i < days; i++) {
      labels.push(`${date.getDate()} ${date.getMonth()} ${date.getFullYear()}`);
      date.setDate(date.getDate() + 1);
    }
    return {
      ...this.GetTransactionProfit(transactions),
      chart_profit: { ...this.GetTransactionChartProfit(transactions, { month: true, day: true, year: true, labels: labels }) },
      chart_items: this.GetItemChartProfit(transactions),
      days
    };
  }

  private GetRecentTransactions(transactions: TransactionDto[], count: number): StatisticRecentTransactions {
    transactions = transactions.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()).slice(0, count);
    return {
      ...this.GetTransactionProfit(transactions),
      transactions: transactions
    };
  }

  private GetBestSeller(transactions: TransactionDto[]): StatisticItemBestSeller {
    const category = [
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
      items: this.GetItemsProfit(transactions),
      items_chart: { ...this.GetItemChartProfit(transactions) },
      category: this.GetItemCategoryProfit(transactions, category),
      category_chart: this.GetItemCategoryChartProfit(transactions, category),
    };
  }

  convertFromTransaction(transactions: TransactionDto[]): StatisticDto {
    return {
      best_seller: this.GetBestSeller(transactions),
      total: this.GetTotalProfit(transactions),
      today: this.GetToDayProfit(transactions),
      recent_days: this.GetRecentDaysProfit(transactions, 10),
      recent_transactions: this.GetRecentTransactions(transactions, 10)
    };
  }

  async getStatistic(): Promise<StatisticDto> {
    if (this._statistic)
      return this._statistic;
    const transactions = await this.client.transaction.getAll();
    this._statistic = this.convertFromTransaction(transactions);
    return this._statistic;
  }
}

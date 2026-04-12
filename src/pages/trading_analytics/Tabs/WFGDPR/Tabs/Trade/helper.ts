import { TauriTypes } from "$types";
import { GroupByKey } from "@utils/helper";

export const GenerateCategoryReport = (
  trades: TauriTypes.PlayerTrade[],
  settings: TauriTypes.SettingsSummary | undefined,
): TauriTypes.FinancialReport[] => {
  if (!settings) return [GenerateFinancialReport(trades)];

  const reports: TauriTypes.FinancialReport[] = [];

  for (const category of settings.categories) {
    const categoryTrades = trades.filter((trade) => {
      const items = [...(trade.offeredItems || []), ...(trade.receivedItems || [])];
      const hasMatchingTag = items.some((item) => item.properties?.tags?.some((tag: string) => category.tags.includes(tag)));
      const hasMatchingType = items.some((item) => item.item_type && category.types.includes(item.item_type));

      return hasMatchingTag || hasMatchingType;
    });

    const categoryReport = GenerateFinancialReport(categoryTrades);
    categoryReport.properties = {
      ...categoryReport.properties,
      name: category.name,
      icon: category.icon,
    };
    reports.push(categoryReport);
  }

  return reports;
};
export const GenerateFinancialReport = (items: TauriTypes.PlayerTrade[]): TauriTypes.FinancialReport => {
  let purchases = items.filter((i) => i.type === "purchase");
  let purchases_items = purchases.flatMap((p) => p.receivedItems || []).filter((i) => i.item_type !== "Credits" && i.item_type !== "Platinum");
  let expenses = purchases.reduce((acc, p) => acc + (p.platinum || 0), 0);
  let highest_expense = Math.max(...purchases.map((t) => t.platinum || 0));
  let lowest_expense = Math.min(...purchases.map((t) => t.platinum || 0));

  let purchase_quantities_by_item = Object.entries(GroupByKey("properties.item_name", purchases_items)).map(([name, items]) => ({
    name,
    quantity: (items as any[]).reduce((acc, i) => acc + (i.quantity || 0), 0),
  }));

  let sales = items.filter((i) => i.type === "sale");
  let sales_items = sales.flatMap((s) => s.offeredItems || []).filter((i) => i.item_type !== "Credits" && i.item_type !== "Platinum");
  let revenue = sales.reduce((acc, s) => acc + (s.platinum || 0), 0);
  let highest_revenue = Math.max(...sales.map((t) => t.platinum || 0));
  let lowest_revenue = Math.min(...sales.map((t) => t.platinum || 0));

  let sale_quantities_by_item = Object.entries(GroupByKey("properties.item_name", sales_items)).map(([name, items]) => ({
    name,
    quantity: (items as any[]).reduce((acc, i) => acc + (i.quantity || 0), 0),
  }));

  return {
    // General transaction metrics
    total_transactions: items.length,
    average_transaction: (revenue - expenses) / (items.length || 1),

    // Profit metrics
    total_profit: revenue - expenses,
    average_profit: (revenue - expenses) / (items.length || 1),
    profit_margin: revenue / (expenses || 1),
    roi: ((revenue - expenses) / (expenses || 1)) * 100, // Return on Investment percentage

    // Revenue metrics
    sale_count: sales.length,
    highest_revenue: highest_revenue,
    lowest_revenue: lowest_revenue,
    average_revenue: revenue / (sales.length || 1),
    revenue: revenue,

    // Expense metrics
    purchases_count: purchases.length,
    highest_expense: highest_expense,
    lowest_expense: lowest_expense,
    average_expense: expenses / (purchases.length || 1),
    expenses: expenses,
    properties: {
      most_purchased_items: purchase_quantities_by_item.sort((a, b) => b.quantity - a.quantity).slice(0, 5),
      most_sold_items: sale_quantities_by_item.sort((a, b) => b.quantity - a.quantity).slice(0, 5),
      total_trades: items.filter((i) => i.type === "trade").length,
      total_credits: items.reduce((acc, i) => acc + (i.credits || 0), 0),
    },
  };
};

export const GenerateReport = (items: TauriTypes.PlayerTrade[], settings: TauriTypes.SettingsSummary | undefined): TauriTypes.FinancialReport => {
  let report = GenerateFinancialReport(items);
  let category_report = GenerateCategoryReport(items, settings);
  if (report.properties) report.properties["categories"] = category_report;
  // Here you can modify the report based on settings if needed
  return report;
};

export const GenerateYearlyReport = (
  items: TauriTypes.PlayerTrade[],
): Record<
  string,
  {
    total_purchases: number[];
    total_sales: number[];
    total_trades: number[];
    report: TauriTypes.FinancialReport;
  }
> => {
  const result: Record<
    string,
    {
      total_purchases: number[];
      total_sales: number[];
      total_trades: number[];
      report: TauriTypes.FinancialReport;
    }
  > = {};

  for (const trade of items) {
    const tradeDate = new Date(trade.tradeTime);
    if (isNaN(tradeDate.getTime())) continue;

    const year = tradeDate.getFullYear().toString();
    const month = tradeDate.getMonth(); // 0-11

    if (!result[year]) {
      result[year] = {
        total_purchases: Array(12).fill(0),
        total_sales: Array(12).fill(0),
        total_trades: Array(12).fill(0),
        report: GenerateFinancialReport([]),
      };
    }

    if (trade.type === TauriTypes.TransactionType.Purchase) result[year].total_purchases[month] += trade.platinum || 0;
    else if (trade.type === TauriTypes.TransactionType.Sale) result[year].total_sales[month] += trade.platinum || 0;
    else if (trade.type === TauriTypes.TransactionType.Trade) result[year].total_trades[month] += 1;
  }

  // Generate yearly financial reports
  for (const [year] of Object.entries(result)) {
    const yearTrades = items.filter((trade) => {
      const tradeDate = new Date(trade.tradeTime);
      return !isNaN(tradeDate.getTime()) && tradeDate.getFullYear().toString() === year;
    });
    result[year].report = GenerateFinancialReport(yearTrades);
  }

  return result;
};

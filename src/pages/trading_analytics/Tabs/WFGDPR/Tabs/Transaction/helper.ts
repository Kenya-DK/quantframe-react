import { TauriTypes } from "$types";
import { GroupByKey } from "@utils/helper";

export const GenerateFinancialReport = (transactions: TauriTypes.WFGDPRTransaction[]): TauriTypes.FinancialReport => {
  // Get the prices of all items received in purchases
  let expenses = transactions.reduce((acc, p) => acc + (p.price || 0), 0);
  // Can y Get the item with the highest price and lowest price
  let highest_expense = Math.max(...transactions.map((t) => t.price || 0));
  let lowest_expense = Math.min(...transactions.map((t) => t.price || 0));
  let purchase_quantities_by_item = Object.entries(GroupByKey("sku", transactions)).map(([name, items]) => {
    let quantity = items.length; // Total quantity purchased for this SKU
    let price = (items as any[]).reduce((acc, i) => acc + (i.price || 0), 0); // Total price for this SKU
    return { name, quantity, price };
  });

  return {
    // General transaction metrics
    total_transactions: transactions.length,
    average_transaction: 0,

    // Profit metrics
    total_profit: 0,
    average_profit: 0,
    profit_margin: 0,
    roi: 0, // Return on Investment percentage

    // Revenue metrics
    sale_count: 0,
    highest_revenue: 0,
    lowest_revenue: 0,
    average_revenue: 0,
    revenue: 0,

    // Expense metrics
    purchases_count: transactions.length,
    highest_expense: highest_expense,
    lowest_expense: lowest_expense,
    average_expense: expenses / (transactions.length || 1),
    expenses: expenses,
    properties: {
      most_purchased_items: purchase_quantities_by_item.sort((a, b) => b.quantity - a.quantity).slice(0, 50),
    },
  };
};

export const GenerateReport = (items: TauriTypes.WFGDPRTransaction[]): TauriTypes.FinancialReport => {
  let report = GenerateFinancialReport(items);
  // Here you can modify the report based on settings if needed
  return report;
};

export const GenerateYearlyReport = (
  items: TauriTypes.WFGDPRTransaction[],
): Record<
  string,
  {
    total_purchases: number[];
    report: TauriTypes.FinancialReport;
  }
> => {
  const result: Record<
    string,
    {
      total_purchases: number[];
      report: TauriTypes.FinancialReport;
    }
  > = {};

  for (const trade of items) {
    const tradeDate = new Date(trade.date);
    if (isNaN(tradeDate.getTime())) continue;

    const year = tradeDate.getFullYear().toString();
    const month = tradeDate.getMonth(); // 0-11

    if (!result[year]) {
      result[year] = {
        total_purchases: Array(12).fill(0),
        report: GenerateFinancialReport([]),
      };
    }

    result[year].total_purchases[month] += trade.price || 0;
  }

  // Generate yearly financial reports
  for (const [year] of Object.entries(result)) {
    const yearTrades = items.filter((trade) => {
      const tradeDate = new Date(trade.date);
      return !isNaN(tradeDate.getTime()) && tradeDate.getFullYear().toString() === year;
    });
    result[year].report = GenerateFinancialReport(yearTrades);
  }

  return result;
};

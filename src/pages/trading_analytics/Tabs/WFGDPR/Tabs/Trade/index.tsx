import { Box, Grid, Group, Select, Table, Title, useMantineTheme } from "@mantine/core";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useQueries } from "./queries";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";
import { BarCardChart } from "@components/Shared/BarCardChart";
import i18next from "i18next";
import { BestByCategoryTable } from "@components/DataDisplay/BestByCategoryTable";
import { BarChartFinancialSummary } from "@components/DataDisplay/BarChartFinancialSummary";

interface TradePanelProps {
  isActive?: boolean;
  year_list?: string[];
}

export const TradePanel = ({ isActive, year_list }: TradePanelProps) => {
  console.log("TradePanel isActive:", isActive);
  const theme = useMantineTheme();
  // States For DataGrid
  const queryData = useForm({
    initialValues: { page: 1, limit: 10, query: "", year: new Date().getFullYear() } as TauriTypes.WFGDPRTradeControllerGetListParams,
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.tabs.wfgdpr.trade.${key}`, { ...context }, i18Key);
  const useTranslateCards = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`cards.${key}`, { ...context }, i18Key);

  // Queries
  const { financialReportQuery } = useQueries({ queryData: queryData.values, isActive });
  return (
    <Box p={"md"} h={"85vh"}>
      <Grid>
        <Grid.Col span={6}>
          <StatsWithSegments
            p={0}
            orientation="vertical"
            hidePercentBar
            segments={[
              {
                label: useTranslate("labels.total_transactions"),
                count: financialReportQuery.data?.total_transactions || 0,
                color: "orange",
                tooltip: useTranslate("tooltips.total_credits"),
                part: financialReportQuery.data?.properties.total_credits || 0,
                suffix: " C",
                decimalScale: 2,
              },
              {
                label: useTranslate("labels.trade_count"),
                count: financialReportQuery.data?.properties.total_trades || 0,
                color: "var(--qf-transaction-type-trade)",
                part: null,
              },
              {
                label: useTranslate("labels.purchases_count"),
                count: financialReportQuery.data?.purchases_count || 0,
                color: "var(--qf-transaction-type-purchase)",
                part: financialReportQuery.data?.expenses || 0,
                suffix: " P",
              },
              {
                label: useTranslate("labels.sales_count"),
                count: financialReportQuery.data?.sale_count || 0,
                color: "var(--qf-transaction-type-sale)",
                part: financialReportQuery.data?.revenue || 0,
                suffix: " P",
              },
              {
                label: useTranslate("labels.total_profit"),
                count: financialReportQuery.data?.total_profit || 0,
                color: "teal",
                tooltip: useTranslate("tooltips.profit_margin"),
                part: financialReportQuery.data?.profit_margin || 0,
                decimalScale: 2,
              },
            ]}
            showPercent
            percentSymbol="%"
          />
        </Grid.Col>
        <Grid.Col span={3}>
          <Title order={4} mb={"sm"}>
            {useTranslate("titles.most_purchased_items")}
          </Title>
          <Table>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>{useTranslate("table_headers.item_name")}</Table.Th>
                <Table.Th>{useTranslate("table_headers.quantity")}</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {financialReportQuery.data?.properties.most_purchased_items.map((item) => (
                <Table.Tr key={item[0]}>
                  <Table.Td>{item[0]}</Table.Td>
                  <Table.Td>{item[1]}</Table.Td>
                </Table.Tr>
              )) || null}
            </Table.Tbody>
          </Table>
        </Grid.Col>
        <Grid.Col span={3}>
          <Title order={4} mb={"sm"}>
            {useTranslate("titles.most_sold_items")}
          </Title>
          <Table>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>{useTranslate("table_headers.item_name")}</Table.Th>
                <Table.Th>{useTranslate("table_headers.quantity")}</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {financialReportQuery.data?.properties.most_sold_items.map((item) => (
                <Table.Tr key={item[0]}>
                  <Table.Td>{item[0]}</Table.Td>
                  <Table.Td>{item[1]}</Table.Td>
                </Table.Tr>
              )) || null}
            </Table.Tbody>
          </Table>
        </Grid.Col>
      </Grid>
      <Grid>
        <Grid.Col span={6}>
          <BarCardChart
            title={useTranslateCards("yearly_trade_overview.title", { year: queryData.values.year })}
            boxHeight={400}
            showDatasetLabels
            labels={i18next.t("months", { returnObjects: true }) as string[]}
            chartStyle={{ background: theme.colors.dark[7], height: "200px" }}
            datasets={[
              {
                label: useTranslateCards("yearly_trade_overview.total_trades"),
                data: financialReportQuery.data?.properties.graph.values.total_trades || [],
                backgroundColor: theme.other.transactionType.trade,
              },
              {
                label: useTranslateCards("yearly_trade_overview.total_purchase"),
                data: financialReportQuery.data?.properties.graph.values.total_purchase || [],
                backgroundColor: theme.other.transactionType.purchase,
              },
              {
                label: useTranslateCards("yearly_trade_overview.total_sales"),
                data: financialReportQuery.data?.properties.graph.values.total_sales || [],
                backgroundColor: theme.other.transactionType.sale,
              },
            ]}
            context={
              <Group>
                <Group flex={1}>
                  <BarChartFinancialSummary statistics={financialReportQuery.data?.properties.year} />
                </Group>
                <Group flex={"1 auto"} justify={"flex-end"}>
                  <Select
                    w={100}
                    data={[
                      ...(year_list || []).map((year) => ({ value: year, label: year })),
                      { value: new Date().getFullYear().toString(), label: new Date().getFullYear().toString() },
                    ]}
                    value={queryData.values.year?.toString()}
                    onChange={(value) => {
                      if (!value) return;
                      queryData.setFieldValue("year", Number(value));
                    }}
                  />
                </Group>
              </Group>
            }
          />
        </Grid.Col>
        <Grid.Col span={6}>
          <BestByCategoryTable records={financialReportQuery.data?.properties.categories || []} />
        </Grid.Col>
      </Grid>
    </Box>
  );
};

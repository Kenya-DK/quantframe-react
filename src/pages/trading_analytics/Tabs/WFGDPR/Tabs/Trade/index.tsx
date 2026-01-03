import { Box, Grid, Table, Title } from "@mantine/core";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { SearchField } from "@components/Forms/SearchField";
import { useQueries } from "./queries";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";

interface TradePanelProps {}

export const TradePanel = ({}: TradePanelProps = {}) => {
  // States For DataGrid
  const queryData = useForm({
    initialValues: { page: 1, limit: 10, query: "" } as TauriTypes.WFGDPRTradeControllerGetListParams,
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.tabs.wfgdpr.trade.${key}`, { ...context }, i18Key);

  // Queries
  const { paginationQuery, financialReportQuery } = useQueries({ queryData: queryData.values });
  return (
    <Box p={"md"}>
      <SearchField
        value={queryData.values.query || ""}
        onSearch={() => {
          queryData.validate();
        }}
        searchDisabled={paginationQuery.isLoading}
        onChange={(text) => queryData.setFieldValue("query", text)}
      />
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
    </Box>
  );
};

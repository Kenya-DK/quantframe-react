import { Box, Grid, Group, NumberFormatter, Paper, Select, Table, Text, Title, useMantineTheme } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { useEffect, useState } from "react";
import { ApplyFilter, ComplexFilter, Operator, OperatorType } from "@utils/filter.helper";
import { faCoins } from "@fortawesome/free-solid-svg-icons";
import { ColorInfo } from "@components/Shared/ColorInfo";
import dayjs from "dayjs";
import { DataTableSearch } from "@components/Shared/DataTableSearch";
import { SelectItemTags } from "@components/Forms/SelectItemTags";
import { FinancialReportCard } from "@components/Shared/FinancialReportCard";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { GenerateReport, GenerateYearlyReport } from "./helper";
import classes from "../../WFGDPR.module.css";
import { BestByCategoryTable } from "@components/DataDisplay/BestByCategoryTable";
import { useAppContext } from "@contexts/app.context";
import { BarCardChart } from "@components/Shared/BarCardChart";
import i18next from "i18next";
import { BarChartFinancialSummary } from "../../../../../../components/DataDisplay/BarChartFinancialSummary";
interface TradePanelProps {
  value: TauriTypes.WFGDPRAccount | null;
}
interface QueryData {
  query?: string;
  page: number;
  limit: number;
  sort_by: string;
  sort_direction: "asc" | "desc";
  type?: string;
  tags?: string[];
  from_date?: string;
  to_date?: string;
}

export const TradePanel = ({ value }: TradePanelProps) => {
  const { settings } = useAppContext();
  const theme = useMantineTheme();
  const [queryData, setQueryData] = useState<QueryData>({
    page: 1,
    limit: 50,
    sort_by: "created_at",
    sort_direction: "desc",
    type: undefined,
  });
  const [showReport, setShowReport] = useState<boolean>(false);
  const [financialReport, setFinancialReport] = useState<TauriTypes.FinancialReport | undefined>(undefined);
  const [financialReportYears, setFinancialReportYears] = useState<
    Record<string, { total_purchases: number[]; total_sales: number[]; total_trades: number[]; report: TauriTypes.FinancialReport }>
  >({});
  const [reportYear, setReportYear] = useState<string>(
    financialReportYears ? Object.keys(financialReportYears)[0] : new Date().getFullYear().toString(),
  );

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.tabs.wfgdpr.trade.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`columns.${key}`, { ...context }, i18Key);
  const useTranslateTransactionType = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`transaction_type.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`buttons.${key}`, { ...context }, i18Key);

  const GenerateFilter = (): ComplexFilter => {
    let filter: ComplexFilter = { AND: [], OR: [] };
    if (queryData.type) filter.AND?.push({ type: { [Operator.EQUALS]: queryData.type } });
    if (queryData.query) filter.OR?.push({ "properties.names": { isCaseSensitive: false, [Operator.MATCHES]: queryData.query } });
    if (queryData.tags && queryData.tags.length > 0)
      filter.AND?.push({ "properties.tags": { type: OperatorType.ARRAY, [Operator.CONTAINS_VALUE]: queryData.tags } });
    return filter;
  };

  useEffect(() => {
    if (!showReport) return;
    let filteredTrades = ApplyFilter(value?.trades || [], GenerateFilter());
    let report = GenerateReport(filteredTrades, settings?.summary_settings);
    setFinancialReport(report);
    setFinancialReportYears(GenerateYearlyReport(filteredTrades));
    console.log(GenerateYearlyReport(filteredTrades));
  }, [showReport]);

  return (
    <Box p={3}>
      <DataTableSearch
        className={`${classes.databaseTrades} ${useHasAlert() ? classes.alert : ""} `}
        idAccessor={"tradeTime"}
        query={queryData.query}
        hideComponents={showReport ? ["context", "table"] : []}
        onSearchChange={(query) => setQueryData((prev) => ({ ...prev, query }))}
        rightSectionWidth={35 * 2}
        rightSection={
          <Group gap={3}>
            <ActionWithTooltip
              tooltip={useTranslateButtons("show_financial_report_tooltip")}
              color={showReport ? "blue" : "gray"}
              icon={faCoins}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm" }}
              onClick={() => setShowReport((prev) => !prev)}
            />
          </Group>
        }
        filter={
          <Paper p={"sm"} mt={"md"}>
            <Group>
              <SelectItemTags value={queryData.tags || []} onChange={(value) => setQueryData((prev) => ({ ...prev, tags: value }))} />
            </Group>
          </Paper>
        }
        context={
          <Group gap={"md"} mt={"md"} grow>
            <Group>
              {Object.values([TauriTypes.TransactionType.Purchase, TauriTypes.TransactionType.Sale, TauriTypes.TransactionType.Trade]).map(
                (status) => (
                  <ColorInfo
                    active={status == queryData.type}
                    key={status}
                    onClick={() => setQueryData((prev) => ({ ...prev, type: status == prev.type ? undefined : status }))}
                    infoProps={{
                      "data-color-mode": "bg",
                      "data-transaction-type": status,
                    }}
                    text={useTranslateTransactionType(`${status}`)}
                    tooltip={useTranslateTransactionType(`details.${status}`)}
                  />
                ),
              )}
            </Group>
          </Group>
        }
        records={value?.trades || []}
        customRowAttributes={(record) => ({
          "data-color-mode": "box-shadow",
          "data-transaction-type": record.type,
        })}
        filters={GenerateFilter()}
        sorting={{
          field: queryData.sort_by,
          direction: queryData.sort_direction,
        }}
        columns={[
          {
            accessor: "created_at",
            title: useTranslateDataGridColumns("created_at"),
            sortable: true,
            render: ({ tradeTime }) => {
              return <Text>{dayjs(tradeTime).format("DD.MM.YYYY HH:mm")}</Text>;
            },
          },
          {
            accessor: "offered_items",
            title: useTranslateDataGridColumns("offered_items"),
            sortable: true,
            render: ({ offeredItems }) => <Text>{offeredItems.length}</Text>,
          },
          {
            accessor: "received_items",
            title: useTranslateDataGridColumns("received_items"),
            sortable: true,
            render: ({ receivedItems }) => <Text>{receivedItems.length}</Text>,
          },
          {
            accessor: "credits",
            title: useTranslateDataGridColumns("credits"),
            sortable: true,
            render: ({ credits }) => <NumberFormatter value={credits} thousandSeparator="," thousandsGroupStyle="thousand" />,
          },
        ]}
      />
      {showReport && (
        <Box mt={"md"}>
          <Grid>
            <Grid.Col span={6}>
              <FinancialReportCard data={financialReport} hidePercentBar />
            </Grid.Col>
            <Grid.Col span={6}>
              <BestByCategoryTable records={financialReport?.properties?.categories || []} />
            </Grid.Col>
          </Grid>

          <Grid mt={3}>
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
                  {financialReport?.properties?.most_purchased_items.map((item: { name: string; quantity: number }) => (
                    <Table.Tr key={item.name}>
                      <Table.Td>{item.name}</Table.Td>
                      <Table.Td>{item.quantity}</Table.Td>
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
                  {financialReport?.properties?.most_sold_items.map((item: { name: string; quantity: number }) => (
                    <Table.Tr key={item.name}>
                      <Table.Td>{item.name}</Table.Td>
                      <Table.Td>{item.quantity}</Table.Td>
                    </Table.Tr>
                  )) || null}
                </Table.Tbody>
              </Table>
            </Grid.Col>
            <Grid.Col span={6}>
              <BarCardChart
                title={useTranslate("yearly_trade_overview.title", { year: reportYear })}
                boxHeight={400}
                showDatasetLabels
                labels={i18next.t("months", { returnObjects: true }) as string[]}
                chartStyle={{ background: theme.colors.dark[7], height: "200px" }}
                datasets={[
                  {
                    label: useTranslate("yearly_trade_overview.total_trades"),
                    data: financialReportYears[reportYear]?.total_trades || [],
                    backgroundColor: theme.other.transactionType.trade,
                  },
                  {
                    label: useTranslate("yearly_trade_overview.total_purchases"),
                    data: financialReportYears[reportYear]?.total_purchases || [],
                    backgroundColor: theme.other.transactionType.purchase,
                  },
                  {
                    label: useTranslate("yearly_trade_overview.total_sales"),
                    data: financialReportYears[reportYear]?.total_sales || [],
                    backgroundColor: theme.other.transactionType.sale,
                  },
                ]}
                context={
                  <Group>
                    <Group flex={1}>
                      <BarChartFinancialSummary statistics={financialReportYears[reportYear]?.report} />
                    </Group>
                    <Group flex={"1 auto"} justify={"flex-end"}>
                      <Select w={100} data={Object.keys(financialReportYears)} value={reportYear} onChange={(value) => setReportYear(value || "")} />
                    </Group>
                  </Group>
                }
              />
            </Grid.Col>
          </Grid>
        </Box>
      )}
    </Box>
  );
};

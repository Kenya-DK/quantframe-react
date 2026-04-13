import { Box, Grid, Group, Select, Table, Text, Title, useMantineTheme } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { useEffect, useState } from "react";
import { ApplyFilter, ComplexFilter, Operator } from "@utils/filter.helper";
import { faCoins } from "@fortawesome/free-solid-svg-icons";
import dayjs from "dayjs";
import { DataTableSearch } from "@components/Shared/DataTableSearch";
import { FinancialReportCard } from "@components/Shared/FinancialReportCard";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { GenerateReport, GenerateYearlyReport } from "./helper";
import classes from "../../WFGDPR.module.css";
import { BarCardChart } from "@components/Shared/BarCardChart";
import i18next from "i18next";
import { BarChartFinancialSummary } from "@components/DataDisplay/BarChartFinancialSummary";

interface PurchasePanelProps {
  value: TauriTypes.WFGDPRAccount | null;
}
interface QueryData {
  query?: string;
  page: number;
  limit: number;
  sort_by: string;
  sort_direction: "asc" | "desc";
  from_date?: string;
  to_date?: string;
}
export const PurchasePanel = ({ value }: PurchasePanelProps) => {
  const theme = useMantineTheme();
  const [queryData, setQueryData] = useState<QueryData>({
    page: 1,
    limit: 50,
    sort_by: "date",
    sort_direction: "desc",
  });
  const [showReport, setShowReport] = useState<boolean>(false);
  const [financialReport, setFinancialReport] = useState<TauriTypes.FinancialReport | undefined>(undefined);
  const [financialReportYears, setFinancialReportYears] = useState<Record<string, { total_purchases: number[]; report: TauriTypes.FinancialReport }>>(
    {},
  );
  const [reportYear, setReportYear] = useState<string>(
    financialReportYears ? Object.keys(financialReportYears)[0] : new Date().getFullYear().toString(),
  );

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.tabs.wfgdpr.purchase.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`columns.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`buttons.${key}`, { ...context }, i18Key);

  const GenerateFilter = (): ComplexFilter => {
    let filter: ComplexFilter = { AND: [], OR: [] };
    if (queryData.query) filter.OR?.push({ "properties.names": { isCaseSensitive: false, [Operator.MATCHES]: queryData.query } });

    return filter;
  };

  useEffect(() => {
    if (!showReport) return;
    let filteredTrades = ApplyFilter(value?.purchases || [], GenerateFilter());
    let report = GenerateReport(filteredTrades);
    setFinancialReport(report);
    setFinancialReportYears(GenerateYearlyReport(filteredTrades));
    console.log(GenerateYearlyReport(filteredTrades));
  }, [showReport]);

  return (
    <Box p={3}>
      <DataTableSearch
        className={`${classes.databasePurchases} ${useHasAlert() ? classes.alert : ""} `}
        idAccessor={"tradeTime"}
        query={queryData.query}
        hideComponents={showReport ? ["context", "table"] : []}
        onSearchChange={(query) => setQueryData((prev) => ({ ...prev, query }))}
        rightSectionWidth={35 * 1}
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
        records={value?.purchases || []}
        filters={GenerateFilter()}
        sorting={{
          field: queryData.sort_by,
          direction: queryData.sort_direction,
        }}
        columns={[
          {
            accessor: "date",
            title: useTranslateDataGridColumns("date"),
            sortable: true,
            render: ({ date }) => {
              return <Text>{dayjs(date).format("DD.MM.YYYY HH:mm")}</Text>;
            },
          },
          {
            accessor: "price",
            title: useTranslateDataGridColumns("price"),
            sortable: true,
          },
          {
            accessor: "shop_id",
            title: useTranslateDataGridColumns("shop_id"),
            sortable: true,
          },
          {
            accessor: "items_received",
            title: useTranslateDataGridColumns("items_received"),
            sortable: true,
            render: ({ items_received }) => <Text>{items_received.length}</Text>,
          },
        ]}
      />
      {showReport && (
        <Box mt={"md"}>
          <Grid>
            <Grid.Col span={6}>
              <FinancialReportCard
                data={financialReport}
                hideComponents={["total_transactions", "trade_count", "revenue", "total_profit", "highest_revenue"]}
                hidePercentBar
              />
              <BarCardChart
                title={useTranslate("yearly_trade_overview.title", { year: reportYear })}
                boxHeight={300}
                labels={i18next.t("months", { returnObjects: true }) as string[]}
                chartStyle={{ background: theme.colors.dark[7], height: "200px" }}
                datasets={[
                  {
                    label: useTranslate("yearly_trade_overview.total_purchases"),
                    data: financialReportYears[reportYear]?.total_purchases || [],
                    backgroundColor: theme.other.transactionType.purchase,
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
            <Grid.Col span={6}>
              <Title order={4} mb={"sm"}>
                {useTranslate("titles.most_purchased_items")}
              </Title>
              <Table.ScrollContainer minWidth={"50%"} maxHeight={"calc(100vh - 500px)"}>
                <Table>
                  <Table.Thead>
                    <Table.Tr>
                      <Table.Th>{useTranslate("table_headers.item_name")}</Table.Th>
                      <Table.Th>{useTranslate("table_headers.price")}</Table.Th>
                      <Table.Th>{useTranslate("table_headers.quantity")}</Table.Th>
                    </Table.Tr>
                  </Table.Thead>
                  <Table.Tbody>
                    {financialReport?.properties?.most_purchased_items.map((item: { name: string; price: number; quantity: number }) => (
                      <Table.Tr key={item.name}>
                        <Table.Td>{item.name}</Table.Td>
                        <Table.Td>{item.price.toFixed(2)}</Table.Td>
                        <Table.Td>{item.quantity}</Table.Td>
                      </Table.Tr>
                    )) || null}
                  </Table.Tbody>
                </Table>
              </Table.ScrollContainer>
            </Grid.Col>
          </Grid>
        </Box>
      )}
    </Box>
  );
};

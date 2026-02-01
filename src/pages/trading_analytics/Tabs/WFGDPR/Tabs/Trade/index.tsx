import { Box, Grid, Group, Select, Table, Title, useMantineTheme } from "@mantine/core";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useQueries } from "./queries";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { BarCardChart } from "@components/Shared/BarCardChart";
import i18next from "i18next";
import { BestByCategoryTable } from "@components/DataDisplay/BestByCategoryTable";
import { BarChartFinancialSummary } from "@components/DataDisplay/BarChartFinancialSummary";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { useEffect, useState } from "react";
import { FinancialReportCard } from "@components/Shared/FinancialReportCard";

interface TradePanelProps {
  isActive?: boolean;
  year_list?: string[];
}

export const TradePanel = ({ isActive, year_list }: TradePanelProps) => {
  const theme = useMantineTheme();
  // States For DataGrid
  const queryData = useForm({
    initialValues: { page: 1, limit: 10, query: "", year: new Date().getFullYear() } as TauriTypes.WFGDPRTradeControllerGetListParams,
  });

  const [availableYears, setAvailableYears] = useState<{ label: string; value: string }[]>([]);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.tabs.wfgdpr.trade.${key}`, { ...context }, i18Key);
  const useTranslateCards = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`cards.${key}`, { ...context }, i18Key);

  // Queries
  const { financialReportQuery, refetchQueries } = useQueries({ queryData: queryData.values, isActive });
  const handleRefresh = () => {
    refetchQueries();
  };

  useEffect(() => {
    let years = year_list || [];
    if (!years.includes(new Date().getFullYear().toString())) {
      years = [...years, new Date().getFullYear().toString()];
    }
    const yearOptions = years.sort((a, b) => Number(b) - Number(a)).map((year) => ({ label: year, value: year }));
    setAvailableYears(yearOptions);
  }, [year_list]);

  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshWFGDPRAll, handleRefresh, []);
  return (
    <Box p={"md"} h={"85vh"}>
      <Grid>
        <Grid.Col span={6}>
          <FinancialReportCard data={financialReportQuery.data} loading={financialReportQuery.isLoading} />
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
                    data={availableYears}
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

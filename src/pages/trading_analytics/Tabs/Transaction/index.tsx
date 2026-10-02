import { TauriTypes } from "$types";
import { HasPermission } from "@api/index";
import { ItemName } from "@components/DataDisplay/ItemName";
import { SearchField } from "@components/Forms/SearchField";
import { SelectItemTags } from "@components/Forms/SelectItemTags";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { ColorInfo } from "@components/Shared/ColorInfo";
import { FinancialReportCard } from "@components/Shared/FinancialReportCard";
import { Loading } from "@components/Shared/Loading";
import { faCalculator, faChartLine, faCoins, faDownload, faHammer, faList, faTrash } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { Box, Grid, Group, NumberFormatter, Paper, SegmentedControl, Select, Table, Text, Title } from "@mantine/core";
import { DatePickerInput } from "@mantine/dates";
import { useLocalStorage } from "@mantine/hooks";
import { getSafePage } from "@utils/helper";
import dayjs from "dayjs";
import { DataTable } from "mantine-datatable";
import { useEffect, useMemo, useState, useTransition } from "react";
import classes from "../../TradingAnalytics.module.css";
import { useModals } from "./modals";
import { useMutations } from "./mutations";
import { ChartView } from "./views";
import { useQueries } from "./queries";
interface TransactionPanelProps {
  isActive?: boolean;
}

export const TransactionPanel = ({ isActive }: TransactionPanelProps = {}) => {
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.TransactionControllerGetListParams>({
    key: "transaction_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50, sort_by: "created_at", sort_direction: "desc" },
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.transaction.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateTransactionType = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`transaction_type.${key}`, { ...context }, i18Key);
  const useTranslateTransactionItemType = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`item_type.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateView = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`view.${key}`, { ...context }, i18Key);
  const useTranslateChart = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`chart.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`prompts.${key}`, { ...context }, i18Key);
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`prompts.${key}`, { ...context }, i18Key);

  // States
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.TransactionDto[]>([]); // New state for selected records
  const [showReport, setShowReport] = useState<boolean>(false);
  const [filterOpened, setFilterOpened] = useState<boolean>(false);
  const [canExport, setCanExport] = useState<boolean>(false);
  const [viewMode, setViewMode] = useLocalStorage<string>({
    key: "transaction_view_mode",
    getInitialValueInEffect: false,
    defaultValue: "list",
  });
  const [chartRange, setChartRange] = useLocalStorage<string>({
    key: "transaction_chart_range",
    getInitialValueInEffect: false,
    defaultValue: "90",
  });

  // Progressive rendering === allows navigation during table render (without it ui freezes until table loads)
  const [isPending, startTransition] = useTransition();
  const [displayedRecords, setDisplayedRecords] = useState<TauriTypes.TransactionDto[]>([]);

  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA).then((res) => setCanExport(res));
  }, []);

  // Chart range is independent of the list filter; it defaults to the last 90 days.
  const chartParams = useMemo<TauriTypes.TransactionControllerGetListParams>(() => {
    const base = { ...queryData, page: 1, limit: -1 };
    if (chartRange === "all") return { ...base, from_date: undefined, to_date: undefined };
    const to = dayjs();
    const from = to.subtract(Number(chartRange), "day");
    return { ...base, from_date: from.format("YYYY-MM-DD"), to_date: to.format("YYYY-MM-DD") };
  }, [queryData, chartRange]);

  // Queries
  const { paginationQuery, financialReportQuery, chartQuery, refetchQueries } = useQueries({
    queryData,
    chartParams,
    isActive,
    loadFinancialReport: showReport,
    loadChart: viewMode === "chart",
  });
  const handleRefresh = () => {
    console.log("Refreshing transactions due to Tauri event");
    refetchQueries();
  };

  // Mutations
  const { exportMutation, updateMutation, deleteMutation, deleteMultipleMutation, calculateTaxMutation } = useMutations({
    refetchQueries,
    setLoadingRows,
  });

  // Modals
  const { OpenDeleteModal, OpenUpdateModal, OpenDeleteBulkModal } = useModals({
    refetchQueries,
    deleteMutation,
    setLoadingRows,
    updateMutation,
    deleteMultipleMutation,
    useTranslateBasePrompt,
    useTranslatePrompt,
  });

  useEffect(() => {
    setSelectedRecords([]);
  }, [deleteMultipleMutation.isSuccess, deleteMutation.isSuccess]);

  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshTransactions, handleRefresh, []);

  // Progressive rendering
  useEffect(() => {
    if (paginationQuery.isFetching) {
      setDisplayedRecords([]);
    } else if (paginationQuery.data?.results) {
      startTransition(() => {
        setDisplayedRecords(paginationQuery.data?.results || []);
      });
    }
  }, [paginationQuery.isFetching, paginationQuery.data?.results]);

  return (
    <Box p={"md"}>
      <SearchField
        value={queryData.query || ""}
        onChange={(value) => setQueryData((prev) => ({ ...prev, query: value }))}
        onSearch={() => refetchQueries()}
        onFilterToggle={(s) => setFilterOpened(s)}
        filter={
          <Paper p={"sm"} mt={"md"}>
            <Group>
              <SelectItemTags value={queryData.tags || []} onChange={(value) => setQueryData((prev) => ({ ...prev, tags: value }))} />
              <DatePickerInput
                clearable
                label={useTranslateTabItem("date_range_label")}
                description={useTranslateTabItem("date_range_description")}
                placeholder={useTranslateTabItem("date_range_placeholder")}
                w={200}
                type="range"
                valueFormat="YYYY MMM DD"
                value={[queryData.from_date ? new Date(queryData.from_date) : null, queryData.to_date ? new Date(queryData.to_date) : null]}
                onChange={(value) => {
                  let [start, end] = value || [undefined, undefined];
                  setQueryData((prev) => ({ ...prev, from_date: start || undefined, to_date: end || undefined }));
                }}
              />
            </Group>
          </Paper>
        }
        rightSectionWidth={35 * 5}
        rightSection={
          <Group gap={3}>
            <ActionWithTooltip
              tooltip={useTranslateButtons("export_transactions_tooltip")}
              icon={faDownload}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: !canExport }}
              onClick={() => exportMutation.mutate(queryData)}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("show_financial_report_tooltip")}
              color={showReport ? "blue" : "gray"}
              icon={faCoins}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm" }}
              onClick={() => setShowReport((prev) => !prev)}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("calculate_tax_tooltip")}
              icon={faCalculator}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm" }}
              onClick={() => calculateTaxMutation.mutate(undefined)}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("delete_all_tooltip", { count: selectedRecords.length })}
              color={"red.7"}
              icon={faTrash}
              iconProps={{ size: "xs" }}
              actionProps={{
                size: "sm",
                disabled: selectedRecords.length == 0 || deleteMutation.isPending,
              }}
              onClick={() => OpenDeleteBulkModal(selectedRecords.map((record) => record.id))}
            />
          </Group>
        }
      />
      {!showReport && (
        <Box>
          <Group gap={"md"} mt={"md"} grow>
            <Group>
              {Object.values([TauriTypes.TransactionType.Purchase, TauriTypes.TransactionType.Sale]).map((status) => (
                <ColorInfo
                  active={status == queryData.transaction_type}
                  key={status}
                  onClick={() => setQueryData((prev) => ({ ...prev, transaction_type: status == prev.transaction_type ? undefined : status }))}
                  infoProps={{
                    "data-color-mode": "bg",
                    "data-transaction-type": status,
                  }}
                  text={useTranslateTransactionType(`${status}`)}
                  tooltip={useTranslateTransactionType(`details.${status}`)}
                />
              ))}
            </Group>
            <Group justify="center">
              <SegmentedControl
                size="xs"
                radius="md"
                value={viewMode}
                onChange={setViewMode}
                data={[
                  {
                    value: "list",
                    label: (
                      <Group gap={6} wrap="nowrap">
                        <FontAwesomeIcon icon={faList} />
                        <span>{useTranslateView("list")}</span>
                      </Group>
                    ),
                  },
                  {
                    value: "chart",
                    label: (
                      <Group gap={6} wrap="nowrap">
                        <FontAwesomeIcon icon={faChartLine} />
                        <span>{useTranslateView("chart")}</span>
                      </Group>
                    ),
                  },
                ]}
              />
            </Group>
            <Group justify="flex-end">
              {Object.values(TauriTypes.TransactionItemType).map((type) => (
                <ColorInfo
                  active={type == queryData.item_type}
                  key={type}
                  onClick={() => setQueryData((prev) => ({ ...prev, item_type: type == prev.item_type ? undefined : type }))}
                  infoProps={{
                    "data-color-mode": "bg",
                    "data-item-type": type,
                  }}
                  text={useTranslateTransactionItemType(`${type}`)}
                  tooltip={useTranslateTransactionItemType(`details.${type}`)}
                />
              ))}
            </Group>
          </Group>
          {viewMode === "chart" && (
            <Box mt={"md"}>
              <Group justify="flex-end" mb={"sm"}>
                <Select
                  allowDeselect={false}
                  label={useTranslateChart("range.label")}
                  data={[
                    { value: "7", label: useTranslateChart("range.options.7") },
                    { value: "30", label: useTranslateChart("range.options.30") },
                    { value: "90", label: useTranslateChart("range.options.90") },
                    { value: "365", label: useTranslateChart("range.options.365") },
                    { value: "all", label: useTranslateChart("range.options.all") },
                  ]}
                  value={chartRange}
                  onChange={(value) => value && setChartRange(value)}
                  w={200}
                  radius="md"
                />
              </Group>
              {chartQuery.isFetching ? <Loading /> : <ChartView transactions={chartQuery.data?.results || []} />}
            </Box>
          )}
          {viewMode === "list" && (
            <DataTable
              className={`${classes.databaseTransactions} ${useHasAlert() ? classes.alert : ""} ${filterOpened ? classes.filterOpened : ""}`}
              mt={"md"}
              striped
              fetching={paginationQuery.isFetching || isPending || calculateTaxMutation.isPending}
              records={displayedRecords}
              page={getSafePage(queryData.page, paginationQuery.data?.total_pages)}
              onPageChange={(page) => setQueryData((prev) => ({ ...prev, page }))}
              totalRecords={paginationQuery.data?.total || 0}
              recordsPerPage={queryData.limit || 10}
              recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
              onRecordsPerPageChange={(limit) => setQueryData((prev) => ({ ...prev, limit }))}
              customRowAttributes={(record) => {
                return {
                  "data-color-mode": "box-shadow",
                  "data-transaction-type": record.transaction_type,
                };
              }}
              selectedRecords={selectedRecords}
              onSelectedRecordsChange={setSelectedRecords}
              sortStatus={{
                columnAccessor: queryData.sort_by || "name",
                direction: queryData.sort_direction || "desc",
              }}
              onSortStatusChange={(sort) => {
                if (!sort || !sort.columnAccessor) return;
                setQueryData((prev) => ({ ...prev, sort_by: sort.columnAccessor as string, sort_direction: sort.direction }));
              }}
              // define columns
              columns={[
                {
                  accessor: "item_name",
                  title: useTranslateCommon("item_name.title"),
                  sortable: true,
                  width: 250,
                  render: (row) => <ItemName color="gray.4" size="md" value={row} />,
                },
                {
                  accessor: "item_type",
                  title: useTranslateDataGridColumns("item_type"),
                  sortable: true,
                  render: ({ item_type }) => (
                    <Text data-color-mode="text" data-item-type={item_type}>
                      {useTranslateTransactionItemType(item_type)}
                    </Text>
                  ),
                },
                {
                  accessor: "user_name",
                  title: useTranslateDataGridColumns("user_name"),
                  sortable: true,
                },
                {
                  accessor: "quantity",
                  title: useTranslateCommon("datatable_columns.quantity.title"),
                  sortable: true,
                },
                {
                  accessor: "price",
                  title: useTranslateDataGridColumns("price"),
                  sortable: true,
                },
                {
                  accessor: "profit",
                  title: useTranslateDataGridColumns("profit"),
                  sortable: true,
                  render: ({ profit }) => (profit ? <Text c={profit >= 0 ? "green.7" : "red.7"}>{profit.toFixed(2)}</Text> : <Text>N/A</Text>),
                },
                {
                  accessor: "credits",
                  title: useTranslateDataGridColumns("credits"),
                  sortable: true,
                  render: ({ credits }) => <NumberFormatter value={credits} thousandSeparator="," thousandsGroupStyle="thousand" />,
                },
                {
                  accessor: "created_at",
                  title: useTranslateDataGridColumns("created_at"),
                  sortable: true,
                  render: ({ created_at }) => {
                    return <Text>{dayjs(created_at).format("DD.MM.YYYY HH:mm")}</Text>;
                  },
                },
                {
                  accessor: "actions",
                  title: useTranslateCommon("datatable_columns.actions.title"),
                  width: 75,
                  render: (row) => (
                    <Group gap={3}>
                      <ActionWithTooltip
                        tooltip={useTranslateCommon("datatable_columns.actions.buttons.edit_tooltip")}
                        icon={faHammer}
                        loading={loadingRows.includes(`${row.id}`)}
                        iconProps={{ size: "xs" }}
                        actionProps={{ size: "sm" }}
                        onClick={async (e) => {
                          e.stopPropagation();
                          OpenUpdateModal(row);
                        }}
                      />
                      <ActionWithTooltip
                        tooltip={useTranslateCommon("datatable_columns.actions.buttons.delete_tooltip")}
                        icon={faTrash}
                        color="red"
                        loading={loadingRows.includes(`${row.id}`)}
                        iconProps={{ size: "xs" }}
                        actionProps={{ size: "sm" }}
                        onClick={async (e) => {
                          e.stopPropagation();
                          OpenDeleteModal(row.id);
                        }}
                      />
                    </Group>
                  ),
                },
              ]}
            />
          )}
        </Box>
      )}
      {showReport && (
        <Box mt={"md"}>
          <Grid>
            <Grid.Col span={6}>
              <FinancialReportCard data={financialReportQuery.data} loading={financialReportQuery.isLoading} hideTradeCount />
            </Grid.Col>
            <Grid.Col span={3}>
              <Title order={4} mb={"sm"}>
                {useTranslateTabItem("titles.most_purchased_items")}
              </Title>
              <Table>
                <Table.Thead>
                  <Table.Tr>
                    <Table.Th>{useTranslateTabItem("table_headers.item_name")}</Table.Th>
                    <Table.Th>{useTranslateTabItem("table_headers.quantity")}</Table.Th>
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
                {useTranslateTabItem("titles.most_sold_items")}
              </Title>
              <Table>
                <Table.Thead>
                  <Table.Tr>
                    <Table.Th>{useTranslateTabItem("table_headers.item_name")}</Table.Th>
                    <Table.Th>{useTranslateTabItem("table_headers.quantity")}</Table.Th>
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
          <DataTable
            className={`${classes.databaseTradingPartners} ${useHasAlert() ? classes.alert : ""} ${filterOpened ? classes.filterOpened : ""}`}
            mt={"md"}
            striped
            fetching={paginationQuery.isFetching || calculateTaxMutation.isPending}
            records={financialReportQuery.data?.properties.trading_partners || []}
            idAccessor={"properties.user"}
            // define columns
            columns={[
              {
                accessor: "user_name",
                title: useTranslateDataGridColumns("user_name"),
                render: ({ properties }) => properties.user,
              },
              {
                accessor: "sale_count",
                title: useTranslateDataGridColumns("sale_count"),
              },
              {
                accessor: "revenue",
                title: useTranslateDataGridColumns("revenue"),
              },
              {
                accessor: "purchases_count",
                title: useTranslateDataGridColumns("purchases_count"),
              },
              {
                accessor: "expenses",
                title: useTranslateDataGridColumns("expenses"),
              },
              {
                accessor: "total_transactions",
                title: useTranslateDataGridColumns("total_transactions"),
              },
            ]}
          />
        </Box>
      )}
    </Box>
  );
};

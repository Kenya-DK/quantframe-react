import { Box, Grid, Group, NumberFormatter, Paper, Table, Text, Title } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { TauriTypes } from "$types";
import { useQueries } from "./queries";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { SearchField } from "@components/Forms/SearchField";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import classes from "../../TradingAnalytics.module.css";
import { DataTable } from "mantine-datatable";
import { getSafePage } from "@utils/helper";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { ColorInfo } from "@components/Shared/ColorInfo";
import { SelectItemTags } from "@components/Forms/SelectItemTags";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faCalculator, faCoins, faDownload, faHammer, faTrash } from "@fortawesome/free-solid-svg-icons";
import { useMutations } from "./mutations";
import { useModals } from "./modals";
import { HasPermission } from "@api/index";
import { DatePickerInput } from "@mantine/dates";
import dayjs from "dayjs";
import { ItemName } from "@components/DataDisplay/ItemName";
import { FinancialReportCard } from "../../../../components/Shared/FinancialReportCard";
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

  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA).then((res) => setCanExport(res));
  }, []);

  // Queries
  const { paginationQuery, financialReportQuery, refetchQueries } = useQueries({ queryData, isActive });
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

  return (
    <Box p={"md"}>
      {" "}
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
          <DataTable
            className={`${classes.databaseTransactions} ${useHasAlert() ? classes.alert : ""} ${filterOpened ? classes.filterOpened : ""}`}
            mt={"md"}
            striped
            fetching={paginationQuery.isLoading || calculateTaxMutation.isPending}
            records={paginationQuery.data?.results || []}
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
            fetching={paginationQuery.isLoading || calculateTaxMutation.isPending}
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

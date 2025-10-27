import { Box, Grid, Group, Paper, Text } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { TauriTypes } from "$types";
import { useQueries } from "./queries";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { SearchField } from "@components/Forms/SearchField";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import classes from "../../TradingAnalytics.module.css";
import { DataTable } from "mantine-datatable";
import { getSafePage, GetSubTypeDisplay } from "@utils/helper";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { ColorInfo } from "@components/Shared/ColorInfo";
import { SelectItemTags } from "@components/Forms/SelectItemTags";
import { FinancialReportCard } from "@components/Shared/FinancialReportCard";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faCoins, faDownload, faHammer, faTrash } from "@fortawesome/free-solid-svg-icons";
import { useMutations } from "./mutations";
import { useModals } from "./modals";
import { HasPermission } from "@api/index";
import { DatePickerInput } from "@mantine/dates";
import dayjs from "dayjs";
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
    refetchQueries();
  };

  // Mutations
  const { exportMutation, updateMutation, deleteMutation, deleteBulkMutation } = useMutations({
    refetchQueries,
    setLoadingRows,
    setSelectedRecords,
  });

  // Modals
  const { OpenDeleteModal, OpenUpdateModal, OpenDeleteBulkModal } = useModals({
    refetchQueries,
    deleteMutation,
    setLoadingRows,
    updateMutation,
    deleteBulkMutation,
    useTranslateBasePrompt,
    useTranslatePrompt,
  });

  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshTransactions, handleRefresh, [refetchQueries]);

  return (
    <Box p={"md"}>
      <Grid>
        <Grid.Col span={showReport ? 7 : 12}>
          <SearchField
            value={queryData.query || ""}
            onChange={(value) => setQueryData((prev) => ({ ...prev, query: value }))}
            onSearch={() => refetchQueries()}
            hideSearch
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
            rightSectionWidth={115}
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
            fetching={paginationQuery.isLoading}
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
                render: ({ item_name, sub_type, properties }) => (
                  <TextTranslate
                    color="gray.4"
                    i18nKey={useTranslateCommon("item_name.value", undefined, true)}
                    values={{
                      name: `${item_name} ${properties ? " " + properties.mod_name : ""}`,
                      sub_type: GetSubTypeDisplay(sub_type),
                    }}
                  />
                ),
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
        </Grid.Col>
        {showReport && (
          <Grid.Col span={5}>
            <FinancialReportCard data={financialReportQuery.data || null} loading={financialReportQuery.isLoading} />
          </Grid.Col>
        )}
      </Grid>
    </Box>
  );
};

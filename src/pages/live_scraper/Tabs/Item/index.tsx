import { Box, Grid, Group, NumberFormatter } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { TauriTypes } from "$types";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import classes from "../../LiveScraper.module.css";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useStockQueries } from "./queries";
import { useEffect, useState } from "react";
import { useModals } from "./modals";
import { DataTable } from "mantine-datatable";
import { CreateItemForm } from "@components/Forms/CreateItem";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";
import { SearchField } from "@components/Forms/SearchField";
import { ColorInfo } from "@components/Shared/ColorInfo";
import { notifications } from "@mantine/notifications";
import { GetChatLinkNameMultiple, getSafePage } from "@utils/helper";
import { ColumnMinMaxPrice } from "../../Columns/ColumnMinMaxPrice";
import { ColumnActions } from "../../Columns/ColumnActions";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faDownload, faEdit, faMessage, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { HasPermission } from "@api/index";
import { ItemName } from "@components/DataDisplay/ItemName/ItemName";
import { useMutations } from "./mutations";

interface ItemPanelProps {
  isActive?: boolean;
}

export const ItemPanel = ({ isActive }: ItemPanelProps = {}) => {
  // Contexts
  const { is_running } = useLiveScraperContext();
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.StockItemControllerGetListParams>({
    key: "stock_item_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10 },
  });
  // States
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [canExport, setCanExport] = useState<boolean>(false);
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.StockItem[]>([]);

  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA).then((res) => setCanExport(res));
  }, []);

  // Translate
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`live_scraper.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.item.${key}`, { ...context }, i18Key);
  const useTranslateSegments = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`segments.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`datatable.columns.${key}`, { ...context }, i18Key);
  // Queries
  const { paginationQuery, financialReportQuery, statusCountsQuery, refetchQueries } = useStockQueries({ queryData, isActive });

  // Mutations
  const { createMutation, updateMutation, updateMultipleMutation, exportMutation, sellStockMutation, deleteMutation, deleteMultipleMutation } =
    useMutations({
      refetchQueries,
      setLoadingRows,
    });
  // Modals
  const { OpenMinimumPriceModal, OpenWTSModal, OpenDeleteMultipleModal, OpenUpdateMultipleModal, OpenSellModal, OpenInfoModal, OpenDeleteModal } =
    useModals({
      updateMutation,
      updateMultipleMutation,
      sellStockMutation,
      deleteMutation,
      deleteMultipleMutation,
    });
  const handleRefresh = (_data: any) => {
    refetchQueries(true);
  };

  useEffect(() => {
    setSelectedRecords([]);
  }, [deleteMultipleMutation.isSuccess, deleteMutation.isSuccess]);

  const hasOverride = (record: TauriTypes.StockItem) => {
    return record.minimum_sma != null || record.minimum_profit != null;
  };

  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshStockItems, handleRefresh, [refetchQueries]);
  return (
    <Box>
      <Grid>
        <Grid.Col span={8}>
          <CreateItemForm onSubmit={(values) => createMutation.mutateAsync(values)} />
          <Group gap={"md"} mt={"md"}>
            {Object.entries(statusCountsQuery.data || {})
              .sort(([a], [b]) => a.localeCompare(b))
              .map(([key, count]) => (
                <ColorInfo
                  active={key == queryData.status}
                  key={key}
                  onClick={() =>
                    setQueryData((prev) => ({
                      ...prev,
                      status: (key as TauriTypes.StockStatus) == prev.status ? undefined : (key as TauriTypes.StockStatus),
                    }))
                  }
                  infoProps={{
                    "data-color-mode": "bg",
                    "data-stock-status": key,
                  }}
                  text={useTranslateStockStatus(`${key}`) + ` (${count})`}
                  tooltip={useTranslateStockStatus(`details.${key}`)}
                />
              ))}
          </Group>
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithSegments
            showPercent
            percentSymbol="%"
            segments={[
              { label: useTranslateSegments("bought"), count: financialReportQuery.data?.expenses || 0, color: "var(--qf-negative-color)" },
              { label: useTranslateSegments("listed"), count: financialReportQuery.data?.revenue || 0, color: "var(--qf-positive-color)" },
              { label: useTranslateSegments("profit"), count: financialReportQuery.data?.total_profit || 0, color: "var(--qf-profit)" },
            ]}
          />
        </Grid.Col>
      </Grid>
      <SearchField
        value={queryData.query || ""}
        onChange={(value) => setQueryData((prev) => ({ ...prev, query: value }))}
        rightSectionWidth={30 * 4}
        rightSection={
          <Group gap={3}>
            <ActionWithTooltip
              tooltip={useTranslate("export_json_tooltip")}
              icon={faDownload}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: !canExport }}
              onClick={() => exportMutation.mutate(queryData)}
            />
            <ActionWithTooltip
              tooltip={useTranslate("update_multiple_tooltip")}
              icon={faEdit}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: selectedRecords.length === 0 }}
              onClick={() => OpenUpdateMultipleModal(selectedRecords.map((r) => r.id))}
            />
            <ActionWithTooltip
              tooltip={useTranslate("wts_multiple_tooltip")}
              icon={faMessage}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: selectedRecords.length === 0 }}
              onClick={async () => {
                let filteredRecords = selectedRecords.filter((r) => r.list_price && r.list_price > 0);
                OpenWTSModal({
                  prefix: "WTS ",
                  suffix: " :heart:",
                  items: (await GetChatLinkNameMultiple(filteredRecords)).map((chatLink, index) => {
                    chatLink.suffix += `<SP>${filteredRecords[index].list_price || 0}p`;
                    return chatLink;
                  }),
                });
              }}
            />
            <ActionWithTooltip
              tooltip={useTranslate("delete_multiple_tooltip")}
              icon={faTrashCan}
              color="red.7"
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: selectedRecords.length === 0 }}
              onClick={() => OpenDeleteMultipleModal(selectedRecords.map((r) => r.id))}
            />
          </Group>
        }
      />
      <DataTable
        className={`${classes.databaseStockItems} ${useHasAlert() ? classes.alert : ""} ${is_running ? classes.running : ""}`}
        customRowAttributes={(record) => {
          return {
            "data-color-mode": "box-shadow",
            "data-stock-status": record.status,
          };
        }}
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
        sortStatus={{
          columnAccessor: queryData.sort_by || "name",
          direction: queryData.sort_direction || "desc",
        }}
        onSortStatusChange={(sort) => {
          if (!sort || !sort.columnAccessor) return;
          setQueryData((prev) => ({ ...prev, sort_by: sort.columnAccessor as string, sort_direction: sort.direction }));
        }}
        onCellClick={({ record, column }) => {
          switch (column.accessor) {
            case "item_name":
              let name = record.item_name;
              navigator.clipboard.writeText(name);
              notifications.show({
                title: useTranslateCommon("notifications.copy_to_clipboard.title"),
                message: useTranslateCommon("notifications.copy_to_clipboard.message", { message: name }),
                color: "green.7",
              });
              break;
          }
        }}
        selectedRecords={selectedRecords}
        onSelectedRecordsChange={setSelectedRecords}
        // define columns
        columns={[
          {
            accessor: "item_name",
            title: useTranslateCommon("item_name.title"),
            sortable: true,
            render: (row) => <ItemName color="gray.4" size="md" value={row} />,
          },
          {
            accessor: "bought",
            title: useTranslateDataGridColumns("bought"),
            sortable: true,
            render: ({ bought }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={bought} />,
          },
          {
            accessor: "minimum_price",
            width: 310,
            sortable: true,
            title: useTranslateCommon("datatable_columns.minimum_price.title"),
            render: ({ id, minimum_price, list_price }) => (
              <ColumnMinMaxPrice
                id={id}
                minimum_price={minimum_price}
                onUpdate={async (id: number, minimum_price: number) => await updateMutation.mutateAsync({ id, minimum_price, list_price })}
                onEdit={async (id: number, minimum_price: number) => OpenMinimumPriceModal(id, minimum_price)}
              />
            ),
          },
          {
            accessor: "list_price",
            sortable: true,
            title: useTranslateCommon("datatable_columns.list_price"),
          },
          {
            accessor: "owned",
            sortable: true,
            title: useTranslateDataGridColumns("owned"),
          },
          {
            accessor: "actions",
            title: useTranslateCommon("datatable_columns.actions.title"),
            width: 215,
            render: (row) => (
              <ColumnActions
                row={row}
                buttonProps={{ edit: { color: hasOverride(row) ? "yellow.7" : "blue.7" } }}
                hideButtons={["open_filter"]}
                loadingRows={loadingRows}
                onManual={() => OpenSellModal(row)}
                onAuto={(price) => sellStockMutation.mutateAsync({ ...row, price, quantity: 1 })}
                onInfo={() => OpenInfoModal(row)}
                onHide={(hide) => updateMutation.mutateAsync({ id: row.id, is_hidden: hide, list_price: row.list_price })}
                onDelete={() => OpenDeleteModal(row.id)}
                onEdit={() => OpenUpdateMultipleModal([row.id])}
              />
            ),
          },
        ]}
      />
    </Box>
  );
};

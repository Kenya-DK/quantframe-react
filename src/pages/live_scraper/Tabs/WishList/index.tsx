import { Box, Grid, Group } from "@mantine/core";
import { SearchField } from "@components/Forms/SearchField";
import { useLocalStorage } from "@mantine/hooks";
import { TauriTypes } from "$types";
import { CreateItemForm } from "@components/Forms/CreateItem/CreateItemForm";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";
import { ColorInfo } from "@components/Shared/ColorInfo";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { DataTable } from "mantine-datatable";
import classes from "../../LiveScraper.module.css";
import { notifications } from "@mantine/notifications";
import { GetItemDisplay, getSafePage, GetSubTypeDisplay } from "@utils/helper";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useWishListQueries } from "./queries";
import { useWishListMutations } from "./mutations";
import { useEffect, useState } from "react";
import { useStockModals } from "./modals";
import { ColumnActions } from "../../Columns/ColumnActions";
import { ColumnMinMaxPrice } from "../../Columns/ColumnMinMaxPrice";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faDownload, faEdit, faMessage, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { HasPermission } from "@api/index";
import { ItemName } from "@components/DataDisplay/ItemName/ItemName";

interface WishListPanelProps {
  isActive?: boolean;
}

export const WishListPanel = ({ isActive }: WishListPanelProps = {}) => {
  // Contexts
  const { is_running } = useLiveScraperContext();
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.WishListControllerGetListParams>({
    key: "wish_list_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10 },
  });
  // States
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [canExport, setCanExport] = useState<boolean>(false);
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.WishListItem[]>([]);

  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA).then((res) => setCanExport(res));
  }, []);

  // Translate
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`live_scraper.${key}`, { ...context }, i18Key);
  const useTranslateSegments = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`segments.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  // Queries
  const { paginationQuery, financialReportQuery, statusCountsQuery, refetchQueries } = useWishListQueries({ queryData, isActive });

  // Mutations
  const { createMutation, boughtMutation, exportMutation, updateMutation, deleteMutation, deleteMultipleMutation, updateMultipleMutation } =
    useWishListMutations({
      refetchQueries,
      setLoadingRows,
    });
  // Modals
  const { OpenMinimumPriceModal, OpenWTBModal, OpenUpdateMultipleModal, OpenDeleteModal, OpenDeleteMultipleModal, OpenBoughtModal, OpenInfoModal } =
    useStockModals({
      updateMutation,
      updateMultipleMutation,
      boughtMutation,
      deleteMutation,
      deleteMultipleMutation,
    });
  const handleRefresh = (_data: any) => {
    refetchQueries(true);
  };
  useEffect(() => {
    setSelectedRecords([]);
  }, [deleteMultipleMutation.isSuccess, deleteMutation.isSuccess]);
  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshWishListItems, handleRefresh, [refetchQueries]);
  return (
    <Box>
      <Grid>
        <Grid.Col span={8}>
          <CreateItemForm hide_bought onSubmit={(values) => createMutation.mutateAsync(values)} />
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
            segments={[{ label: useTranslateSegments("listed"), count: financialReportQuery.data?.revenue || 0, color: "var(--qf-positive-color)" }]}
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
              onClick={() =>
                OpenWTBModal({
                  prefix: "WTS ",
                  suffix: " :heart:",
                  items: selectedRecords
                    .filter((r) => r.item_name && r.list_price)
                    .map((r) => ({
                      name: `${GetItemDisplay(r)}`,
                      suffix: GetSubTypeDisplay(r)?.replace("(", "").replace(")", ""),
                      price: r.list_price || 0,
                    })),
                })
              }
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
            render: (row) => <ItemName hideQuantity color="gray.4" size="md" value={row} />,
          },
          {
            accessor: "quantity",
            title: useTranslateCommon("datatable_columns.quantity.title"),
          },
          {
            accessor: "maximum_price",
            width: 310,
            sortable: true,
            title: useTranslateCommon("datatable_columns.maximum_price.title"),
            render: ({ id, maximum_price }) => (
              <ColumnMinMaxPrice
                i18nKey="maximum_price"
                id={id}
                minimum_price={maximum_price}
                onUpdate={async (id: number, minimum_price: number) => await updateMutation.mutateAsync({ id, maximum_price: minimum_price })}
                onEdit={async (id: number, minimum_price: number) => OpenMinimumPriceModal(id, minimum_price)}
              />
            ),
          },
          {
            accessor: "list_price",
            title: useTranslateCommon("datatable_columns.list_price"),
          },
          {
            accessor: "actions",
            title: useTranslateCommon("datatable_columns.actions.title"),
            width: 215,
            render: (row) => (
              <ColumnActions
                row={row}
                i18nKeyOverride={{
                  manual_tooltip: "bought_manual_tooltip",
                  auto_tooltip: "bought_auto_tooltip",
                }}
                hideButtons={["open_filter"]}
                loadingRows={loadingRows}
                onManual={() => OpenBoughtModal(row)}
                onAuto={(price) => boughtMutation.mutateAsync({ ...row, price })}
                onInfo={() => OpenInfoModal(row)}
                onHide={(hide) => updateMutation.mutateAsync({ id: row.id, is_hidden: hide })}
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

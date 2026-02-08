import { ActionIcon, Box, Grid, Group, NumberFormatter, Paper, Tooltip } from "@mantine/core";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useLocalStorage } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { useStockQueries } from "./queries";
import { ColorInfo } from "@components/Shared/ColorInfo";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";
import { SearchField } from "@components/Forms/SearchField";
import classes from "../../LiveScraper.module.css";
import { DataTable } from "mantine-datatable";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { getSafePage } from "@utils/helper";
import { useStockMutations } from "./mutations";
import { useStockModals } from "./modals";
import { ColumnMinMaxPrice } from "../../Columns/ColumnMinMaxPrice";
import { ColumnActions } from "../../Columns/ColumnActions";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faDownload, faEdit, faInfo, faMessage, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { HasPermission } from "@api/index";
import { notifications } from "@mantine/notifications";
import { ItemName } from "@components/DataDisplay/ItemName/ItemName";
import { RivenAttribute } from "@components/DataDisplay/RivenAttribute";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
export type RivenPanelProps = {
  isActive?: boolean;
};

export const RivenPanel = ({ isActive }: RivenPanelProps = {}) => {
  // Responsive
  // Contexts
  const { is_running } = useLiveScraperContext();
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.StockItemControllerGetListParams>({
    key: "stock_riven_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10 },
  });
  // States
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [canExport, setCanExport] = useState<boolean>(false);
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.StockRiven[]>([]);

  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA).then((res) => setCanExport(res));
  }, []);

  // Translate
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`live_scraper.${key}`, { ...context }, i18Key);
  const useTranslateTabRiven = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.riven.${key}`, { ...context }, i18Key);
  const useTranslateSegments = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`segments.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabRiven(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`prompts.${key}`, { ...context }, i18Key);
  // Queries
  const { paginationQuery, financialReportQuery, statusCountsQuery, refetchQueries } = useStockQueries({ queryData, isActive });
  const handleRefresh = (data: { id: string }) => {
    if (data.id) setSelectedRecords((prev) => prev.filter((record) => record.id !== Number(data.id)));
    refetchQueries(true);
  };
  // Mutations
  const { updateMutation, createMutation, exportMutation, sellMutation, deleteMutation, updateMultipleMutation, deleteMultipleMutation } =
    useStockMutations({
      refetchQueries,
      setLoadingRows,
    });
  // Modals
  const {
    OpenMinimumPriceModal,
    OpenCreateRiven,
    OpenSellModal,
    OpenInfoModal,
    OpenFilterModal,
    OpenDeleteModal,
    OpenUpdateMultipleModal,
    OpenDeleteMultipleModal,
    OpenWTSModal,
  } = useStockModals({
    useTranslateBasePrompt,
    createMutation,
    updateMutation,
    updateMultipleMutation,
    sellMutation,
    deleteMutation,
    deleteMultipleMutation,
  });
  useEffect(() => {
    setSelectedRecords([]);
  }, [deleteMultipleMutation.isSuccess, deleteMutation.isSuccess]);
  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshStockRivens, handleRefresh, [refetchQueries]);
  return (
    <Box>
      <Grid>
        <Grid.Col span={8}>
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
        onCreate={() => OpenCreateRiven()}
        rightSectionWidth={30 * 5}
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
                  template: "[<link> <mod_name>]<rank> <price>p",
                  items: filteredRecords,
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
        className={`${classes.databaseStockRivens} ${useHasAlert() ? classes.alert : ""} ${is_running ? classes.running : ""}`}
        customRowAttributes={(record) => {
          return {
            "data-color-mode": "box-shadow",
            "data-stock-status": record.status,
          };
        }}
        mt={"md"}
        striped
        fetching={paginationQuery.isFetching}
        records={paginationQuery.isFetching ? [] : (paginationQuery.data?.results || [])}
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
            case "weapon_name":
              let name = record.weapon_name + " " + record.mod_name;
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
        columns={[
          {
            accessor: "weapon_name",
            title: useTranslateCommon("item_name.title"),
            sortable: true,
            width: "auto",
            render: (row) => <ItemName color="gray.4" size="md" value={row} />,
          },
          {
            accessor: "attributes",
            width: "auto",
            title: useTranslateDataGridColumns("attributes"),
            render: ({ attributes }) => (
              <Tooltip
                withArrow
                openDelay={100}
                closeDelay={100}
                styles={{
                  tooltip: { backgroundColor: "transparent", padding: 0, boxShadow: "none" },
                  arrow: { backgroundColor: "transparent", borderWidth: 0 },
                }}
                label={
                  <Paper withBorder p="xs">
                    {attributes.map((attr, idx) => (
                      <RivenAttribute key={idx} value={attr} compact hideDetails={true} />
                    ))}
                  </Paper>
                }
              >
                <ActionIcon size="sm" variant="outline">
                  <FontAwesomeIcon icon={faInfo} />
                </ActionIcon>
              </Tooltip>
            ),
          },
          {
            accessor: "bought",
            title: useTranslateDataGridColumns("bought"),
            sortable: true,
            render: ({ bought }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={bought} />,
          },
          {
            accessor: "minimum_price",
            sortable: true,
            width: 300,
            title: useTranslateCommon("datatable_columns.minimum_price.title"),
            render: ({ id, minimum_price }) => (
              <ColumnMinMaxPrice
                id={id}
                minimum_price={minimum_price}
                onUpdate={async (id: number, minimum_price: number) => await updateMutation.mutateAsync({ id, minimum_price })}
                onEdit={async (id: number, minimum_price: number) => OpenMinimumPriceModal(id, minimum_price)}
              />
            ),
          },
          {
            accessor: "list_price",
            title: useTranslateCommon("datatable_columns.list_price"),
            sortable: true,
          },
          {
            accessor: "actions",
            title: useTranslateCommon("datatable_columns.actions.title"),
            width: 220,
            render: (row) => (
              <ColumnActions
                row={row}
                buttonProps={{
                  open_filter: {
                    color: row.filter?.enabled ? "blue.7" : "gray.7",
                  },
                }}
                loadingRows={loadingRows}
                onManual={() => OpenSellModal({ ...row, wfm_url: row.wfm_weapon_url, rank: row.sub_type?.rank || 0, price: 0 })}
                onAuto={(price) => sellMutation.mutateAsync({ ...row, wfm_url: row.wfm_weapon_url, rank: row.sub_type?.rank || 0, price })}
                onInfo={() => OpenInfoModal(row)}
                onFilter={() => OpenFilterModal(row)}
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

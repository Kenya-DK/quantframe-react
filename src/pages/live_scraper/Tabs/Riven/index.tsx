import { Box, Grid, Group, NumberFormatter } from "@mantine/core";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useLocalStorage, useMediaQuery } from "@mantine/hooks";
import { useState } from "react";
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
import { TextTranslate } from "@components/Shared/TextTranslate";
import { getSafePage, GetSubTypeDisplay } from "@utils/helper";
import { RivenAttributes } from "@components/DataDisplay/RivenAttributes";
import { useStockMutations } from "./mutations";
import { useStockModals } from "./modals";
import { ColumnMinMaxPrice } from "../../Columns/ColumnMinMaxPrice";
import { ColumnActions } from "../../Columns/ColumnActions";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faDownload } from "@fortawesome/free-solid-svg-icons";
import { HasPermission } from "@api/index";
export type RivenPanelProps = {
  isActive?: boolean;
};

export const RivenPanel = ({ isActive }: RivenPanelProps = {}) => {
  // Responsive
  // Treat as “wide” only when landscape AND ≥800px wide
  const isWide = useMediaQuery("(min-width: 800px) and (orientation: landscape)");
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
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabRiven(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabRiven(`success.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`prompts.${key}`, { ...context }, i18Key);
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabRiven(`prompts.${key}`, { ...context }, i18Key);
  // Queries
  const { paginationQuery, financialReportQuery, statusCountsQuery, refetchQueries } = useStockQueries({ queryData, isActive });
  const handleRefresh = (_data: any) => {
    refetchQueries(true);
  };
  // Mutations
  const { updateStockMutation, createStockMutation, exportMutation, sellStockMutation, deleteStockMutation } = useStockMutations({
    useTranslateSuccess,
    useTranslateErrors,
    refetchQueries,
    setLoadingRows,
  });
  // Modals
  const { OpenMinimumPriceModal, OpenCreateRiven, OpenSellModal, OpenInfoModal, OpenFilterModal, OpenDeleteModal } = useStockModals({
    useTranslateBasePrompt,
    useTranslatePrompt,
    createStockMutation,
    updateStockMutation,
    sellStockMutation,
    deleteStockMutation,
  });
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
        rightSectionWidth={75}
        rightSection={
          <Group>
            <ActionWithTooltip
              tooltip={useTranslate("export_json_tooltip")}
              icon={faDownload}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: !HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA) }}
              onClick={() => exportMutation.mutate(queryData)}
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
        columns={[
          {
            accessor: "weapon_name",
            title: useTranslateCommon("item_name.title"),
            sortable: true,
            render: ({ weapon_name, mod_name, sub_type }) => (
              <TextTranslate
                color="gray.4"
                i18nKey={useTranslateCommon("item_name.value", undefined, true)}
                values={{
                  name: weapon_name + " " + mod_name,
                  sub_type: GetSubTypeDisplay(sub_type),
                }}
              />
            ),
          },
          {
            accessor: "attributes",
            title: useTranslateDataGridColumns("attributes"),
            render: ({ attributes }) => <RivenAttributes tooltip={!isWide} attributes={attributes} />,
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
                onUpdate={async (id: number, minimum_price: number) => await updateStockMutation.mutateAsync({ id, minimum_price })}
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
                onAuto={(price) => sellStockMutation.mutateAsync({ ...row, wfm_url: row.wfm_weapon_url, rank: row.sub_type?.rank || 0, price })}
                onInfo={() => OpenInfoModal(row)}
                onFilter={() => OpenFilterModal(row)}
                onHide={(hide) => updateStockMutation.mutateAsync({ id: row.id, is_hidden: hide })}
                onDelete={() => OpenDeleteModal(row.id)}
              />
            ),
          },
        ]}
      />
    </Box>
  );
};

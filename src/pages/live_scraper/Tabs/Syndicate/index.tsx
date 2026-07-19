import { WFMarketTypes } from "$types";
import { ItemName } from "@components/DataDisplay/ItemName";
import { SearchField } from "@components/Forms/SearchField";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { Box, Grid, Group } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { getSafePage } from "@utils/helper";
import { DataTable } from "mantine-datatable";
import classes from "../../LiveScraper.module.css";
import { useStockQueries } from "./queries";

interface SyndicatePanelProps {
  isActive?: boolean;
}

export const SyndicatePanel = ({ isActive }: SyndicatePanelProps = {}) => {
  // Contexts
  const { is_running } = useLiveScraperContext();
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<WFMarketTypes.WfmOrderControllerGetListParams>({
    key: "syndicate_item_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10, operations: ["Syndicate"] },
  });

  // Translate
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`live_scraper.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.syndicate.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`datatable.columns.${key}`, { ...context }, i18Key);
  // Queries
  const { paginationQuery, refetchQueries } = useStockQueries({ queryData, isActive });

  // Use the custom hook for Tauri events
  // useTauriEvent(TauriTypes.Events.RefreshStockItems, handleRefresh, [refetchQueries]);
  return (
    <Box>
      <Grid>
        <Grid.Col span={8}>
          <Group gap={"md"} mt={"md"}>
            {/* {Object.entries(statusCountsQuery.data || {})
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
              ))} */}
          </Group>
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithSegments
            showPercent
            percentSymbol="%"
            segments={
              [
                // { label: useTranslateSegments("bought"), count: financialReportQuery.data?.expenses || 0, color: "var(--qf-negative-color)" },
                // { label: useTranslateSegments("listed"), count: financialReportQuery.data?.revenue || 0, color: "var(--qf-positive-color)" },
                // { label: useTranslateSegments("profit"), count: financialReportQuery.data?.total_profit || 0, color: "var(--qf-profit)" },
              ]
            }
          />
        </Grid.Col>
      </Grid>
      <SearchField value={queryData.query || ""} onChange={(value) => setQueryData((prev) => ({ ...prev, query: value }))} />
      <DataTable
        className={`${classes.databaseSyndicateItems} ${useHasAlert() ? classes.alert : ""} ${is_running ? classes.running : ""}`}
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
              let name = record.properties?.name || "N/A";
              navigator.clipboard.writeText(name);
              notifications.show({
                title: useTranslateCommon("notifications.copy_to_clipboard.title"),
                message: useTranslateCommon("notifications.copy_to_clipboard.message", { message: name }),
                color: "green.7",
              });
              break;
          }
        }}
        // define columns
        columns={[
          {
            accessor: "item_name",
            title: useTranslateCommon("item_name.title"),
            sortable: true,
            render: (row) => <ItemName color="gray.4" size="md" value={row} />,
          },
          {
            accessor: "syndicate",
            sortable: true,
            title: useTranslateDataGridColumns("syndicate"),
            render: (row) => row.properties?.syndicate || "N/A",
          },
          {
            accessor: "standingCost",
            sortable: true,
            title: useTranslateDataGridColumns("standing_cost"),
            render: (row) => row.properties?.standingCost || "N/A",
          },
          {
            accessor: "platinum",
            sortable: true,
            title: useTranslateCommon("datatable_columns.list_price"),
          },
        ]}
      />
    </Box>
  );
};

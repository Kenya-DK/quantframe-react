import { Box, Checkbox, Group, Paper } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useState } from "react";
import { DataTable } from "mantine-datatable";
import classes from "../../Debug.module.css";
import { SearchField } from "@components/Forms/SearchField";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useQueries } from "./queries";
import { TauriTypes } from "$types";
import { getSafePage } from "@utils/helper";
import { useMutations } from "./mutations";
import { ActionWithTooltip } from "../../../../components/Shared/ActionWithTooltip";
import { faDownload } from "@fortawesome/free-solid-svg-icons";
interface EELogPanelProps {}
export const EELogPanel = ({}: EELogPanelProps) => {
  // Translate general
  const useTranslateTabLogging = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`debug.tabs.ee_log.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabLogging(`datatable.columns.${key}`, { ...context }, i18Key);

  // States For DataGrid
  const [queryData, setQueryData] = useState<TauriTypes.EELogControllerGetListParams>({
    page: 1,
    limit: 10,
    hide_empty: true,
  });

  // Queries
  const { paginationQuery } = useQueries({ queryData });
  // Mutations
  const { exportMutation } = useMutations({ refetchQueries: () => paginationQuery.refetch() });
  return (
    <Box>
      <SearchField
        value={queryData.query || ""}
        onChange={(value) => setQueryData((prev) => ({ ...prev, query: value }))}
        rightSectionWidth={35 * 2}
        rightSection={
          <Group gap={3}>
            <ActionWithTooltip
              tooltip={useTranslateTabLogging("export_json_tooltip")}
              icon={faDownload}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={() => exportMutation.mutate(queryData)}
            />
          </Group>
        }
        filter={
          <Paper p={"sm"} mt={"md"}>
            <Group>
              <Checkbox
                label={useTranslateTabLogging("hide_empty")}
                checked={queryData.hide_empty || false}
                onChange={(e) => setQueryData((prev) => ({ ...prev, hide_empty: e.currentTarget.checked }))}
              />
            </Group>
          </Paper>
        }
      />
      <DataTable
        className={`${classes.dataTableEELog}`}
        data-alert={useHasAlert()}
        striped
        idAccessor={"index"}
        mt={10}
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
          { accessor: "index", title: "#", width: 60 },
          { accessor: "line", title: useTranslateDataGridColumns("line") },
          { accessor: "date", title: useTranslateDataGridColumns("date"), width: 200 },
        ]}
      />
    </Box>
  );
};

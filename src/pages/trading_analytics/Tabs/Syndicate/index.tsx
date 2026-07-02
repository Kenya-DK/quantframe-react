import { QuantframeApiTypes, TauriTypes } from "$types";
import api, { HasPermission } from "@api/index";
import { ItemName } from "@components/DataDisplay/ItemName";
import { MinMax } from "@components/Forms/MinMax";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { Loading } from "@components/Shared/Loading";
import { PatreonOverlay } from "@components/Shared/PatreonOverlay";
import { faDownload } from "@fortawesome/free-solid-svg-icons";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { Box, Grid, Group, MultiSelect, NumberFormatter, Paper, SimpleGrid } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useQuery } from "@tanstack/react-query";
import { getSafePage } from "@utils/helper";
import { DataTable } from "mantine-datatable";
import { useEffect, useState } from "react";
import classes from "../../TradingAnalytics.module.css";
import { useMutations } from "./mutations";
import { useQueries } from "./queries";
interface SyndicatePanelProps {
  isActive?: boolean;
}

export const SyndicatePanel = ({ isActive }: SyndicatePanelProps = {}) => {
  // States For DataGrid
  const queryData = useForm({
    initialValues: { page: 1, limit: 10, query: "" } as QuantframeApiTypes.SyndicateItemPriceControllerGetListParams,
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.syndicate.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`fields.${key}`, { ...context }, i18Key);
  // States
  const [filterOpened, setFilterOpened] = useState<boolean>(false);
  const [canExport, setCanExport] = useState<boolean>(false);
  const [canUse, setCanUse] = useState<boolean>(false);
  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ["cache_syndicates"],
    queryFn: () => api.cache.getSyndicates(),
  });

  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA).then((res) => setCanExport(res));
    HasPermission(TauriTypes.PermissionsFlags.SYNDICATE_PRICES_SEARCH).then((res) => setCanUse(res));
  }, []);
  // Queries
  const { paginationQuery, refetchQueries } = useQueries({ queryData: queryData.values, isActive: canUse && isActive });

  // Mutations
  const { exportMutation } = useMutations({ refetchQueries });

  const IsLoading = () => (paginationQuery.isFetching || exportMutation.isPending) && !(paginationQuery.isSuccess && exportMutation.isSuccess);

  return (
    <Box p={"md"} pos={"relative"}>
      <PatreonOverlay permission={TauriTypes.PermissionsFlags.SYNDICATE_PRICES_SEARCH} tier="T1+" />
      <SearchField
        value={queryData.values.query || ""}
        onSearch={() => {
          queryData.validate();
          if (queryData.isValid()) refetchQueries();
        }}
        searchDisabled={paginationQuery.isLoading}
        onChange={(text) => queryData.setFieldValue("query", text)}
        onFilterToggle={(s) => setFilterOpened(s)}
        rightSectionWidth={35 * 3}
        rightSection={
          <Group gap={3}>
            <ActionWithTooltip
              tooltip={useTranslateTabItem("export_json_tooltip")}
              icon={faDownload}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: !canExport || IsLoading() }}
              onClick={() => exportMutation.mutate(queryData.values)}
            />
          </Group>
        }
        filter={
          <Paper p={"sm"} mt={"md"}>
            <Grid>
              <Grid.Col span={6}>
                <MultiSelect
                  searchable
                  limit={5}
                  label={useTranslateFormFields("syndicate.label")}
                  description={useTranslateFormFields("syndicate.description")}
                  data={data?.filter((item) => item.canSelect).map((item) => ({ label: item.name, value: item.uniqueName })) ?? []}
                  value={queryData.values.syndicates || []}
                  onChange={(value) => queryData.setFieldValue("syndicates", value)}
                  clearable
                />
              </Grid.Col>
              <Grid.Col span={6}>
                <SimpleGrid cols={3} spacing={"sm"}>
                  <MinMax
                    label={useTranslateTabItem("volume_label")}
                    value={{ min: queryData.values.volume_gt, max: queryData.values.volume_lt }}
                    onChange={(value) => {
                      queryData.setFieldValue("volume_gt", value?.min);
                      queryData.setFieldValue("volume_lt", value?.max);
                    }}
                  />
                </SimpleGrid>
              </Grid.Col>
            </Grid>
          </Paper>
        }
      />
      <DataTable
        className={`${classes.databaseItem} ${useHasAlert() ? classes.alert : ""} ${filterOpened ? classes.filterOpened : ""}`}
        mt={"md"}
        striped
        customLoader={<Loading />}
        fetching={IsLoading()}
        records={IsLoading() ? [] : paginationQuery.data?.results || []}
        page={getSafePage(queryData.values.page, paginationQuery.data?.total_pages)}
        onPageChange={(page) => queryData.setFieldValue("page", page)}
        totalRecords={paginationQuery.data?.total || 0}
        recordsPerPage={queryData.values.limit || 10}
        recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
        onRecordsPerPageChange={(limit) => queryData.setFieldValue("limit", limit)}
        idAccessor={(record) => record.wfmId}
        sortStatus={{
          columnAccessor: queryData.values.sort_by || "name",
          direction: queryData.values.sort_direction || "desc",
        }}
        onSortStatusChange={(sort) => {
          if (!sort || !sort.columnAccessor) return;
          queryData.setFieldValue("sort_by", sort.columnAccessor as any);
          queryData.setFieldValue("sort_direction", sort.direction);
        }}
        // define columns
        columns={[
          {
            accessor: "name",
            title: useTranslateCommon("item_name.title"),
            sortable: true,
            width: 250,
            render: (row) => <ItemName color="gray.4" size="md" value={row} />,
          },
          {
            accessor: "syndicate",
            sortable: true,
            title: useTranslateDataGridColumns("syndicate"),
          },
          {
            accessor: "standingCost",
            sortable: true,
            title: useTranslateDataGridColumns("standing_cost"),
            render: ({ standingCost }) => <NumberFormatter decimalScale={2} value={standingCost} />,
          },
          {
            accessor: "volume",
            sortable: true,
            title: useTranslateDataGridColumns("volume"),
            render: ({ volume }) => <NumberFormatter decimalScale={2} value={volume} />,
          },
          {
            accessor: "minPrice",
            sortable: true,
            title: useTranslateDataGridColumns("min_price"),
            render: ({ minPrice }) => <NumberFormatter decimalScale={2} value={minPrice} />,
          },
          {
            accessor: "maxPrice",
            sortable: true,
            title: useTranslateDataGridColumns("max_price"),
            render: ({ maxPrice }) => <NumberFormatter decimalScale={2} value={maxPrice} />,
          },
        ]}
      />
    </Box>
  );
};

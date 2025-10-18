import { Box, Grid, Group, NumberFormatter, Paper, SimpleGrid } from "@mantine/core";
import { useEffect, useState } from "react";
import { QuantframeApiTypes, TauriTypes } from "$types";
import { useQueries } from "./queries";
import { SearchField } from "@components/Forms/SearchField";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import classes from "../../TradingAnalytics.module.css";
import { DataTable } from "mantine-datatable";
import { getSafePage, GetSubTypeDisplay } from "@utils/helper";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { SelectItemTags } from "@components/Forms/SelectItemTags";
import dayjs from "dayjs";
import { useForm } from "@mantine/form";
import { DatePickerInput } from "@mantine/dates";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faDownload } from "@fortawesome/free-solid-svg-icons";
import { HasPermission } from "@api/index";
import { useMutations } from "./mutations";
import { Loading } from "@components/Shared/Loading";
import { MinMax } from "@components/Forms/MinMax";
interface ItemPanelProps {
  isActive?: boolean;
}

export const ItemPanel = ({ isActive }: ItemPanelProps = {}) => {
  // States For DataGrid
  const queryData = useForm({
    initialValues: { page: 1, limit: 10, query: "" } as QuantframeApiTypes.ItemPriceControllerGetListParams,
    validate: {
      to_date: (value: string | undefined) => {
        const fromDate = queryData.values.from_date;
        if (!fromDate) return true;
        const to_date = dayjs(value).format("YYYY-MM-DD");
        if (dayjs(to_date).diff(dayjs(fromDate), "day") > 90) return true;
        if (!dayjs(to_date).isBefore(dayjs().subtract(1, "day"))) return true;
        return false;
      },
    },
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.item.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`datatable.columns.${key}`, { ...context }, i18Key);

  // States
  const [filterOpened, setFilterOpened] = useState<boolean>(false);
  const [dates, setDates] = useState<[string | null, string | null]>([null, null]);
  const [canExport, setCanExport] = useState<boolean>(false);

  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA).then((res) => setCanExport(res));
  }, []);
  // Queries
  const { paginationQuery, refetchQueries } = useQueries({ queryData: queryData.values, isActive });

  // Mutations
  const { exportMutation } = useMutations({
    refetchQueries,
  });
  useEffect(() => {
    if (dates[0] && dates[1] && dates[0] > dates[1]) return;
    if (dates[0]) queryData.setFieldValue("from_date", dates[0]);
    if (dates[1]) queryData.setFieldValue("to_date", dates[1]);
  }, [dates]);

  const IsLoading = () => (paginationQuery.isFetching || exportMutation.isPending) && isActive;

  return (
    <Box p={"md"}>
      <SearchField
        value={queryData.values.query || ""}
        onSearch={() => {
          queryData.validate();
          if (queryData.isValid()) refetchQueries();
        }}
        searchDisabled={paginationQuery.isLoading}
        onChange={(text) => queryData.setFieldValue("query", text)}
        onFilterToggle={(s) => setFilterOpened(s)}
        rightSectionWidth={295}
        rightSection={
          <Group gap={3}>
            <DatePickerInput
              required
              placeholder={useTranslateTabItem("date_range_placeholder")}
              minDate={dayjs(queryData.values.to_date).subtract(90, "day").format("YYYY-MM-DD")}
              maxDate={dayjs().subtract(1, "day").format("YYYY-MM-DD")}
              w={200}
              type="range"
              valueFormat="YYYY MMM DD"
              value={dates}
              onChange={setDates}
              error={queryData.errors.from_date || queryData.errors.to_date}
            />
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
              <Grid.Col span={3}>
                <SelectItemTags value={queryData.values.tags || []} onChange={(value) => queryData.setFieldValue("tags", value)} />
              </Grid.Col>
              <Grid.Col span={9}>
                <SimpleGrid cols={3} spacing={"sm"}>
                  <MinMax
                    label={useTranslateTabItem("volume_label")}
                    value={[queryData.values.volume_gt, queryData.values.volume_lt]}
                    onChange={(value) => {
                      if (!value) return;
                      queryData.setFieldValue("volume_gt", value[0]);
                      queryData.setFieldValue("volume_lt", value[1] || undefined);
                    }}
                  />
                  <MinMax
                    label={useTranslateTabItem("supply_label")}
                    value={[queryData.values.supply_gt, queryData.values.supply_lt]}
                    onChange={(value) => {
                      if (!value) return;
                      queryData.setFieldValue("supply_gt", value[0]);
                      queryData.setFieldValue("supply_lt", value[1] || undefined);
                    }}
                  />
                  <MinMax
                    label={useTranslateTabItem("demand_label")}
                    value={[queryData.values.demand_gt, queryData.values.demand_lt]}
                    onChange={(value) => {
                      if (!value) return;
                      queryData.setFieldValue("demand_gt", value[0]);
                      queryData.setFieldValue("demand_lt", value[1] || undefined);
                    }}
                  />
                  <MinMax
                    label={useTranslateTabItem("min_price_label")}
                    value={[queryData.values.min_price_gt, queryData.values.min_price_lt]}
                    onChange={(value) => {
                      if (!value) return;
                      queryData.setFieldValue("min_price_gt", value[0]);
                      queryData.setFieldValue("min_price_lt", value[1] || undefined);
                    }}
                  />
                  <MinMax
                    label={useTranslateTabItem("max_price_label")}
                    value={[queryData.values.max_price_gt, queryData.values.max_price_lt]}
                    onChange={(value) => {
                      if (!value) return;
                      queryData.setFieldValue("max_price_gt", value[0]);
                      queryData.setFieldValue("max_price_lt", value[1] || undefined);
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
        records={paginationQuery.data?.results || []}
        page={getSafePage(queryData.values.page, paginationQuery.data?.total_pages)}
        onPageChange={(page) => queryData.setFieldValue("page", page)}
        totalRecords={paginationQuery.data?.total || 0}
        recordsPerPage={queryData.values.limit || 10}
        recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
        onRecordsPerPageChange={(limit) => queryData.setFieldValue("limit", limit)}
        idAccessor={(record) => `item-price-${record.wfm_id}${record.datetime}`}
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
            render: ({ name, sub_type }) => (
              <TextTranslate
                color="gray.4"
                i18nKey={useTranslateCommon("item_name.value", undefined, true)}
                values={{
                  name: `${name}`,
                  sub_type: GetSubTypeDisplay(sub_type),
                }}
              />
            ),
          },
          {
            accessor: "volume",
            sortable: true,
            title: useTranslateDataGridColumns("volume"),
            render: ({ volume }) => <NumberFormatter decimalScale={2} value={volume} />,
          },
          {
            accessor: "min_price",
            sortable: true,
            title: useTranslateDataGridColumns("min_price"),
            render: ({ min_price }) => <NumberFormatter decimalScale={2} value={min_price} />,
          },
          {
            accessor: "max_price",
            sortable: true,
            title: useTranslateDataGridColumns("max_price"),
            render: ({ max_price }) => <NumberFormatter decimalScale={2} value={max_price} />,
          },
          {
            accessor: "open_price",
            sortable: true,
            title: useTranslateDataGridColumns("open_price"),
            render: ({ open_price }) => <NumberFormatter decimalScale={2} value={open_price} />,
          },
          {
            accessor: "closed_price",
            sortable: true,
            title: useTranslateDataGridColumns("closed_price"),
            render: ({ closed_price }) => <NumberFormatter decimalScale={2} value={closed_price} />,
          },
          {
            accessor: "avg_price",
            sortable: true,
            title: useTranslateDataGridColumns("avg_price"),
            render: ({ avg_price }) => <NumberFormatter decimalScale={2} value={avg_price} />,
          },
          {
            accessor: "wa_price",
            sortable: true,
            title: useTranslateDataGridColumns("wa_price"),
            render: ({ wa_price }) => <NumberFormatter decimalScale={2} value={wa_price} />,
          },
          {
            accessor: "median",
            sortable: true,
            title: useTranslateDataGridColumns("median"),
            render: ({ median }) => <NumberFormatter decimalScale={2} value={median} />,
          },
          {
            accessor: "moving_avg",
            sortable: true,
            title: useTranslateDataGridColumns("moving_avg"),
            render: ({ moving_avg }) => <NumberFormatter decimalScale={2} value={moving_avg} />,
          },
          {
            accessor: "donch_top",
            sortable: true,
            title: useTranslateDataGridColumns("donch_top"),
            render: ({ donch_top }) => <NumberFormatter decimalScale={2} value={donch_top} />,
          },
          {
            accessor: "donch_bot",
            sortable: true,
            title: useTranslateDataGridColumns("donch_bot"),
            render: ({ donch_bot }) => <NumberFormatter decimalScale={2} value={donch_bot} />,
          },
          {
            accessor: "trading_tax",
            sortable: true,
            title: useTranslateDataGridColumns("trading_tax"),
            render: ({ trading_tax }) => <NumberFormatter decimalScale={2} value={trading_tax} />,
          },
          {
            accessor: "supply",
            sortable: true,
            title: useTranslateDataGridColumns("supply"),
            render: ({ supply }) => <NumberFormatter decimalScale={2} value={supply} />,
          },
          {
            accessor: "demand",
            sortable: true,
            title: useTranslateDataGridColumns("demand"),
            render: ({ demand }) => <NumberFormatter decimalScale={2} value={demand} />,
          },
        ]}
      />
    </Box>
  );
};

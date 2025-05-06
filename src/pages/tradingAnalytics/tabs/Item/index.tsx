import { Box, Group, MultiSelect, Select } from "@mantine/core";
import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { DataTable } from "mantine-datatable";
import { SearchField } from "@components/SearchField";
import { useForm } from "@mantine/form";
import { QuantframeApiTypes, WFMarketTypes } from "$types";

import dayjs from "dayjs";
import { DatePickerInput } from "@mantine/dates";
import { useEffect, useState } from "react";
import { upperFirst } from "@mantine/hooks";
import { PremiumOverlay } from "../../../../components/PremiumOverlay";

interface ItemPanelProps {}

export const ItemPanel = ({}: ItemPanelProps) => {
  // States For DataGrid
  const dataGridState = useForm({
    initialValues: { page: 1, limit: 10, query: "" } as QuantframeApiTypes.ItemControllerGetListParams,
    validate: {
      to_date: (value) => {
        const fromDate = dataGridState.values.from_date;
        if (!fromDate) return null;
        const to_date = dayjs(value).format("YYYY-MM-DD");
        if (dayjs(to_date).diff(dayjs(fromDate), "day") > 90) return "Date range cannot be more than 90 days";
        if (!dayjs(to_date).isBefore(dayjs().subtract(1, "day"))) return "Date cannot be in the future";
        return null;
      },
    },
    onValuesChange: () => {
      dataGridState.validate();
    },
  });
  const [tags, setTags] = useState<{ label: string; value: string }[]>([]);
  const [dates, setDates] = useState<[string | null, string | null]>([null, null]);
  const [filterOpened, setFilterOpened] = useState(false);

  useEffect(() => {
    const go = async () => {
      const items = await api.cache.getTradableItems();
      const a = items.map((item) => item.tags).flat();
      const uniqueTags = Array.from(new Set(a)).map((tag) => ({ label: upperFirst(tag.replace("_", " ")), value: tag }));
      setTags(uniqueTags);
    };
    go();
  }, []);

  useEffect(() => {
    if (dates[0] && dates[1] && dates[0] > dates[1]) return;
    if (dates[0]) dataGridState.setFieldValue("from_date", dates[0]);
    if (dates[1]) dataGridState.setFieldValue("to_date", dates[1]);
  }, [dates]);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabOverview = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.item.${key}`, { ...context }, i18Key);
  const useTranslateDataTable = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOverview(`datatable.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOverview(`fields.${key}`, { ...context }, i18Key);
  const useTranslateOrderStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`order_type.${key}`, { ...context }, i18Key);

  // Queys
  const { data, isFetching, refetch } = useQuery({
    queryKey: ["item_prices", dataGridState.values.sort_by, dataGridState.values.sort_direction],
    queryFn: () => api.items.getAll(dataGridState.values),
    refetchOnWindowFocus: false,
    retry: false,
    enabled: dataGridState.values.from_date != undefined && dataGridState.values.to_date != undefined,
  });

  return (
    <Box p={"md"} style={{ position: "relative" }}>
      <PremiumOverlay tier="free" />
      <SearchField
        value={dataGridState.values.query || ""}
        onSearch={() => refetch()}
        onChange={(text) => dataGridState.setFieldValue("query", text)}
        onFilterToggle={(op) => setFilterOpened(op)}
        filter={
          <Box mb={"md"}>
            <Group>
              <Select
                label={useTranslateFields("order_type.label")}
                description={useTranslateFields("order_type.description")}
                data={Object.values(WFMarketTypes.OrderType).map((mode) => ({
                  value: mode,
                  label: useTranslateOrderStatus(mode),
                }))}
                value={dataGridState.values.order_type as WFMarketTypes.OrderType}
                onChange={(value) => dataGridState.setFieldValue("order_type", value as WFMarketTypes.OrderType)}
              />
              <MultiSelect
                searchable
                limit={5}
                label={useTranslateFields("tags.label")}
                description={useTranslateFields("tags.description")}
                data={tags}
                value={dataGridState.values.tags}
                onChange={(value: string[]) => dataGridState.setFieldValue("tags", value)}
                clearable
              />
              <DatePickerInput
                required
                minDate={dayjs().subtract(90, "day").format("YYYY-MM-DD")}
                maxDate={dayjs().subtract(1, "day").format("YYYY-MM-DD")}
                w={200}
                type="range"
                valueFormat="YYYY MMM DD"
                label={useTranslateFields("date_range.label")}
                description={useTranslateFields("date_range.description")}
                value={dates}
                onChange={setDates}
                error={dataGridState.errors.from_date || dataGridState.errors.to_date}
              />
            </Group>
          </Box>
        }
      />
      <DataTable
        mt={"md"}
        height={`calc(100vh - ${filterOpened ? 306 : 225}px)`}
        fetching={isFetching}
        records={data?.results || []}
        page={dataGridState.values.page}
        onPageChange={(page) => dataGridState.setFieldValue("page", page)}
        totalRecords={data?.total}
        recordsPerPage={dataGridState.values.limit}
        recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
        onRecordsPerPageChange={(limit) => dataGridState.setFieldValue("limit", limit)}
        sortStatus={{
          columnAccessor: dataGridState.values.sort_by || "created_at",
          direction: dataGridState.values.sort_direction || "desc",
        }}
        onSortStatusChange={(sort) => {
          if (!sort || !sort.columnAccessor) return;
          dataGridState.setFieldValue("sort_by", sort.columnAccessor as any);
          dataGridState.setFieldValue("sort_direction", sort.direction);
        }}
        // define columns
        columns={[
          {
            accessor: "name",
            title: useTranslateDataTable("columns.name"),
            sortable: true,
          },
          {
            accessor: "order_type",
            title: useTranslateDataTable("columns.order_type"),
            sortable: true,
          },
          {
            accessor: "volume",
            title: useTranslateDataTable("columns.volume"),
            sortable: true,
          },
          {
            accessor: "min_price",
            title: useTranslateDataTable("columns.min_price"),
            sortable: true,
          },
          {
            accessor: "max_price",
            title: useTranslateDataTable("columns.max_price"),
            sortable: true,
          },
          {
            accessor: "avg_price",
            title: useTranslateDataTable("columns.avg_price"),
            sortable: true,
          },
          {
            accessor: "moving_avg",
            title: useTranslateDataTable("columns.moving_avg"),
            sortable: true,
          },
        ]}
      />
    </Box>
  );
};

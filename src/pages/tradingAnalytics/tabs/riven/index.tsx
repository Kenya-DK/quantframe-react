import { Box, Center, NumberFormatter } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { DataTable } from "mantine-datatable";
import { SearchField } from "@components/SearchField";
import { useForm } from "@mantine/form";
import { QuantframeApiTypes } from "$types";
import { Loading } from "@components/Loading";
import { AlertError } from "@components/AlertError";
import dayjs from "dayjs";
import { DatePickerInput } from "@mantine/dates";
import { useEffect, useState } from "react";
import { PremiumOverlay } from "@components/PremiumOverlay";
import { PermissionsFlags } from "@utils/permissions";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../TradingAnalytics.module.css";

interface RivenPanelProps {}

export const RivenPanel = ({}: RivenPanelProps) => {
  // States For DataGrid
  const dataGridState = useForm({
    initialValues: { page: 1, limit: 10, query: "" } as QuantframeApiTypes.RivenControllerGetRivenListParams,
    validate: {
      to_date: (value: string) => {
        const fromDate = dataGridState.values.from_date;
        if (!fromDate) return true;
        const to_date = dayjs(value).format("YYYY-MM-DD");
        if (dayjs(to_date).diff(dayjs(fromDate), "day") > 90) return true;
        if (!dayjs(to_date).isBefore(dayjs().subtract(1, "day"))) return true;
        return false;
      },
    },
  });
  const [dates, setDates] = useState<[string | null, string | null]>([null, null]);
  const [filterOpened, setFilterOpened] = useState(false);

  useEffect(() => {
    if (dates[0] && dates[1] && dates[0] > dates[1]) return;
    if (dates[0]) dataGridState.setFieldValue("from_date", dates[0]);
    if (dates[1]) dataGridState.setFieldValue("to_date", dates[1]);
  }, [dates]);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabOverview = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.riven.${key}`, { ...context }, i18Key);
  const useTranslateDataTable = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOverview(`datatable.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOverview(`fields.${key}`, { ...context }, i18Key);

  // Queys
  const { data, isFetching, refetch, error } = useQuery({
    queryKey: ["riven_prices", dataGridState.values.sort_by, dataGridState.values.sort_direction],
    queryFn: () => api.rivens.getAll(dataGridState.values),
    refetchOnWindowFocus: false,
    retry: false,
    enabled: dataGridState.values.from_date != undefined && dataGridState.values.to_date != undefined,
  });

  return (
    <Box p={"md"} style={{ position: "relative" }}>
      <PremiumOverlay tier="T3+" permission={PermissionsFlags.RIVEN_PRICES_SEARCH} />
      <SearchField
        value={dataGridState.values.query || ""}
        onSearch={() => {
          dataGridState.validate();
          if (dataGridState.isValid()) refetch();
        }}
        searchDisabled={isFetching}
        onChange={(text) => dataGridState.setFieldValue("query", text)}
        onFilterToggle={(op) => setFilterOpened(op)}
        rightSectionWidth={255}
        rightSection={
          <DatePickerInput
            required
            placeholder={useTranslateFields("date_range.placeholder")}
            minDate={dayjs().subtract(90, "day").format("YYYY-MM-DD")}
            maxDate={dayjs().subtract(1, "day").format("YYYY-MM-DD")}
            w={200}
            type="range"
            valueFormat="YYYY MMM DD"
            value={dates}
            onChange={setDates}
            error={dataGridState.errors.from_date || dataGridState.errors.to_date}
          />
        }
      />
      <DataTable
        mt={"md"}
        className={`${classes.databaseRivens} ${useHasAlert() ? classes.alert : ""} ${filterOpened ? classes.filterOpened : ""}`}
        fetching={isFetching || !!error}
        customLoader={
          <Box style={{ width: "100%", height: "100%" }} p={"md"}>
            {isFetching ? (
              <Loading />
            ) : (
              <Center style={{ width: "100%", height: "100%" }}>
                <AlertError error={error as any} />
              </Center>
            )}
          </Box>
        }
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
          },
          {
            accessor: "volume",
            title: useTranslateDataTable("columns.volume"),
            sortable: true,
            render: (item) => <NumberFormatter thousandSeparator decimalScale={2} value={item.volume} />,
          },
          {
            accessor: "min_price",
            title: useTranslateDataTable("columns.min_price"),
            sortable: true,
            render: (item) => <NumberFormatter thousandSeparator decimalScale={2} value={item.min_price} />,
          },
          {
            accessor: "max_price",
            title: useTranslateDataTable("columns.max_price"),
            sortable: true,
            render: (item) => <NumberFormatter thousandSeparator decimalScale={2} value={item.max_price} />,
          },
          {
            accessor: "avg_price",
            title: useTranslateDataTable("columns.avg_price"),
            sortable: true,
            render: (item) => <NumberFormatter thousandSeparator decimalScale={2} value={item.avg_price} />,
          },
          {
            accessor: "avg_re_rolls",
            title: useTranslateDataTable("columns.avg_re_rolls"),
            sortable: true,
            render: (item) => <NumberFormatter thousandSeparator decimalScale={2} value={item.avg_re_rolls || 0} />,
          },
        ]}
      />
    </Box>
  );
};

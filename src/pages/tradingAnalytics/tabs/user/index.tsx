import { Box, Grid, Group, Paper, Select, useMantineTheme } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { DatePickerInput } from "@mantine/dates";
import { useEffect, useState } from "react";
import { PremiumOverlay } from "@components/PremiumOverlay";
import { PermissionsFlags } from "@utils/permissions";
import api from "@api/index";
import { QuantframeApiTypes } from "$types";
import { useForm } from "@mantine/form";
import { useQuery } from "@tanstack/react-query";
import { Bar } from "react-chartjs-2";

interface UserPanelProps {}

export const UserPanel = ({}: UserPanelProps) => {
  const theme = useMantineTheme();
  // States For DataGrid
  const dataGridState = useForm({
    initialValues: { group_by: "day" } as QuantframeApiTypes.WfmControllerGetUserActiveHistoryParams,
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

  useEffect(() => {
    if (dates[0] && dates[1] && dates[0] > dates[1]) return;
    if (dates[0]) dataGridState.setFieldValue("from_date", dates[0]);
    if (dates[1]) dataGridState.setFieldValue("to_date", dates[1]);
  }, [dates]);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabOverview = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.user.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOverview(`fields.${key}`, { ...context }, i18Key);

  // Queys
  const { data } = useQuery({
    queryKey: ["riven_prices", dataGridState.values],
    queryFn: () => api.user.getAll(dataGridState.values),
    refetchOnWindowFocus: false,
    retry: false,
    enabled: dataGridState.values.from_date != undefined && dataGridState.values.to_date != undefined,
  });

  return (
    <Box p={"md"} h={"50vh"} style={{ position: "relative" }}>
      <Grid>
        <Grid.Col span={9} style={{ position: "relative", height: "60vh" }}>
          <Bar
            style={{
              border: `1px solid ${theme.colors.gray[3]}`,
              borderRadius: 8,
              color: theme.colors.gray[7],
            }}
            options={{
              color: theme.colors.gray[7],
              // Elements options apply to all of the options unless overridden in a dataset
              // In this case, we are setting the border of each horizontal bar to be 2px wide
              elements: {
                bar: {
                  borderWidth: 1,
                },
              },
              scales: {
                x: {
                  grid: {
                    display: true,
                    drawOnChartArea: true,
                    drawTicks: false,
                    color: "rgba(255, 255, 255, .2)",
                  },
                  ticks: {
                    color: "#ffffffff", // red X-axis labels
                    font: { size: 14, weight: "bold" },
                  },
                  title: {
                    display: true,
                    color: "#ffffffff", // green title
                    font: { size: 16, weight: "bold" },
                  },
                },
                y: {
                  grid: {
                    display: true,
                    drawOnChartArea: true,
                    drawTicks: false,
                    color: "rgba(255, 255, 255, .2)",
                  },
                  ticks: {
                    color: "#ffffffff", // blue Y-axis labels
                    font: { size: 14, weight: "bold" },
                  },
                },
              },
              responsive: true,
              plugins: {
                legend: {
                  labels: {
                    color: "#ffffffff", // orange legend text
                    font: { size: 20, weight: "bold" },
                  },
                },
              },
            }}
            data={{
              labels: data?.labels || [],
              datasets: [
                {
                  label: useTranslateTabOverview("chart.datasets.registered_users"),
                  data: data?.registered_users_chart || [],
                  backgroundColor: "rgb(0, 158, 33)",
                },
                {
                  label: useTranslateTabOverview("chart.datasets.active_users"),
                  data: data?.total_users_chart || [],
                  backgroundColor: "rgb(242, 168, 60)",
                },
              ],
            }}
          />
        </Grid.Col>
        <Grid.Col span={3}>
          <Paper p={"md"} withBorder>
            <Group>
              <Select
                allowDeselect={false}
                label={useTranslateFields("group_by.label")}
                data={[
                  { value: "day", label: useTranslateFields("group_by.options.day") },
                  { value: "hour", label: useTranslateFields("group_by.options.hour") },
                ]}
                value={dataGridState.values.group_by}
                onChange={(value) => dataGridState.setFieldValue("group_by", value as any)}
                error={dataGridState.errors.group_by}
              />
              <DatePickerInput
                required
                label={useTranslateFields("date_range.label")}
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
            </Group>
          </Paper>
        </Grid.Col>
      </Grid>
      <PremiumOverlay tier="T3+" permission={PermissionsFlags.WFM_USER_ACTIVE_HISTORY} />
    </Box>
  );
};

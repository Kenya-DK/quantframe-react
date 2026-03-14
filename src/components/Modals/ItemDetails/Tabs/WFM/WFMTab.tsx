import { Box, Button, Grid, Group, Text, Title } from "@mantine/core";
import { TauriTypes, WFMarketTypes } from "$types";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { open } from "@tauri-apps/plugin-shell";
export type WFMTabProps = {
  value: TauriTypes.StockItem<{
    orders?: WFMarketTypes.Order[];
    buy_highest_price?: number;
    buy_lowest_price?: number;
    demand?: number;
    roi_percent?: number | null;
    sell_highest_price?: number;
    sell_lowest_price?: number;
    spread?: number;
    spread_percent?: number;
    supply?: number;
    [key: string]: any;
  }>;
};

export function WFMTab({ value }: WFMTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`item_details.tabs.wfm.${key}`, { ...context }, i18Key);

  const stats = value?.properties;

  const segments = [
    {
      group: "1",
      label: useTranslateTab("labels.buy_lowest"),
      count: stats?.buy_lowest_price ?? 0,
      color: "var(--qf-negative-color)",
      tooltip: useTranslateTab("tooltips.buy_highest_price"),
      part: stats?.buy_highest_price ?? 0,
      decimalScale: 2,
    },
    {
      group: "1",
      label: useTranslateTab("labels.sell_lowest"),
      count: stats?.sell_lowest_price ?? 0,
      color: "var(--qf-positive-color)",
      tooltip: useTranslateTab("tooltips.sell_highest_price"),
      part: stats?.sell_highest_price ?? 0,
      decimalScale: 2,
    },
    {
      group: "1",
      label: useTranslateTab("labels.spread"),
      color: "var(--qf-profit)",
      count: stats?.spread ?? 0,
      part: stats?.spread_percent ?? 0,
      decimalScale: 2,
      suffix: "%",
      tooltip: useTranslateTab("tooltips.spread_percent"),
    },
    {
      group: "1",
      label: useTranslateTab("labels.demand"),
      color: "orange",
      count: stats?.demand ?? 0,
      part: stats?.supply ?? 0,
      tooltip: useTranslateTab("tooltips.supply"),
    },
    {
      group: "1",
      label: useTranslateTab("labels.roi_percent"),
      color: "var(--qf-positive-color)",
      count: Math.floor(stats?.roi_percent ?? 0), // Ensure ROI is not negative for the bar representation
      suffix: "%",
      decimalScale: 2,
    },
  ];
  const GetProperty = (key: string) => {
    return value[key as keyof typeof value] ?? value.properties?.[key];
  };
  return (
    <Box>
      <Grid>
        <Grid.Col span={10}>
          <StatsWithSegments
            orientation="vertical"
            p={0}
            hidePercentBar
            showPercent
            segments={segments.filter((segment) => segment.group === "1")}
            footer={null}
          />
        </Grid.Col>
        <Grid.Col span={2}>
          <Title order={3}>Live Orders</Title>
          {value?.properties.orders?.map((order, index) => (
            <Group
              key={index}
              mb="xs"
              style={{ border: "1px solid var(--mantine-color-dark-3)", borderRadius: "4px" }}
              p={3}
              justify="space-between"
              align="center"
            >
              <Text>{order.user?.ingame_name}</Text>
              <Text>{order.platinum}</Text>
            </Group>
          ))}
          <Button fullWidth variant="outline" color="blue" onClick={() => open(`https://warframe.market/items/${GetProperty("wfm_url")}`)}>
            {useTranslateTab("buttons.view_on_wfm")}
          </Button>
        </Grid.Col>
      </Grid>
    </Box>
  );
}

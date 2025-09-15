import { Center, Divider, Grid, Group, TextInput, Title } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateEnums, useTranslateModals } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { PriceHistoryListItem } from "@components/DataDisplay/PriceHistoryListItem";

export type OverviewTabProps = {
  value: TauriTypes.StockRivenDetails | undefined;
};

export function OverviewTab({ value }: OverviewTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`stock_riven_details.tabs.overview.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`fields.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  if (!value) return <></>;
  return (
    <Grid>
      <Grid.Col span={6}>
        <Group grow>
          <TextInput label={useTranslateFields("created_at")} value={dayjs(value.stock.created_at).format("DD/MM/YYYY HH:mm:ss")} readOnly />
          <TextInput label={useTranslateFields("updated_at")} value={dayjs(value.stock.updated_at).format("DD/MM/YYYY HH:mm:ss")} readOnly />
        </Group>
        <Group grow>
          <TextInput
            label={useTranslateFields("status")}
            data-stock-status={value.stock.status}
            data-color-mode="text"
            value={useTranslateStockStatus(value.stock.status)}
            readOnly
          />
          <TextInput label={useTranslateFields("minimum_price")} value={value.stock.minimum_price || "N/A"} readOnly />
        </Group>
        <Group grow>
          <TextInput label={useTranslateFields("bought")} value={value.stock.bought} readOnly />
          <TextInput label={useTranslateFields("list_price")} value={value.stock.list_price || "N/A"} readOnly />
        </Group>
        <Divider mt={"md"} />
        <Group grow>
          <TextInput label={useTranslateFields("profit")} value={value.stock_profit || "N/A"} readOnly />
          {/* <TextInput label={useTranslateFields("highest_price")} value={value.order_info?.properties?.highest_price || "N/A"} readOnly />
          <TextInput label={useTranslateFields("lowest_price")} value={value.order_info?.properties?.lowest_price || "N/A"} readOnly /> */}
        </Group>
      </Grid.Col>
      <Grid.Col span={6}>
        <Title order={3}>{useTranslateFields("listed")}</Title>
        {value.stock.price_history.length <= 0 && (
          <Center h={"100%"}>
            <Title order={3}>{useTranslateFields("no_listed")}</Title>
          </Center>
        )}
        {value.stock.price_history.length > 0 &&
          value.stock.price_history
            .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
            .slice(0, 5)
            .map((price, index) => <PriceHistoryListItem key={index} history={price} />)}
      </Grid.Col>
    </Grid>
  );
}

import { Button, Center, Divider, Grid, Group, TextInput, Title } from "@mantine/core";
import { PriceHistory, SubType, TauriTypes, WFMarketTypes } from "$types";
import { useTranslateEnums, useTranslateModals } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { ItemComponents } from "@components/DataDisplay/ItemComponents";
import { PriceHistoryListItem } from "@components/DataDisplay/PriceHistoryListItem";

interface ItemProperties {
  closed_avg: number;
  highest_price: number;
  wfm_url: string;
  item_id: string;
  name: string;
  image: string;
  lowest_price: number;
  operations: string[];
  order_id: string;
  orders: WFMarketTypes.Order<any>[];
  profit: number;
  quantity: number;
  sub_type: SubType;
  t_type?: TauriTypes.CacheTradableItemSubType;
  price_history: PriceHistory[];
  components?: TauriTypes.ItemComponent[];
}

export type OverviewTabProps = {
  value: WFMarketTypes.Order<ItemProperties> | undefined;
};

export function OverviewTab({ value }: OverviewTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`wfm_order_details.tabs.overview.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`order_type.${key}`, { ...context }, i18Key);
  if (!value) return <></>;
  const { properties } = value;
  return (
    <Grid>
      <Grid.Col span={6}>
        <Group grow>
          <TextInput label={useTranslateFields("created_at")} value={dayjs(value?.createdAt).format("DD/MM/YYYY HH:mm:ss")} readOnly />
          <TextInput label={useTranslateFields("updated_at")} value={dayjs(value?.updatedAt).format("DD/MM/YYYY HH:mm:ss")} readOnly />
        </Group>
        <Group grow>
          <TextInput
            label={useTranslateFields("order_type")}
            data-order-type={value?.type}
            data-color-mode="text"
            value={useTranslateStockStatus(value?.type || "")}
            readOnly
          />
        </Group>
        <Group grow>
          <TextInput label={useTranslateFields("platinum")} value={value?.platinum || "N/A"} readOnly />
          <TextInput label={useTranslateFields("quantity")} value={value?.quantity || "N/A"} readOnly />
          <TextInput label={useTranslateFields("per_trade")} value={value?.perTrade || "N/A"} readOnly />
        </Group>
        <Divider mt={"md"} />
        <Group grow>
          <TextInput label={useTranslateFields("closed_avg")} value={properties?.closed_avg || "N/A"} readOnly />
          <TextInput label={useTranslateFields("profit")} value={properties?.profit || "N/A"} readOnly />
          <TextInput label={useTranslateFields("highest_price")} value={properties?.highest_price || "N/A"} readOnly />
          <TextInput label={useTranslateFields("lowest_price")} value={properties?.lowest_price || "N/A"} readOnly />
        </Group>
        {properties?.components && properties.components.length > 0 && (
          <>
            <Divider mt={"md"} />
            <ItemComponents components={properties.components} />
          </>
        )}
        <Divider mt={"md"} />
        <Button
          mt={"md"}
          color="blue"
          variant="outline"
          onClick={() => {
            open(`https://warframe.market/items/${properties?.wfm_url}`);
          }}
        >
          {useTranslateButtons("wfm")}
        </Button>
      </Grid.Col>
      <Grid.Col span={6}>
        <Title order={3}>{useTranslateFields("listed")}</Title>
        {properties && properties?.price_history && properties?.price_history?.length <= 0 && (
          <Center h={"90%"}>
            <Title order={3}>{useTranslateFields("no_listed")}</Title>
          </Center>
        )}
        {properties &&
          properties?.price_history &&
          properties?.price_history.length > 0 &&
          properties?.price_history
            .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
            .slice(0, 5)
            .map((price, index) => <PriceHistoryListItem key={index} history={price} />)}
      </Grid.Col>
    </Grid>
  );
}

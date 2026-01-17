import { Button, Center, Divider, Grid, Group, TextInput, Title } from "@mantine/core";
import { WFMarketTypes } from "$types";
import { useTranslateEnums, useTranslateModals } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { ItemComponents } from "@components/DataDisplay/ItemComponents";
import { PriceHistoryListItem } from "@components/DataDisplay/PriceHistoryListItem";

export type OverviewTabProps = {
  value: WFMarketTypes.OrderDetails | undefined;
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
  const { order_info, item_info } = value;
  return (
    <Grid>
      <Grid.Col span={6}>
        <Group grow>
          <TextInput label={useTranslateFields("created_at")} value={dayjs(order_info?.createdAt).format("DD/MM/YYYY HH:mm:ss")} readOnly />
          <TextInput label={useTranslateFields("updated_at")} value={dayjs(order_info?.updatedAt).format("DD/MM/YYYY HH:mm:ss")} readOnly />
        </Group>
        <Group grow>
          <TextInput
            label={useTranslateFields("order_type")}
            data-order-type={order_info?.type}
            data-color-mode="text"
            value={useTranslateStockStatus(order_info?.type || "")}
            readOnly
          />
          <TextInput label={useTranslateFields("operations")} value={order_info?.properties?.operations.join(", ") || "N/A"} readOnly />
        </Group>
        <Group grow>
          <TextInput label={useTranslateFields("platinum")} value={order_info?.platinum || "N/A"} readOnly />
          <TextInput label={useTranslateFields("quantity")} value={order_info?.quantity || "N/A"} readOnly />
          <TextInput label={useTranslateFields("per_trade")} value={order_info?.perTrade || "N/A"} readOnly />
        </Group>
        <Divider mt={"md"} />
        <Group grow>
          <TextInput label={useTranslateFields("closed_avg")} value={order_info?.properties?.closed_avg || "N/A"} readOnly />
          <TextInput label={useTranslateFields("profit")} value={order_info?.properties?.profit || "N/A"} readOnly />
          <TextInput label={useTranslateFields("highest_price")} value={order_info?.properties?.highest_price || "N/A"} readOnly />
          <TextInput label={useTranslateFields("lowest_price")} value={order_info?.properties?.lowest_price || "N/A"} readOnly />
        </Group>
        {item_info?.components && item_info.components.length > 0 && (
          <>
            <Divider mt={"md"} />
            <ItemComponents components={item_info.components} />
          </>
        )}
        <Divider mt={"md"} />
        <Button
          mt={"md"}
          color="blue"
          variant="outline"
          onClick={() => {
            open(`https://warframe.market/items/${item_info?.wfm_url_name}`);
          }}
        >
          {useTranslateButtons("wfm")}
        </Button>
      </Grid.Col>
      <Grid.Col span={6}>
        <Title order={3}>{useTranslateFields("listed")}</Title>
        {order_info?.properties && order_info?.properties?.price_history.length <= 0 && (
          <Center h={"90%"}>
            <Title order={3}>{useTranslateFields("no_listed")}</Title>
          </Center>
        )}
        {order_info?.properties &&
          order_info?.properties?.price_history.length > 0 &&
          order_info?.properties?.price_history
            .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
            .slice(0, 5)
            .map((price, index) => <PriceHistoryListItem key={index} history={price} />)}
      </Grid.Col>
    </Grid>
  );
}

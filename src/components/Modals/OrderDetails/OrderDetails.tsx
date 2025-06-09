import { Grid, Title, Tabs, Center, ScrollArea, Group, TextInput, Button } from "@mantine/core";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { PriceHistoryListItem } from "@components/PriceHistory";
import { WFMarketTypes } from "$types/index";
import { OrderListItem } from "@components/OrderListItem";
import dayjs from "dayjs";
import api from "@api/index";
import { useEffect, useState } from "react";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
import { open } from "@tauri-apps/plugin-shell";

export type OrderDetailsProps = {
  value: WFMarketTypes.OrderDto;
};
export function OrderDetails({ value }: OrderDetailsProps) {
  //State
  const [item, setItem] = useState<TauriTypes.CacheTradableItem | undefined>(undefined);

  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ["cache_items"],
    queryFn: () => api.cache.getTradableItems(),
  });

  useEffect(() => {
    if (!data) {
      setItem(undefined);
      return;
    }
    const item = data.find((x) => x.wfm_url_name === value.item?.url_name);
    setItem(item);
  }, [data, value]);
  // Translate general
  const useTranslateStockRivenInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`order_details.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockRivenInfo(`tabs.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockRivenInfo(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockRivenInfo(`buttons.${key}`, { ...context }, i18Key);

  return (
    <Tabs defaultValue="general" h={"75vh"}>
      <Tabs.List>
        <Tabs.Tab value="general">{useTranslateTabs("general.title")}</Tabs.Tab>
        <Tabs.Tab value="orders">{useTranslateTabs("orders.title")}</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="general">
        <Grid>
          <Grid.Col span={6}>
            <Group grow>
              <TextInput label={useTranslateFields("created_at")} value={dayjs(value.creation_date).format("DD/MM/YYYY HH:mm:ss")} readOnly />
              <TextInput label={useTranslateFields("updated_at")} value={dayjs(value.last_update).format("DD/MM/YYYY HH:mm:ss")} readOnly />
            </Group>
            <Group grow>
              <TextInput label={useTranslateFields("list_price")} value={value.platinum || "N/A"} readOnly />
              <TextInput label={useTranslateFields("profit")} value={value.info.profit || "N/A"} readOnly />
            </Group>
            <Group grow>
              <TextInput label={useTranslateFields("total_buyers")} value={value.info?.total_buyers || "N/A"} readOnly />
              <TextInput label={useTranslateFields("highest_price")} value={value.info?.highest_price || "N/A"} readOnly />
              <TextInput label={useTranslateFields("lowest_price")} value={value.info?.lowest_price || "N/A"} readOnly />
            </Group>
            <Group mt={"md"} grow>
              <Button
                color="blue"
                variant="outline"
                onClick={() => {
                  open(`https://warframe.market/items/${value.item?.url_name}`);
                }}
              >
                {useTranslateButtons("wfm")}
              </Button>
              {item && (
                <Button
                  color="blue"
                  variant="outline"
                  onClick={() => {
                    open(item.wiki_url);
                  }}
                >
                  {useTranslateButtons("wiki")}
                </Button>
              )}
            </Group>
          </Grid.Col>
          <Grid.Col span={6}>
            <Title order={3}>{useTranslateFields("listed")}</Title>
            {(value.info?.price_history?.length || 0) <= 0 && (
              <Center h={"100%"}>
                <Title order={3}>{useTranslateFields("no_listed")}</Title>
              </Center>
            )}
            {(value.info?.price_history?.length || 0) > 0 &&
              value.info?.price_history
                .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
                .slice(0, 5)
                .map((price, index) => <PriceHistoryListItem key={index} history={price} />)}
          </Grid.Col>
        </Grid>
      </Tabs.Panel>

      <Tabs.Panel value="orders">
        <ScrollArea h={"70.5vh"}>
          {!value.info?.orders?.length && (
            <Center h={"100%"}>
              <Title order={3}>{useTranslateFields("no_orders")}</Title>
            </Center>
          )}
          {(value.info?.orders?.length || 0) > 0 &&
            value.info?.orders?.slice(0, 5).map((auction, index) => <OrderListItem key={index} order={auction} />)}
        </ScrollArea>
      </Tabs.Panel>
    </Tabs>
  );
}

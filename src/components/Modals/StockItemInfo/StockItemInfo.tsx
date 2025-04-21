import { Group, TextInput, Button, Grid, Title, Tabs, Center, ScrollArea } from "@mantine/core";
import { CacheTradableItem, StockItem } from "@api/types";
import { useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect, useState } from "react";
import { PriceHistoryListItem } from "@components/PriceHistory";
import { OrderItem } from "@components/OrderItem";
import { open } from "@tauri-apps/plugin-shell";
export type StockItemInfoProps = {
  value: StockItem;
};
export function StockItemInfo({ value }: StockItemInfoProps) {
  //State
  const [item, setItem] = useState<CacheTradableItem | undefined>(undefined);

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
    const item = data.find((x) => x.wfm_url_name === value.wfm_url);
    setItem(item);
  }, [data, value]);

  // Translate general
  const useTranslateStockItemInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`stock_item_info.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockItemInfo(`fields.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockItemInfo(`tabs.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockItemInfo(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
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
              <TextInput label={useTranslateFields("created_at")} value={dayjs(value.created_at).format("DD/MM/YYYY HH:mm:ss")} readOnly />
              <TextInput label={useTranslateFields("updated_at")} value={dayjs(value.updated_at).format("DD/MM/YYYY HH:mm:ss")} readOnly />
            </Group>
            <Group grow>
              <TextInput
                label={useTranslateFields("status")}
                data-stock-status={value.status}
                data-color-mode="text"
                value={useTranslateStockStatus(value.status)}
                readOnly
              />
              <TextInput label={useTranslateFields("minimum_price")} value={value.minimum_price || "N/A"} readOnly />
              <TextInput label={useTranslateFields("owned")} value={value.owned || "N/A"} readOnly />
            </Group>
            <Group grow>
              <TextInput label={useTranslateFields("bought")} value={value.bought} readOnly />
              <TextInput label={useTranslateFields("list_price")} value={value.list_price || "N/A"} readOnly />
              <TextInput label={useTranslateFields("moving_avg")} value={value.info?.moving_avg || "N/A"} readOnly />
              <TextInput label={useTranslateFields("profit")} value={value.info?.profit || "N/A"} readOnly />
            </Group>
            <Group grow>
              <TextInput label={useTranslateFields("total_sellers")} value={value.info?.total_sellers || "N/A"} readOnly />
              <TextInput label={useTranslateFields("highest_price")} value={value.info?.highest_price || "N/A"} readOnly />
              <TextInput label={useTranslateFields("lowest_price")} value={value.info?.lowest_price || "N/A"} readOnly />
            </Group>
            <Group mt={"md"} grow>
              <Button
                color="blue"
                variant="outline"
                onClick={() => {
                  open(`https://warframe.market/items/${value.wfm_url}`);
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
            {value.price_history.length <= 0 && (
              <Center h={"100%"}>
                <Title order={3}>{useTranslateFields("no_listed")}</Title>
              </Center>
            )}
            {value.price_history.length > 0 &&
              value.price_history
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
            value.info?.orders?.slice(0, 5).map((order, index) => (
              <OrderItem
                show_border
                paperProps={{ mb: "sm" }}
                key={index}
                show_user
                order={{
                  ...order,
                  item: {
                    en: {
                      item_name: item?.name || "",
                    },
                    id: item?.wfm_id || "",
                    url_name: item?.wfm_url_name || "",
                    icon: item?.image_url || "",
                    icon_format: "png",
                    thumb: item?.image_url || "",
                    sub_icon: "",
                    mod_max_rank: item?.sub_type?.max_rank || 0,
                    subtypes: [],
                    tags: [],
                    ducats: 0,
                    quantity_for_set: 0,
                  },
                }}
              />
            ))}
        </ScrollArea>
      </Tabs.Panel>
    </Tabs>
  );
}

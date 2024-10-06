import { Group, TextInput, Grid, Title, Tabs, Center, ScrollArea, Button } from "@mantine/core";
import { StockRiven } from "@api/types";
import { useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { PriceHistoryListItem } from "@components/PriceHistory";
import { AuctionListItem } from "@components/AuctionListItem";

export type StockRivenInfoProps = {
  value: StockRiven;
};
export function StockRivenInfo({ value }: StockRivenInfoProps) {
  // Translate general
  const useTranslateStockRivenInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`stock_riven_info.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockRivenInfo(`tabs.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockRivenInfo(`fields.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockRivenInfo(`buttons.${key}`, { ...context }, i18Key);

  return (
    <Tabs defaultValue="general" h={"75vh"}>
      <Tabs.List>
        <Tabs.Tab value="general">{useTranslateTabs("general.title")}</Tabs.Tab>
        <Tabs.Tab value="auctions">{useTranslateTabs("auctions.title")}</Tabs.Tab>
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
                data-color-mode="text"
                data-stock-status={value.status}
                value={useTranslateStockStatus(value.status)}
                readOnly
              />
              <TextInput label={useTranslateFields("minimum_price")} value={value.minimum_price || "N/A"} readOnly />
              <TextInput label={useTranslateFields("re_rolls")} value={value.re_rolls || 0} readOnly />
              <TextInput label={useTranslateFields("mastery_rank")} value={value.mastery_rank || 0} readOnly />
              <TextInput label={useTranslateFields("rank")} value={value.sub_type?.rank || 0} readOnly />
            </Group>
            <Group grow>
              <TextInput label={useTranslateFields("bought")} value={value.bought} readOnly />
              <TextInput label={useTranslateFields("list_price")} value={value.list_price || "N/A"} readOnly />
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
                  window.open(
                    `https://warframe.market/auctions/search?type=riven&weapon_url_name=${value.wfm_weapon_url}&polarity=any&sort_by=price_asc`,
                    "_blank"
                  );
                }}
              >
                {useTranslateButtons("find_type")}
              </Button>
              <Button
                color="blue"
                variant="outline"
                onClick={() => {
                  // https://warframe.market/auctions/search?type=riven&weapon_url_name=cortege&positive_stats=cold_damage,critical_chance&negative_stats=critical_damage&polarity=any&sort_by=price_asc
                  window.open(
                    `https://warframe.market/auctions/search?type=riven&weapon_url_name=${
                      value.wfm_weapon_url
                    }&polarity=any&sort_by=price_asc&positive_stats=${value.attributes
                      .filter((x) => x.positive)
                      .map((x) => x.url_name)
                      .join(",")}&negative_stats=${value.attributes
                      .filter((x) => !x.positive)
                      .map((x) => x.url_name)
                      .join(",")}`,
                    "_blank"
                  );
                }}
              >
                {useTranslateButtons("find_similar")}
              </Button>
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

      <Tabs.Panel value="auctions">
        <ScrollArea h={"70.5vh"}>
          {!value.info?.auctions?.length && (
            <Center h={"100%"}>
              <Title order={3}>{useTranslateFields("no_auctions")}</Title>
            </Center>
          )}
          {(value.info?.auctions?.length || 0) > 0 &&
            value.info?.auctions?.slice(0, 5).map((auction, index) => <AuctionListItem show_border key={index} auction={auction} />)}
        </ScrollArea>
      </Tabs.Panel>
    </Tabs>
  );
}

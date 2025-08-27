import { Title, Grid, Group, Text, TextInput, Center } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { WFMAuction } from "@components/DataDisplay/WFMAuction";

export type WFMTabProps = {
  value: TauriTypes.StockRivenDetails | undefined;
};

export function WFMTab({ value }: WFMTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`stock_riven_details.tabs.wfm.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`fields.${key}`, { ...context }, i18Key);

  if (!value || !value.auction_info) return <Text ta="center">{useTranslateTab("no_auction_info")}</Text>;
  return (
    <Grid>
      <Grid.Col span={4}>
        <Group grow>
          <TextInput label={useTranslateFields("created_at")} value={dayjs(value.auction_info.created).format("DD/MM/YYYY HH:mm:ss")} readOnly />
          <TextInput label={useTranslateFields("updated_at")} value={dayjs(value.auction_info.updated).format("DD/MM/YYYY HH:mm:ss")} readOnly />
        </Group>
      </Grid.Col>
      <Grid.Col span={8}>
        <Title order={3}>{useTranslateFields("auction_list")}</Title>
        {value.stock.price_history.length <= 0 && (
          <Center h={"100%"}>
            <Title order={3}>{useTranslateFields("no_listed")}</Title>
          </Center>
        )}
        {value.auction_info.properties?.auctions.map((auction) => (
          <WFMAuction display_style="list" key={auction.id} auction={auction} />
        ))}
      </Grid.Col>
    </Grid>
  );
}

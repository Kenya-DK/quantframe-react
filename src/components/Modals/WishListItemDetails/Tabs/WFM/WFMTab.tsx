import { Title, Grid, Group, Text, TextInput, Center, Button } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { WFMOrder } from "@components/DataDisplay/WFMOrder";

export type WFMTabProps = {
  value: TauriTypes.WishListItemDetails | undefined;
};

export function WFMTab({ value }: WFMTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`wish_list_details.tabs.wfm.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`buttons.${key}`, { ...context }, i18Key);

  if (!value) return <Text ta="center">{useTranslateTab("no_order_info")}</Text>;
  if (!value.order_info)
    return (
      <Button
        mt={"md"}
        color="blue"
        variant="outline"
        onClick={() => {
          open(`https://warframe.market/items/${value.stock.wfm_url}`);
        }}
      >
        {useTranslateButtons("wfm")}
      </Button>
    );
  return (
    <Grid>
      <Grid.Col span={6}>
        <Group grow>
          <TextInput label={useTranslateFields("created_at")} value={dayjs(value.order_info.createdAt).format("DD/MM/YYYY HH:mm:ss")} readOnly />
          <TextInput label={useTranslateFields("updated_at")} value={dayjs(value.order_info.updatedAt).format("DD/MM/YYYY HH:mm:ss")} readOnly />
        </Group>
        <Button
          mt={"md"}
          color="blue"
          variant="outline"
          onClick={() => {
            open(`https://warframe.market/items/${value.stock.wfm_url}`);
          }}
        >
          {useTranslateButtons("wfm")}
        </Button>
      </Grid.Col>
      <Grid.Col span={6}>
        <Title order={3}>{useTranslateFields("order_list")}</Title>
        {value.stock.price_history.length <= 0 && (
          <Center h={"100%"}>
            <Title order={3}>{useTranslateFields("no_listed")}</Title>
          </Center>
        )}
        {value.order_info.properties?.orders.map((order) => (
          <WFMOrder display_style="list" show_border paperProps={{ mb: "sm" }} show_user order={order} key={order.id} />
        ))}
      </Grid.Col>
    </Grid>
  );
}

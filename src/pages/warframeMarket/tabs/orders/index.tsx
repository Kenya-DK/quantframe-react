import { Button, Divider, Flex, Group, Stack, useMantineTheme, Text, Image, Box } from "@mantine/core";
import { useCacheContext, useWarframeMarketContextContext } from "@contexts/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCheck, faCubes } from "@fortawesome/free-solid-svg-icons";
import { Wfm } from "$types/index";
import { useTranslatePage } from "@hooks/index";
import api, { wfmThumbnail } from "@api/index";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
interface OrdersPanelProps {
}
interface PurchaseNewItemProps {
  max_rank: number;
  ordre: Wfm.OrderDto;
  type: "buy" | "sell";
}
const OrderItem = ({ type, max_rank, ordre }: PurchaseNewItemProps) => {
  const translateBase = (key: string, context?: { [key: string]: any }) => useTranslatePage(`warframe_market.${key}`, { ...context })
  const theme = useMantineTheme();
  return (
    <Group grow mt={10}>
      <Group>
        <Image width={48} height={48} fit="contain" src={wfmThumbnail(ordre.item.icon)} alt="Without placeholder" caption={type == "buy" ? translateBase("buy_label") : translateBase("sell_label")} />
        <Stack spacing="xs">
          <Text size="md" weight={500} >{ordre.item.en.item_name}</Text>
          <Flex
            gap="sm"
            justify="flex-start"
            align="flex-start"
            direction="row"
            wrap="wrap"
          >
            <Text size="sm" component="span" weight={500} color="gray.6">{ordre.quantity}</Text>
            <FontAwesomeIcon icon={faCubes} color={theme.colors.gray[6]} />
            <Divider orientation="vertical" color="gray.6" />
            {ordre.item.mod_max_rank && (
              <Text size="sm" color="gray.6">
                {translateBase("rank_label", { max_rank: max_rank, rank: ordre.item.mod_max_rank })}
              </Text>
            )}
          </Flex>
          <Text size="sm" weight={500} color="green.7">{ordre.platinum} Platinum each</Text>
        </Stack>
      </Group>
      <Group position="right" >
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.delete")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.edit")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.bought")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.visible")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.sold")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.hidden")}</Button>
      </Group>
    </Group>
  );
}

export const OrdersPanel = ({ }: OrdersPanelProps) => {
  const useTranslateOrdersPanel = (key: string, context?: { [key: string]: any }) => useTranslatePage(`warframe_market.tabs.orders.${key}`, { ...context })
  const useTranslateNotifaications = (key: string, context?: { [key: string]: any }) => useTranslateOrdersPanel(`notifaications.${key}`, { ...context })
  const useTranslateButtons = (key: string, context?: { [key: string]: any }) => useTranslateOrdersPanel(`buttons.${key}`, { ...context })
  const { orders } = useWarframeMarketContextContext();
  const { items } = useCacheContext();
  const refreshOrdersMutation = useMutation(() => api.auction.refresh(), {
    onSuccess: async () => {
      notifications.show({
        title: useTranslateNotifaications("refresh_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("refresh_message"),
        color: "green"
      });
    },
    onError: () => { },
  })

  return (
    <Box >
      <Button onClick={async () => {
        refreshOrdersMutation.mutate();
      }}>{useTranslateButtons("refresh")}</Button>
      {orders.map((order) => (
        <OrderItem type={order.order_type} max_rank={items.find(x => x.id == order.item.id)?.mod_max_rank || 0} ordre={order} />
      ))}
    </Box>)
}
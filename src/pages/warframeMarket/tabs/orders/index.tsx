import { Divider, Flex, Group, Stack, useMantineTheme, Text, Image, Box, Grid, Tooltip, ActionIcon } from "@mantine/core";
import { useCacheContext, useWarframeMarketContextContext } from "@contexts/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCheck, faCubes, faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { Wfm } from "$types/index";
import { useTranslatePage } from "@hooks/index";
import api, { wfmThumbnail } from "@api/index";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { SearchField } from "../../../../components/searchfield";
import { useState } from "react";
import { modals } from "@mantine/modals";
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
      {/* <Group position="right" >
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.delete")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.edit")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.bought")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.visible")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.sold")}</Button>
        <Button color="blue" variant="outline" size="sm">{translateBase("buttons.hidden")}</Button>
      </Group> */}
    </Group>
  );
}

export const OrdersPanel = ({ }: OrdersPanelProps) => {
  const useTranslateOrdersPanel = (key: string, context?: { [key: string]: any }) => useTranslatePage(`warframe_market.tabs.orders.${key}`, { ...context })
  const useTranslateNotifaications = (key: string, context?: { [key: string]: any }) => useTranslateOrdersPanel(`notifaications.${key}`, { ...context })
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }) => useTranslateOrdersPanel(`prompt.${key}`, { ...context })
  const { orders } = useWarframeMarketContextContext();
  const { items } = useCacheContext();
  const [query, setQuery] = useState<string>("");

  const refreshOrdersMutation = useMutation(() => api.auction.refresh(), {
    onSuccess: async () => {
      notifications.show({
        title: useTranslateNotifaications("refresh.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("refresh.message"),
        color: "green"
      });
    },
    onError: () => { },
  })
  const deleteAllOrdersMutation = useMutation(() => api.orders.delete_all(), {
    onSuccess: async (count) => {
      notifications.show({
        title: useTranslateNotifaications("delete_all.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("delete_all.message", { count: count }),
        color: "green"
      });
    },
    onError: () => { },
  })


  const getOrders = () => {
    if (query.length > 0) {
      return orders.filter((x) => x.item.en.item_name.toLowerCase().includes(query.toLowerCase()));
    }
    return orders;
  }

  return (
    <Box >
      <Grid>
        <Grid.Col span={12}>
          <SearchField value={query} onChange={(text) => setQuery(text)}
            rightSectionWidth={80}
            rightSection={
              <Group spacing={5}>

                <Tooltip label={useTranslateOrdersPanel('tolltip.refresh')}>
                  <ActionIcon variant="filled" color="green.7" onClick={() => {
                    refreshOrdersMutation.mutate();
                  }}>
                    <FontAwesomeIcon icon={faRefresh} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label={useTranslateOrdersPanel('tolltip.delete_all')}>
                  <ActionIcon loading={deleteAllOrdersMutation.isLoading} variant="filled" color="red.7" onClick={() => {
                    modals.openConfirmModal({
                      title: useTranslatePrompt('delete_all.title'),
                      children: (<Text>
                        {useTranslatePrompt('delete_all.message')}
                      </Text>),
                      labels: {
                        confirm: useTranslatePrompt('delete_all.confirm'),
                        cancel: useTranslatePrompt('delete_all.cancel')
                      },
                      confirmProps: { color: 'red' },
                      onConfirm: async () => {
                        deleteAllOrdersMutation.mutate();
                      }
                    })
                  }}>
                    <FontAwesomeIcon icon={faTrashCan} />
                  </ActionIcon>
                </Tooltip>
              </Group>
            }
          />
        </Grid.Col>
      </Grid>
      {getOrders().map((order) => (
        <OrderItem type={order.order_type} max_rank={items.find(x => x.id == order.item.id)?.mod_max_rank || 0} ordre={order} />
      ))}
    </Box>)
}
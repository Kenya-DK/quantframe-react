import { Divider, Group, Stack, Text, Image, Box, Grid, Tooltip, ActionIcon, Paper, SimpleGrid, ScrollArea, useMantineTheme } from "@mantine/core";
import { useCacheContext, useWarframeMarketContextContext } from "@contexts/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCheck, faTrashCan, faRefresh, faCartShopping, faShoppingCart, faPen } from "@fortawesome/free-solid-svg-icons";
import { Wfm, CreateStockItemEntryDto, RustError } from "$types/index";
import { useTranslatePage, useTranslateRustError } from "@hooks/index";
import api, { wfmThumbnail } from "@api/index";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { SearchField } from "@components/searchfield";
import { useEffect, useState } from "react";
import { modals } from "@mantine/modals";
import { TextColor } from "@components/textColor";
import { InfoBox } from "@components/InfoBox";
import { SendNotificationToWindow, formatNumber } from "@utils/index";
interface OrdersPanelProps {
}
interface PurchaseNewItemProps {
  item: Wfm.ItemDto | undefined;
  ordre: Wfm.OrderDto;
  type: "buy" | "sell";
}
const OrderItem = ({ item, ordre }: PurchaseNewItemProps) => {
  const useTranslateNotifaications = (key: string, context?: { [key: string]: any }) => useTranslateOrdersPanel(`notifaications.${key}`, { ...context })
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }) => useTranslateOrdersPanel(`prompt.${key}`, { ...context });
  const theme = useMantineTheme();
  const createStockItemEntryMutation = useMutation((data: CreateStockItemEntryDto) => api.stock.item.create(data), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("createStockItem.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("createStockItem.message", { name: data.name }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })

  const sellStockItemEntryMutation = useMutation((data: { url: string, price: number }) => api.stock.item.sell_by_name(data.url, data.price, 1), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("sellStockItem.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("sellStockItem.message", { name: data.name, price: data.listed_price }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })
  const deleteOrdreEntryMutation = useMutation((data: { id: string }) => api.orders.deleteOrder(data.id), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("delete_ordre.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("delete_ordre.message", { name: data.item.en.item_name }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })
  const useTranslateOrdersPanel = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`warframe_market.tabs.orders.${key}`, { ...context }, i18Key)
  const handleCartClick = async (price?: number) => {
    switch (ordre.order_type) {
      case "buy":
        createStockItemEntryMutation.mutate({
          item_id: ordre.item.url_name,
          price: price || ordre.platinum,
          quantity: 1,
          rank: ordre.mod_rank || 0
        });
        break;
      case "sell":
        sellStockItemEntryMutation.mutate({
          url: ordre.item.url_name,
          price: price || ordre.platinum
        });
        break;
      default:
        break;

    }
  }

  return (
    <Paper p={10} sx={{
      boxShadow: `inset 4px 0 0 0 ${ordre.order_type === "buy" ? theme.colors.green[7] : theme.colors.violet[7]}`,
    }}>
      <Stack spacing={0}>
        <Group position="apart">
          <Group w={"75%"} >
            <Text size="lg" weight={700} truncate="end">
              {ordre.item.en.item_name}
            </Text>
          </Group>
          <Group position="right">
            <TextColor size={"lg"} sx={{ float: "inline-end" }} color="gray.6" i18nKey={useTranslateOrdersPanel("quantity_label", undefined, true)} values={{ quantity: ordre.quantity }} />
          </Group>
        </Group>
        <Divider />
        <Grid mt={5} mb={5}>
          <Grid.Col sm={4} md={4} lg={2.5}>
            <Tooltip label={ordre.item.en.item_name}>
              <Image ml={15} width={64} height={64} fit="contain" src={wfmThumbnail(ordre.item.icon)} />
            </Tooltip>
          </Grid.Col>
          <Grid.Col sm={9} md={9} lg={8.6} sx={{ display: "flex", alignItems: "center", justifyContent: "flex-end" }}>
            <Stack spacing={0}>
              {ordre.mod_rank && (
                <TextColor size={"md"} color="gray.6" i18nKey={useTranslateOrdersPanel("rank_label", undefined, true)} values={{ max_rank: item?.mod_max_rank || 0, rank: ordre.item.mod_max_rank }} />
              )}
              {ordre.subtype && (
                <Text size="sm" color="gray.6">
                  Rank: {ordre.subtype}
                </Text>
              )}
              {ordre.item.vaulted && (
                <Text size="sm" color="yellow.6">
                  Vaulted
                </Text>
              )}
            </Stack>
          </Grid.Col>
        </Grid>

        <Divider />
        <Grid mt={5} p={0}>
          <Grid.Col md={8} p={0} pl={10} >
            <TextColor size={"lg"} sx={{ float: "inline-start", marginRight: 15 }} color="gray.6" i18nKey={useTranslateOrdersPanel("plat_label", undefined, true)} values={{ plat: ordre.platinum }} />
            <TextColor size={"lg"} sx={{ display: "flex", alignItems: "center" }} color="gray.6" i18nKey={useTranslateOrdersPanel("credits_label", undefined, true)} values={{ credits: formatNumber(item?.trade_tax || 0) }} />
          </Grid.Col>
          <Grid.Col md={4} p={0}>
            <Group spacing={1}>

              <Tooltip label={useTranslateOrdersPanel(ordre.order_type === "buy" ? "tolltip.buy_add_to_stock" : "tolltip.sell_remove_from_stock")}>
                <ActionIcon loading={deleteOrdreEntryMutation.isLoading} color="bule.7" onClick={async (e) => {
                  e.stopPropagation();
                  modals.openContextModal({
                    modal: 'prompt',
                    title: useTranslatePrompt("sell.title"),
                    innerProps: {
                      fields: [{ name: 'price', description: useTranslatePrompt("sell.description"), label: useTranslatePrompt("sell.label"), type: 'number', value: 0, }],
                      onConfirm: async (data: { price: number }) => {
                        const { price } = data;
                        if (!price || price <= 0) return;
                        await handleCartClick(price);
                      },
                      onCancel: (id: string) => modals.close(id),
                    },
                  })
                }} >
                  <FontAwesomeIcon icon={faPen} />
                </ActionIcon>
              </Tooltip>
              <Tooltip label={useTranslateOrdersPanel(ordre.order_type === "buy" ? "tolltip.buy_add_to_stock" : "tolltip.sell_remove_from_stock")}>
                <ActionIcon loading={sellStockItemEntryMutation.isLoading || createStockItemEntryMutation.isLoading} color="green.7" onClick={async () => handleCartClick()} >
                  <FontAwesomeIcon icon={faCartShopping} />
                </ActionIcon>
              </Tooltip>
              <Tooltip label={useTranslateOrdersPanel("tolltip.delete")}>
                <ActionIcon loading={deleteOrdreEntryMutation.isLoading} color="red.7" onClick={async () => {
                  deleteOrdreEntryMutation.mutate({ id: ordre.id })
                }} >
                  <FontAwesomeIcon icon={faTrashCan} />
                </ActionIcon>
              </Tooltip>
            </Group>
          </Grid.Col>
        </Grid>
      </Stack>
    </Paper>
  );
}

export const OrdersPanel = ({ }: OrdersPanelProps) => {
  const useTranslateOrdersPanel = (key: string, context?: { [key: string]: any }) => useTranslatePage(`warframe_market.tabs.orders.${key}`, { ...context })
  const useTranslateNotifaications = (key: string, context?: { [key: string]: any }) => useTranslateOrdersPanel(`notifaications.${key}`, { ...context })
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }) => useTranslateOrdersPanel(`prompt.${key}`, { ...context })
  const { orders } = useWarframeMarketContextContext();
  const { items } = useCacheContext();
  const [query, setQuery] = useState<string>("");
  const [order_type, setOrderType] = useState<"buy" | "sell" | "all">("all");
  const theme = useMantineTheme();

  const [buyOrders, setBuyOrders] = useState<Wfm.OrderDto[]>([]);
  const [sellOrders, setSellOrders] = useState<Wfm.OrderDto[]>([]);

  useEffect(() => {
    setBuyOrders(orders.filter(x => x.order_type == "buy"));
    setSellOrders(orders.filter(x => x.order_type == "sell"));
  }, [orders]);

  const refreshOrdersMutation = useMutation(() => api.orders.refresh(), {
    onSuccess: async () => {
      notifications.show({
        title: useTranslateNotifaications("refresh.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("refresh.message"),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })

  const deleteAllOrdersMutation = useMutation(() => api.orders.delete_all(), {
    onSuccess: async () => { },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })

  const getFilterOrders = () => {
    let ordersF = orders;
    if (order_type != "all")
      ordersF = orders.filter(x => x.order_type == order_type);
    else
      ordersF = orders;

    if (query != "")
      ordersF = ordersF.filter((x) => x.item.en.item_name.toLowerCase().includes(query.toLowerCase()));

    // Sort by order_type
    return ordersF.sort((a, b) => {
      if (a.order_type == "buy" && b.order_type == "sell")
        return 1;
      if (a.order_type == "sell" && b.order_type == "buy")
        return -1;
      return 0;
    });
  }

  return (
    <Box >
      <Grid>
        <Grid.Col span={12}>
          <SearchField value={query} onChange={(text) => setQuery(text)}
            rightSectionWidth={100}
            rightSection={
              <Group spacing={5}>
                <Tooltip label={useTranslateOrdersPanel('tolltip.refresh')}>
                  <ActionIcon variant="filled" color="green.7" onClick={() => {
                    refreshOrdersMutation.mutate();
                  }}>
                    <FontAwesomeIcon icon={faRefresh} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label={useTranslateOrdersPanel(`sort.${order_type}`)}>
                  <ActionIcon variant="filled"
                    color={order_type == "buy" ? "green.7" : order_type == "sell" ? "violet.7" : "gray.7"}
                    onClick={() => {
                      // Switch order type
                      if (order_type == "buy")
                        setOrderType("sell");
                      else if (order_type == "sell")
                        setOrderType("all");
                      else
                        setOrderType("buy");
                    }}>
                    <FontAwesomeIcon icon={faShoppingCart} />
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
          <Group mt={15} >
            <InfoBox text={useTranslateOrdersPanel("info.buy", { count: buyOrders.length, plat: buyOrders.reduce((a, b) => a + (b.platinum || 0) * b.quantity, 0) || 0 })} color={theme.colors.green[7]} />
            <InfoBox text={useTranslateOrdersPanel("info.sell", { count: sellOrders.length, plat: sellOrders.reduce((a, b) => a + (b.platinum || 0) * b.quantity, 0) || 0 })} color={theme.colors.violet[7]} />
          </Group>
        </Grid.Col>
      </Grid>
      <ScrollArea mt={25} h={"calc(100vh - 243px)"} pr={15} pl={15}>
        <SimpleGrid
          cols={4}
          spacing="lg"
          breakpoints={[
            { maxWidth: '80rem', cols: 3, spacing: 'lg' },
            { maxWidth: '62rem', cols: 3, spacing: 'md' },
            { maxWidth: '48rem', cols: 2, spacing: 'sm' },
            { maxWidth: '36rem', cols: 1, spacing: 'sm' },
          ]}
        >
          {getFilterOrders().map((order, i) => (
            <OrderItem key={i} type={order.order_type} item={items.find(x => x.id == order.item.id)} ordre={order} />
          ))}
        </SimpleGrid>
      </ScrollArea>
    </Box>)
}
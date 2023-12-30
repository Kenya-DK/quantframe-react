import { ActionIcon, Box, Grid, Group, Stack, Tooltip, Text, ScrollArea } from "@mantine/core";
import api from "@api/index";
import { useStockContextContext, useWarframeMarketContextContext } from "@contexts/index";
import Auction from "@components/auction";
import { modals } from "@mantine/modals";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCheck, faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePage, useTranslateRustError } from "@hooks/index";
import { useState } from "react";
import { SearchField } from "@components/searchfield";
import { RustError, Wfm } from "$types/index";
import { SendNotificationToWindow, getOrderStatusColorCode } from "@utils/index";
import { InfoBox } from "../../../../components/InfoBox";

interface AuctionsPanelProps {
}
export const AuctionsPanel = ({ }: AuctionsPanelProps) => {
  const useTranslateAuctionsPanel = (key: string, context?: { [key: string]: any }) => useTranslatePage(`warframe_market.tabs.auctions.${key}`, { ...context })
  const useTranslateNotifaications = (key: string, context?: { [key: string]: any }) => useTranslateAuctionsPanel(`notifaications.${key}`, { ...context })
  const useTranslatePrompts = (key: string, context?: { [key: string]: any }) => useTranslateAuctionsPanel(`prompt.${key}`, { ...context })
  const [query, setQuery] = useState<string>("");
  const { auctions } = useWarframeMarketContextContext();
  const { rivens } = useStockContextContext();
  const importRivenEntryMutation = useMutation((data: { id: string, price: number }) => api.stock.riven.import_auction(data.id, data.price), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("import.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("import.message", { name: `${data.name} ${data.mod_name}` }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })
  const refreshAuctionsMutation = useMutation(() => api.auction.refresh(), {
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
  const deleteAllAuctionsMutation = useMutation(() => api.auction.delete_all(), {
    onSuccess: async (count) => {
      notifications.show({
        title: useTranslateNotifaications("delete_all.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("delete_all.message", { count: count }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })

  const getAuctions = () => {
    if (query.length > 0)
      return auctions.filter((x) => x.item.name.toLowerCase().includes(query.toLowerCase()) ||
        x.item.weapon_url_name.toLowerCase().includes(query.toLowerCase()));
    return auctions;
  }
  const getFindStock = (id: string) => {
    return rivens.find(x => x.order_id === id);
  }
  return (
    <Box>
      <Grid>
        <Grid.Col span={12}>
          <SearchField value={query} onChange={(text) => setQuery(text)}
            rightSectionWidth={80}
            rightSection={
              <Group spacing={5}>

                <Tooltip label={useTranslateAuctionsPanel('tolltip.refresh')}>
                  <ActionIcon variant="filled" color="green.7" onClick={() => {
                    refreshAuctionsMutation.mutate();
                  }}>
                    <FontAwesomeIcon icon={faRefresh} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label={useTranslateAuctionsPanel('tolltip.delete_all')}>
                  <ActionIcon loading={deleteAllAuctionsMutation.isLoading} variant="filled" color="red.7" onClick={() => {
                    modals.openConfirmModal({
                      title: useTranslatePrompts('delete_all.title'),
                      children: (<Text>
                        {useTranslatePrompts('delete_all.message')}
                      </Text>),
                      labels: {
                        confirm: useTranslatePrompts('delete_all.confirm'),
                        cancel: useTranslatePrompts('delete_all.cancel')
                      },
                      confirmProps: { color: 'red' },
                      onConfirm: async () => {
                        deleteAllAuctionsMutation.mutate();
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
            <InfoBox text={useTranslateAuctionsPanel("info.inactive")} color={getOrderStatusColorCode(Wfm.OrderStatus.Inactive)} />
            <InfoBox text={useTranslateAuctionsPanel("info.is_imported")} color={getOrderStatusColorCode(Wfm.OrderStatus.Live)} />
            <InfoBox text={useTranslateAuctionsPanel("info.is_not_imported")} color={getOrderStatusColorCode(Wfm.OrderStatus.Pending)} />
          </Group>
        </Grid.Col>
      </Grid>
      <ScrollArea p="xs" scrollbarSize={1} h={"calc(100vh - 215px)"} >
        <Stack mt={25}>
          {getAuctions().map((auction) => (
            <Auction key={auction.id} stock_item={getFindStock(auction.id)} auction={auction}
              onImport={(a) => {
                modals.openContextModal({
                  modal: 'prompt',
                  title: useTranslatePrompts("import.title"),
                  innerProps: {
                    fields: [{ name: 'price', description: useTranslatePrompts("import.description"), label: useTranslatePrompts("import.label"), type: 'number', value: 0, placeholder: useTranslatePrompts("import.placeholder") }],
                    onConfirm: async (data: { price: number }) => {
                      const { price } = data;
                      importRivenEntryMutation.mutate({
                        id: a.id,
                        price
                      });
                    },
                    onCancel: (id: string) => modals.close(id),
                  },
                })
              }} />
          ))}
        </Stack>
      </ScrollArea>
    </Box>
  )
}
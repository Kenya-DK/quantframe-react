import { Box, Button, Stack } from "@mantine/core";
import api from "@api/index";
import { useWarframeMarketContextContext } from "../../../../contexts";
import Auction from "../../../../components/auction";
import { modals } from "@mantine/modals";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCheck } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePage } from "../../../../hooks";
interface AuctionsPanelProps {
}
export const AuctionsPanel = ({ }: AuctionsPanelProps) => {
  const useTranslateAuctionsPanel = (key: string, context?: { [key: string]: any }) => useTranslatePage(`warframe_market.tabs.auctions.${key}`, { ...context })
  const useTranslateNotifaications = (key: string, context?: { [key: string]: any }) => useTranslateAuctionsPanel(`notifaications.${key}`, { ...context })
  const useTranslateButtons = (key: string, context?: { [key: string]: any }) => useTranslateAuctionsPanel(`buttons.${key}`, { ...context })
  const useTranslatePrompts = (key: string, context?: { [key: string]: any }) => useTranslateAuctionsPanel(`prompt.${key}`, { ...context })

  const { auctions } = useWarframeMarketContextContext();

  const importRivenEntryMutation = useMutation((data: { id: string, price: number }) => api.stock.riven.import_auction(data.id, data.price), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("import_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("import_message", { name: `${data.name} ${data.mod_name}` }),
        color: "green"
      });
    },
    onError: () => { },
  })
  const refreshAuctionsMutation = useMutation(() => api.auction.refresh(), {
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
    <Box>
      <Button loading={refreshAuctionsMutation.isLoading} onClick={async () => {
        refreshAuctionsMutation.mutate();
      }}>
        {useTranslateButtons("refresh")}
      </Button>
      <Stack>
        {auctions.map((auction) => (
          <Auction key={auction.id} auction={auction}
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
    </Box>
  )
}
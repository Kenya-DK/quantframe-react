import { Button, Container, Stack } from "@mantine/core";
import { useCacheContext, useWarframeMarketContextContext } from "../../contexts";
import Auction from "../../components/auction";
import { useMutation } from "@tanstack/react-query";
import api from "../../api";
import { modals } from "@mantine/modals";
import { useTranslatePage } from "../../hooks";
import { RivenForm } from "../../components/forms/riven.form";
import { CreateStockRivenEntryDto } from "../../types";
export default function AuctionsPage() {
  const { auctions } = useWarframeMarketContextContext();
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslatePage(`auctions.${key}`, { ...context })
  const { riven_attributes, riven_items } = useCacheContext();

  const createRivenEntryMutation = useMutation((data: CreateStockRivenEntryDto) => api.stock.riven.create(data), {
    onSuccess: async (data) => {
      console.log(data);
    },
    onError: () => {
      console.log("error");

    },
  })
  const importRivenEntryMutation = useMutation((data: { id: string, price: number }) => api.stock.riven.import_auction(data.id, data.price), {
    onSuccess: async (data) => {
      console.log(data);
    },
    onError: () => {

    },
  })
  return (
    <Container size="md">
      <h1>Auctions</h1>
      <Button onClick={async () => {
        await api.auction.refresh();
      }}>Refresh</Button>
      <Button onClick={() => {
        modals.open({
          size: "100%",
          onClose: () => window.history.replaceState(null, "", "/orders"),
          withCloseButton: false,
          children: <RivenForm
            availableRivens={riven_items}
            availableAttributes={riven_attributes}
            onSubmit={(data) => {
              createRivenEntryMutation.mutate({
                item_id: data.url_name,
                rank: data.mod_rank,
                price: 0,
                mod_name: data.mod_name,
                attributes: data.attributes,
                mastery_rank: data.mastery_rank,
                re_rolls: data.re_rolls,
                polarity: data.polarity,
              });
            }}
          />,
        });
      }}>Add</Button>
      <Stack spacing="xs">
        {auctions.map((auction) => (
          <Auction key={auction.id} auction={auction}

            onImport={(a) => {
              modals.openContextModal({
                modal: 'prompt',
                title: useTranslate("prompt.price.title"),
                innerProps: {
                  fields: [{ name: 'price', description: useTranslate("prompt.price.description"), label: useTranslate("prompt.price.label"), type: 'number', value: 0, placeholder: useTranslate("prompt.price.placeholder") }],
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
            }}
          />
        ))}
      </Stack>
    </Container>
  );
}

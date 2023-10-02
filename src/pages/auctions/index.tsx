import { Button, Container, Group } from "@mantine/core";
import { useStockContextContext, useWarframeMarketContextContext } from "../../contexts";
import { CreateStockRivenEntryDto, Wfm } from "../../types";
import api from "@api/index";
import { useMutation } from "@tanstack/react-query";
export default function AuctionsPage() {
  const { auctions } = useWarframeMarketContextContext();
  const { rivens } = useStockContextContext();
  const createRivenEntryMutation = useMutation((data: CreateStockRivenEntryDto) => api.stock.riven.create(data), {
    onSuccess: async (data) => {
      console.log(data);
    },
    onError: () => {

    },
  })
  return (
    <Container size="md">
      <h1>Auctions</h1>
      {auctions.map((auction) => (
        <Group key={auction.id}>
          <Button onClick={() => {
            let item = auction.item as Wfm.AuctionItemRiven;
            createRivenEntryMutation.mutate({
              item_id: item.weapon_url_name,
              rank: item.mod_rank,
              price: 200,
              attributes: item.attributes,
              mastery_rank: item.mastery_level,
              polarity: item.polarity,
              re_rolls: item.re_rolls,
              mod_name: item.name,
            })
          }}>Import</Button>
        </Group>
      ))}
      {rivens.map((riven) => (
        <div key={riven.id}>{riven.id}</div>
      ))}
    </Container>
  );
}

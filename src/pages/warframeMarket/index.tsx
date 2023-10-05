import { Stack, Image, Text, Group, Flex, Divider, useMantineTheme, Button, Tabs, Box } from "@mantine/core";
import { Wfm } from "../../types";
import api, { wfmThumbnail } from "@api/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCubes } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePage } from "../../hooks";
import { useCacheContext, useWarframeMarketContextContext } from "../../contexts";
import Auction from "../../components/auction";
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

export default function WarframeMarketPage() {
  const { orders, auctions } = useWarframeMarketContextContext();
  const { items } = useCacheContext();
  return (
    <Box p={0} m={0}>
      <Tabs defaultValue="orders">
        <Tabs.List>
          <Tabs.Tab value="orders" >
            Orders
          </Tabs.Tab>
          <Tabs.Tab value="contracts">
            Contracts
          </Tabs.Tab>
          <Tabs.Tab value="bids" >
            Bids
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="orders">
          <Button onClick={async () => {
            await api.orders.refresh();
          }}>Refresh</Button>
          {orders.map((order) => (
            <OrderItem type={order.order_type} max_rank={items.find(x => x.id == order.item.id)?.mod_max_rank || 0} ordre={order} />
          ))}
        </Tabs.Panel>

        <Tabs.Panel value="contracts">
          <Button onClick={async () => {
            await api.auction.refresh();
          }}>Refresh</Button>
          <Stack>
            {auctions.map((auction) => (
              <Auction key={auction.id} auction={auction} />
            ))}
          </Stack>
        </Tabs.Panel>

        <Tabs.Panel value="bids">
          Settings tab content
        </Tabs.Panel>
      </Tabs>
    </Box>
  );
}

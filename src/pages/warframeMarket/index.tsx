import { Grid, Stack, Image, Text, Group, Flex, Divider, useMantineTheme, Button } from "@mantine/core";
import { Wfm } from "../../types";
import { wfmThumbnail } from "@api/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCubes } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePage } from "../../hooks";
import { useCacheContext, useWarframeMarketContextContext } from "../../contexts";
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
  const { orders } = useWarframeMarketContextContext();
  const [buyOrders] = [orders.filter((order) => order.order_type === "buy"), orders.filter((order) => order.order_type === "sell")];
  const { items } = useCacheContext();
  return (
    <Grid>
      <Grid.Col md={6}>
        {buyOrders.map((order) => (
          <OrderItem type="buy" max_rank={items.find(x => x.id == order.item.id)?.mod_max_rank || 0} ordre={order} />
        ))}
      </Grid.Col>
      <Grid.Col md={6}>
        {/* {sellOrders.map((order) => (
          <OrderItem ordre={order} />
        ))} */}
      </Grid.Col>
    </Grid>
  );
}

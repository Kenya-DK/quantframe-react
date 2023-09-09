import { Grid, Group } from "@mantine/core";
import { useTauriContext } from "@contexts/index";
import { Wfm } from "../../types";


interface PurchaseNewItemProps {
  ordre: Wfm.OrderDto;
}
const OrderItem = ({ordre}: PurchaseNewItemProps) => {
  return (
    <Group grow position="center" >
      {ordre.item.en.item_name}
    <Group grow position="center" >
      {ordre.item.en.item_name}

    </Group>
    <Group grow position="center" >
      {ordre.item.en.item_name}
    </Group>
    <Group grow position="center" >
      {ordre.item.en.item_name}
    </Group>
    </Group>
  );
}

export default function WarframeMarketPage() {
  const { orders } = useTauriContext();
  const [buyOrders, sellOrders] = [orders.filter((order) => order.order_type === "buy"), orders.filter((order) => order.order_type === "sell")];
  return (
    <Grid>
      <Grid.Col md={6}>
        {buyOrders.map((order) => (
          <div key={order.id}>{order.item.en.item_name}</div>
        ))}
      </Grid.Col>
      <Grid.Col md={6}>
        {sellOrders.map((order) => (
          <div key={order.id}>{order.item.en.item_name}</div>
        ))}
      </Grid.Col>
    </Grid>
  );
}

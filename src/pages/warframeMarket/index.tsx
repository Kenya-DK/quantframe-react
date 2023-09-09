import { Grid } from "@mantine/core";
import { useTauriContext } from "@contexts/index";

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

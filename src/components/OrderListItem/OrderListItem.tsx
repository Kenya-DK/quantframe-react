import { Group, Paper, Text } from "@mantine/core";
import dayjs from "dayjs";
import classes from "./OrderListItem.module.css";
import { WFMarketTypes } from "../../types";
export type OrderListItemProps = {
  order: WFMarketTypes.OrderDto;
};

export function OrderListItem({ order }: OrderListItemProps) {
  return (
    <Paper mt={5} classNames={classes} p={5} data-order-type={order.order_type}>
      <Group justify="space-between">
        <Group ml={10} w={"35%"}>
          <Text c="gray.4">{order.user?.ingame_name}</Text>
          {order.quantity > 1 && <Text c="gray.4">{order.quantity}x</Text>}
          {/* <Text c="blue.5">{GetSubTypeDisplay(order.sub_type)} </Text> */}
        </Group>
        <Group w={100}>
          <Text c="blue.5">{order.platinum} </Text>
        </Group>
        <Group justify="right">
          <Text c="gray.4">{dayjs(order.creation_date).format("DD/MM/YYYY HH:mm:ss")}</Text>
        </Group>
      </Group>
    </Paper>
  );
}

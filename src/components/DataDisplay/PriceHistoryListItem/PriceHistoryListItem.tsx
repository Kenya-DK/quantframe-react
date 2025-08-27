import { Group, Paper, Text } from "@mantine/core";
import classes from "./PriceHistoryListItem.module.css";
import { PriceHistory } from "$types";
import { TimerStamp } from "../../Shared/TimerStamp";
export type PriceHistoryListItemProps = {
  history: PriceHistory;
};

export function PriceHistoryListItem({ history }: PriceHistoryListItemProps) {
  return (
    <Paper mt={5} classNames={classes} p={5}>
      <Group justify="space-between">
        <Group w={100}>
          <Text c="blue.5">{history.price} </Text>
        </Group>
        <Group justify="right">
          <TimerStamp date={history.created_at} />
        </Group>
      </Group>
    </Paper>
  );
}

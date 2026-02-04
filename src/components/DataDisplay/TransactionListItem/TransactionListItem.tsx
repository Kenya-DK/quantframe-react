import { memo } from "react";
import { Group, Paper, Stack, Text } from "@mantine/core";
import { TauriTypes } from "$types";
import dayjs from "dayjs";
import classes from "./TransactionListItem.module.css";
import { ItemName } from "../ItemName/ItemName";
export type TransactionListItemProps = {
  transaction: TauriTypes.TransactionDto;
  orientation?: "horizontal" | "vertical";
};

export const TransactionListItem = memo(function TransactionListItem({ transaction, orientation = "horizontal" }: TransactionListItemProps) {
  return (
    <Paper mt={5} classNames={classes} p={5} data-transaction-type={transaction.transaction_type} data-color-mode="box-shadow">
      {orientation === "horizontal" && (
        <Group justify="space-between">
          <Group ml={10} gap={"sm"} w={"50%"}>
            <ItemName color="gray.4" size="md" value={transaction} />
          </Group>
          <Group w={100}>
            <Text c="blue.5">{transaction.price} </Text>
          </Group>
          <Group justify="right">
            <Text c="gray.4">{dayjs(transaction.created_at).format("DD/MM/YYYY HH:mm:ss")}</Text>
          </Group>
        </Group>
      )}
      {orientation === "vertical" && (
        <Stack gap={0}>
          <Group ml={10} gap={"sm"} justify="space-between">
            <ItemName color="gray.4" size="md" value={transaction} />
            <Text c="blue.5">{transaction.price}p</Text>
          </Group>
          <Text ml={10} c="gray.4">
            {dayjs(transaction.created_at).format("DD/MM/YYYY HH:mm:ss")}
          </Text>
        </Stack>
      )}
    </Paper>
  );
});

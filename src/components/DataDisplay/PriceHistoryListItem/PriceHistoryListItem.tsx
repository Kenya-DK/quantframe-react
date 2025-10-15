import { Group, Paper, Text } from "@mantine/core";
import classes from "./PriceHistoryListItem.module.css";
import { PriceHistory } from "$types";
import { TimerStamp } from "../../Shared/TimerStamp";
import { ActionWithTooltip } from "../../Shared/ActionWithTooltip";
import { useTranslateCommon } from "../../../hooks/useTranslate.hook";
import { faTrashCan } from "@fortawesome/free-solid-svg-icons";
export type PriceHistoryListItemProps = {
  index?: number;
  history: PriceHistory;
  onDelete?: (index: number) => void;
};

export function PriceHistoryListItem({ history, onDelete, index }: PriceHistoryListItemProps) {
  return (
    <Paper mt={5} classNames={classes} p={5}>
      <Group justify="space-between">
        <Group w={100}>
          <Text c="blue.5">{history.price} </Text>
        </Group>
        <Group justify="right">
          <TimerStamp date={history.created_at} />
          {onDelete && (
            <ActionWithTooltip
              tooltip={useTranslateCommon("buttons.delete.label")}
              color={"red.7"}
              icon={faTrashCan}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={async (e) => {
                e.stopPropagation();
                onDelete(index === undefined ? -1 : index);
              }}
            />
          )}
        </Group>
      </Group>
    </Paper>
  );
}

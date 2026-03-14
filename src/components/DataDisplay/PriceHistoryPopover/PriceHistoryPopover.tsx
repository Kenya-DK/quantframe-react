import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { PriceHistory } from "$types";
import { useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";
import { Group, Popover, Title } from "@mantine/core";
import { faWarframeMarket } from "@icons";
import { useDisclosure } from "@mantine/hooks";
import { PriceHistoryListItem } from "../PriceHistoryListItem";

export const PriceHistoryPopover = ({
  histories,
  status,
  size,
}: {
  histories: PriceHistory[] | undefined;
  status: string | undefined;
  size?: "xs" | "sm" | "lg" | "1x" | "2x" | "3x" | "4x";
}) => {
  const [opened, { close, open }] = useDisclosure(false);

  return (
    <Popover position="bottom" opened={opened}>
      <Popover.Target>
        <FontAwesomeIcon
          onMouseEnter={open}
          onMouseLeave={close}
          size={size || "4x"}
          icon={faWarframeMarket}
          data-stock-status={status || "live"}
          data-color-mode="text"
          style={{ marginRight: "3px" }}
        />
      </Popover.Target>
      <Popover.Dropdown style={{ pointerEvents: "none" }}>
        <Group justify="space-between" mb="sm">
          <Title order={4} mb="sm">
            {useTranslateComponent("price_history_popover.labels.price_history")}
          </Title>
          <Title order={4} mb="sm" data-stock-status={status || "live"} data-color-mode="text">
            {useTranslateEnums(`stock_status.${status || "live"}`)}
          </Title>
        </Group>
        {(histories || [])
          .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
          .slice(0, 5)
          .map((price, index) => (
            <PriceHistoryListItem key={index} history={price} />
          ))}
      </Popover.Dropdown>
    </Popover>
  );
};

import { TauriTypes } from "$types";
import { DisplayPlatinum } from "@components/DataDisplay/DisplayPlatinum";
import { faHandshake } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Badge, Group, Text, Tooltip } from "@mantine/core";

export type ColumnListPriceProps = {
  list_price?: number | null;
  quantity?: number;
  properties?: TauriTypes.StockEntryPropertiesBase;
};

const MAX_PER_TRADE = 6;

const getPerTrade = (quantity: number) => {
  const max = Math.min(Math.max(quantity, 1), MAX_PER_TRADE);
  for (let per_trade = max; per_trade > 1; per_trade--) {
    if (quantity % per_trade === 0) return per_trade;
  }
  return 1;
};

export function ColumnListPrice({ list_price, quantity = 0, properties }: ColumnListPriceProps) {
  const useTranslatePerTrade = (key: string) => useTranslateForms(`settings.tabs.live_scraper.item.wtb.fields.quantity_per_trade.${key}`);

  const is_bulk = Boolean(properties?.is_bulk);
  const per_trade = is_bulk ? getPerTrade(quantity) : undefined;
  const is_batched = per_trade !== undefined && per_trade > 1;

  return (
    <Group gap="xs" wrap="nowrap" align="center">
      {list_price ? (
        <DisplayPlatinum value={list_price} size="md" />
      ) : (
        <Text size="lg" c="dimmed">
          N/A
        </Text>
      )}
      {is_batched && (
        <Tooltip label={useTranslatePerTrade("tooltip")} position="top" withArrow>
          <Badge
            variant="light"
            color="teal"
            size="lg"
            tt="none"
            leftSection={<FontAwesomeIcon icon={faHandshake} />}
            aria-label={useTranslatePerTrade("label")}
          >
            {per_trade}
          </Badge>
        </Tooltip>
      )}
    </Group>
  );
}

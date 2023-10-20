import { Stack } from "@mantine/core";
import { Inventory } from "@components/inventory";
interface StockItemsPanelProps {
}
export const StockItemsPanel = ({ }: StockItemsPanelProps) => {
  return (
    <Stack >
      <Inventory />
    </Stack>)
}
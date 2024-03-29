import { Box, Tabs } from "@mantine/core";
import { GeneralPanel } from "./general.panel";
import { StockItemDto } from "../../../../types";

interface StockItemInfoModalProps {
  item: StockItemDto
}

export function StockItemInfoModal({ item }: StockItemInfoModalProps) {

  return (
    <Tabs defaultValue="general">
      <Tabs.List>
        <Tabs.Tab value="general">{("general.title")}</Tabs.Tab>
      </Tabs.List>
      <Tabs.Panel value="general" pt="xs">
        <Box h={"75vh"} sx={{ position: "relative" }}>
          <GeneralPanel item={item} />
        </Box>
      </Tabs.Panel>
    </Tabs>
  );
}
import { Box, Tabs } from "@mantine/core";
import { GeneralPanel } from "./general.panel";

interface StockRivenInfoModalProps {
}

export function StockRivenInfoModal({ }: StockRivenInfoModalProps) {

  return (
    <Tabs defaultValue="live_scraper">
      <Tabs.List>
        <Tabs.Tab value="general">{("general.title")}</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="general" pt="xs">
        <Box h={"75vh"} sx={{ position: "relative" }}>
          <GeneralPanel />
        </Box>
      </Tabs.Panel>
    </Tabs>
  );
}
import { Tabs, Box } from "@mantine/core";
import { useTranslatePage } from "@hooks/index";
import { AuctionsPanel, OrdersPanel } from "./tabs";

export default function WarframeMarketPage() {
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslatePage(`warframe_market.${key}`, { ...context })

  return (
    <Box p={0} m={0}>
      <Tabs defaultValue="orders">
        <Tabs.List>
          <Tabs.Tab value="orders" >
            {useTranslate('tabs.orders.title')}
          </Tabs.Tab>
          <Tabs.Tab value="auctions">
            {useTranslate('tabs.auctions.title')}
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="orders">
          <OrdersPanel />
        </Tabs.Panel>

        <Tabs.Panel value="auctions">
          <AuctionsPanel />
        </Tabs.Panel>
      </Tabs>
    </Box>
  );
}

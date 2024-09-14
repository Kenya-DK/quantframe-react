import { Container, Tabs } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { OrderPanel } from "./tabs/orders";
import { AuctionPanel } from "./tabs/auctions";


export default function WarframeMarketPage() {

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`warframe_market.${key}`, { ...context }, i18Key)
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`tabs.${key}`, { ...context }, i18Key)

  const tabs = [
    { label: useTranslateTabs("orders.title"), component: <OrderPanel />, id: "or", icon: <div>Stocks</div> },
    { label: useTranslateTabs("auctions.title"), component: <AuctionPanel />, id: "au" },
  ];
  return (
    <Container p={20} size={"100%"}>
      <Tabs defaultValue={tabs[0].id}>
        <Tabs.List>
          {tabs.map((tab) => (
            <Tabs.Tab value={tab.id} key={tab.id}>
              {tab.label}
            </Tabs.Tab>
          ))}
        </Tabs.List>
        {tabs.map((tab) => (
          <Tabs.Panel value={tab.id} key={tab.id}>
            {tab.component}
          </Tabs.Panel>
        ))}
      </Tabs>
    </Container >
  );
}
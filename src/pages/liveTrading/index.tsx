import { Box, Container, Tabs } from "@mantine/core";
import { StockItemPanel } from "./tabs/item";
import { StockRivenPanel } from "./tabs/riven";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { LiveTradingControl } from "@components/LiveTradingControl";

export default function LiveTradingPage() {

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`liveTrading.${key}`, { ...context }, i18Key)
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`tabs.${key}`, { ...context }, i18Key)

  const tabs = [
    { label: useTranslateTabs("item.title"), component: <StockItemPanel />, id: "item", icon: <div>Stocks</div> },
    { label: useTranslateTabs("riven.title"), component: <StockRivenPanel />, id: "riven" },
  ];
  return (
    <Container size={"100%"}>
      <Box mt={25}>
        <LiveTradingControl />
      </Box>
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
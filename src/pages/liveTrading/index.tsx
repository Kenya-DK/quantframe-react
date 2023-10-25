import { Grid, Tabs } from "@mantine/core";
import { TransactionControl } from "../../components/transactionControl";
import { StockItemsPanel, StockRivenPanel } from "./tabs";
import { useTranslatePage } from "../../hooks";

export default function LiveTradingPage() {
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslatePage(`live_trading.${key}`, { ...context })
  return (
    <Grid>
      <Grid.Col md={12}>
        <TransactionControl />
        <Tabs defaultValue="items">
          <Tabs.List>
            <Tabs.Tab value="items" >
              {useTranslate('tabs.item.title')}
            </Tabs.Tab>
            <Tabs.Tab value="rivens">
              {useTranslate('tabs.riven.title')}
            </Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value="items">
            <StockItemsPanel />
          </Tabs.Panel>
          <Tabs.Panel value="rivens">
            <StockRivenPanel />
          </Tabs.Panel>
        </Tabs>
      </Grid.Col>
    </Grid>
  );
}

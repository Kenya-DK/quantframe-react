import { Tabs } from "@mantine/core";
import { Transactions } from "./transactions";
import { Logging } from "./logging";
export default function DebugPage() {

  return (
    <Tabs defaultValue="warframe_algo_trader">
      <Tabs.List>
        <Tabs.Tab value="logging">Logging</Tabs.Tab>
        <Tabs.Tab value="transactions">Transactions</Tabs.Tab>
        <Tabs.Tab value="trades">Trades</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="logging">
        <Logging />
      </Tabs.Panel>
      <Tabs.Panel value="transactions">
        <Transactions />
      </Tabs.Panel>
    </Tabs>
  );
}

import { Tabs } from "@mantine/core";
import { ImportAlgoTrader } from "./importAlgoTrader";
import { ResetData } from "./resetData";
import { Transactions } from "./transactions";
import { Logging } from "./logging";
import { Trades } from "./trades";
export default function DebugPage() {

  return (
    <Tabs defaultValue="warframe_algo_trader">
      <Tabs.List>
        <Tabs.Tab value="logging">Logging</Tabs.Tab>
        <Tabs.Tab value="warframe_algo_trader">Warframe Algo Trader</Tabs.Tab>
        <Tabs.Tab value="reset_data">Reset Data</Tabs.Tab>
        <Tabs.Tab value="transactions">Transactions</Tabs.Tab>
        <Tabs.Tab value="trades">Trades</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="logging">
        <Logging />
      </Tabs.Panel>
      <Tabs.Panel value="warframe_algo_trader">
        <ImportAlgoTrader />
      </Tabs.Panel>
      <Tabs.Panel value="reset_data">
        <ResetData />
      </Tabs.Panel>
      <Tabs.Panel value="transactions">
        <Transactions />
      </Tabs.Panel>
      <Tabs.Panel value="trades">
        <Trades />
      </Tabs.Panel>
    </Tabs>
  );
}

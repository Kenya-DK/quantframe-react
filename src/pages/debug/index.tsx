import { Tabs } from "@mantine/core";
import { ImportAlgoTrader } from "./importAlgoTrader";
import { ResetData } from "./resetData";
export default function DebugPage() {

  return (
    <Tabs defaultValue="warframe_algo_trader">
      <Tabs.List>
        <Tabs.Tab value="warframe_algo_trader"> Warframe Algo Trader</Tabs.Tab>
        <Tabs.Tab value="reset_data"> Reset Data</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="warframe_algo_trader">
        <ImportAlgoTrader />
      </Tabs.Panel>
      <Tabs.Panel value="reset_data">
        <ResetData />
      </Tabs.Panel>
    </Tabs>
  );
}

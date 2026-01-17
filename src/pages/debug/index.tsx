import { Container, Tabs } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
// import { StatisticProfitBase, TransactionType } from "@api/types";
import { TransactionPanel } from "./tabs/transactions";
import { DataBasePanel } from "./tabs/database";


export default function DebugPage() {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`debug.${key}`, { ...context }, i18Key)
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`tabs.${key}`, { ...context }, i18Key)

  const tabs = [
    { label: useTranslateTabs("transaction.title"), component: <TransactionPanel />, id: "tr", icon: <div>Stocks</div> },
    { label: useTranslateTabs("database.title"), component: <DataBasePanel />, id: "db" },
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
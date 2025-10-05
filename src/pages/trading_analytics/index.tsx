import { Tabs } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { TransactionPanel, ItemPanel, RivenPanel, UserPanel } from "./Tabs";
import { useLocalStorage } from "@mantine/hooks";
import classes from "./TradingAnalytics.module.css";
import { useHasAlert } from "@hooks/useHasAlert.hook";

export default function TradingAnalyticsPage() {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("transaction.title"),
      component: (isActive: boolean) => <TransactionPanel isActive={isActive} />,
      id: "transaction",
    },
    {
      label: useTranslateTabs("item.title"),
      component: (isActive: boolean) => <ItemPanel isActive={isActive} />,
      id: "item",
    },
    {
      label: useTranslateTabs("riven.title"),
      component: (isActive: boolean) => <RivenPanel isActive={isActive} />,
      id: "riven",
    },
    {
      label: useTranslateTabs("user.title"),
      component: (isActive: boolean) => <UserPanel isActive={isActive} />,
      id: "user",
    },
  ];

  const [activeTab, setActiveTab] = useLocalStorage<string>({
    key: "trading_analytics_active_tab",
    defaultValue: tabs[0].id,
  });

  return (
    <Tabs value={activeTab} onChange={(value) => setActiveTab(value || tabs[0].id)} data-has-alert={useHasAlert()} className={classes.tabs}>
      <Tabs.List>
        {tabs.map((tab) => (
          <Tabs.Tab value={tab.id} key={tab.id}>
            {tab.label}
          </Tabs.Tab>
        ))}
      </Tabs.List>
      {tabs.map((tab) => (
        <Tabs.Panel value={tab.id} key={tab.id}>
          {tab.component(activeTab === tab.id)}
        </Tabs.Panel>
      ))}
    </Tabs>
  );
}

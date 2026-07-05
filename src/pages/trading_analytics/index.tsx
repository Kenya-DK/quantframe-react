import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { Tabs } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { useMemo } from "react";
import { ItemPanel, RivenPanel, SyndicatePanel, TransactionPanel, UserPanel, WarframeGDPRParser } from "./Tabs";
import classes from "./TradingAnalytics.module.css";

export default function TradingAnalyticsPage() {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = useMemo(
    () => [
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
        label: useTranslateTabs("syndicate.title"),
        component: (isActive: boolean) => <SyndicatePanel isActive={isActive} />,
        id: "syndicate",
        isPremium: true,
      },
      {
        label: useTranslateTabs("riven.title"),
        component: (isActive: boolean) => <RivenPanel isActive={isActive} />,
        id: "riven",
        isPremium: true,
      },
      {
        label: useTranslateTabs("user.title"),
        component: (isActive: boolean) => <UserPanel isActive={isActive} />,
        id: "user",
        isPremium: true,
      },
      {
        label: useTranslateTabs("wfgdpr.title"),
        component: () => <WarframeGDPRParser />,
        id: "wfgdpr",
        isPremium: false,
      },
    ],
    [],
  );

  const [activeTab, setActiveTab] = useLocalStorage<string>({
    key: "trading_analytics_active_tab",
    defaultValue: tabs[0].id,
  });

  return (
    <Tabs value={activeTab} onChange={(value) => setActiveTab(value || tabs[0].id)} data-has-alert={useHasAlert()} className={classes.tabs}>
      <Tabs.List>
        {tabs.map((tab) => (
          <Tabs.Tab value={tab.id} key={tab.id} rightSection={tab.isPremium ? "👑" : undefined}>
            {tab.label}
          </Tabs.Tab>
        ))}
      </Tabs.List>
      {tabs.map((tab) => (
        <Tabs.Panel value={tab.id} key={tab.id}>
          {activeTab === tab.id && tab.component(true)}
        </Tabs.Panel>
      ))}
    </Tabs>
  );
}

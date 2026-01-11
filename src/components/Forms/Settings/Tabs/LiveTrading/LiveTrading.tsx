import { Tabs } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { RivenPanel } from "./Tabs/Riven";
import { ItemPanel } from "./Tabs/Item";
import { useState } from "react";
import { GeneralPanel } from "./Tabs/General";
import { UseFormReturnType } from "@mantine/form";

export type LiveTradingPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const LiveTradingPanel = ({ form }: LiveTradingPanelProps) => {
  const [hideTab, setHideTab] = useState<boolean>(false);

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("live_scraper.general.title"),
      component: <GeneralPanel form={form} setHideTab={(v) => setHideTab(v)} />,
      id: "general",
    },
    {
      label: useTranslateTabs("live_scraper.item.title"),
      component: <ItemPanel form={form} />,
      id: "item",
    },
    {
      label: useTranslateTabs("live_scraper.riven.title"),
      component: <RivenPanel form={form} />,
      id: "riven",
    },
  ];

  const [activeTab, setActiveTab] = useState<string>(tabs[0].id);

  return (
    <Tabs h={"82vh"} orientation="vertical" value={activeTab} onChange={(value) => setActiveTab(value || tabs[0].id)}>
      <Tabs.List display={hideTab ? "none" : ""}>
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
  );
};

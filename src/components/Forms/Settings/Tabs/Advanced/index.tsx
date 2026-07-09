import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Tabs } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { useState } from "react";
import { HttpServerPanel } from "./Tabs/HttpServer";
import { LogPanel } from "./Tabs/Log";

export type AdvancedPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const AdvancedPanel = ({ form }: AdvancedPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.advanced.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateForm("http_server.title"),
      component: <HttpServerPanel form={form} />,
      id: "http_server",
    },
    {
      label: useTranslateForm("log.title"),
      component: <LogPanel form={form} />,
      id: "log",
    },
  ];

  const [activeTab, setActiveTab] = useState<string>(tabs[0].id);

  return (
    <Tabs h={"82vh"} orientation="vertical" value={activeTab} onChange={(value) => setActiveTab(value || tabs[0].id)}>
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
  );
};


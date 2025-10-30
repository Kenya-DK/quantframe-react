import { Tabs } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useState } from "react";
import { GeneralPanel } from "./Tabs/General";

export type HttpServerPanelProps = {
  value: TauriTypes.HttpServerSettings;
  onSubmit: (value: TauriTypes.HttpServerSettings) => void;
};

export const HttpServerPanel = ({ onSubmit, value }: HttpServerPanelProps) => {
  const [hideTab, setHideTab] = useState<boolean>(false);

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("http_server.general.title"),
      component: (
        <GeneralPanel
          value={value}
          setHideTab={(v) => setHideTab(v)}
          onSubmit={(v) => {
            onSubmit({ ...value, ...v });
          }}
        />
      ),
      id: "general",
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

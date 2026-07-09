import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Tabs } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { useState } from "react";
import { DashboardPanel } from "./Tabs/Dashboard";
import { ThemePanel } from "./Tabs/Theme";

export type AppearancePanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
  onHideButtons?: (value: boolean) => void;
};

export const AppearancePanel = ({ form, onHideButtons }: AppearancePanelProps) => {
  const [hideTab, setHideTab] = useState<boolean>(false);
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.appearance.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateForm("dashboard.title"),
      component: <DashboardPanel form={form} setHideTab={(v) => setHideTab(v)} setHideButtons={(v) => onHideButtons?.(v)} />,
      id: "dashboard",
    },
    {
      label: useTranslateForm("theme.title"),
      component: <ThemePanel value={form.values} onSubmit={(values) => form.setValues(values)} />,
      id: "theme",
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

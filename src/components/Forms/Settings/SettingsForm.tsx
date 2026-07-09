import { TauriTypes } from "$types";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { Button, Container, Group, Tabs } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useLocalStorage } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { AdvancedPanel } from "./Tabs/Advanced";
import { AppearancePanel } from "./Tabs/Appearance";
import { GeneralPanel } from "./Tabs/General";
import { LiveTradingPanel } from "./Tabs/LiveTrading";
import { NotificationsPanel } from "./Tabs/Notifications";

export type SettingsFormProps = {
  value: TauriTypes.Settings & { has_error?: boolean; hide_save_button?: boolean };
  onSubmit: (value: TauriTypes.Settings) => void | Promise<void>;
};

export function SettingsForm({ onSubmit, value }: SettingsFormProps) {
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const form = useForm({
    initialValues: value,
    validate: {
      has_error: (value: boolean | undefined) => {
        if (value == undefined) return null;
        return value ? "Form has errors" : null;
      },
    },
    onValuesChange: (d) => {
      console.log(d);
    },
  });

  const [hideButtons, setHideButtons] = useState<boolean>(false);
  const [showButtons, setShowButtons] = useState<boolean>(false);

  useEffect(() => {
    setShowButtons(form.isDirty() && !hideButtons);
  }, [form, setHideButtons]);

  const tabs = [
    {
      label: useTranslateTabs("general.title"),
      component: <GeneralPanel form={form} />,
      id: "general",
    },
    {
      label: useTranslateTabs("live_scraper.title"),
      component: <LiveTradingPanel form={form} onHideButtons={(v) => setHideButtons(v)} />,
      id: "live_scraper",
    },
    {
      label: useTranslateTabs("appearance.title"),
      component: <AppearancePanel form={form} onHideButtons={(v) => setHideButtons(v)} />,
      id: "appearance",
    },
    {
      label: useTranslateTabs("notifications.title"),
      component: <NotificationsPanel form={form} onHideButtons={(v) => setHideButtons(v)} />,
      id: "notifications",
    },
    {
      label: useTranslateTabs("advanced.title"),
      component: <AdvancedPanel form={form} />,
      id: "advanced",
    },
  ];

  const [activeTab, setActiveTab] = useLocalStorage<string>({
    key: "settings.activeTab",
    defaultValue: tabs[0].id,
  });

  return (
    <Container size={"100%"} h={"85vh"} p={0} pos="relative">
      <Tabs value={activeTab} onChange={(value) => setActiveTab(value || tabs[0].id)}>
        <Tabs.List>
          {tabs.map((tab) => (
            <Tabs.Tab value={tab.id} key={tab.id} disabled={hideButtons}>
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
      <Group justify="flex-end" mt="md" display={showButtons ? "" : "none"}>
        <Button pos="absolute" bottom={10} right={10} onClick={() => onSubmit(form.values)} color="green">
          {useTranslateCommon("buttons.save.label")}
        </Button>
      </Group>
    </Container>
  );
}

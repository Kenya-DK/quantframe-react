import { PaperProps, Container, Tabs, Button, Group, Text } from "@mantine/core";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { LiveTradingPanel } from "./Tabs/LiveTrading";
import { useLocalStorage } from "@mantine/hooks";
import { ThemesPanel } from "./Tabs/Themes";
import { NotificationsPanel } from "./Tabs/Notifications";
import { AdvancedPanel } from "./Tabs/Advanced";
import { SummaryPanel } from "./Tabs/Summary";
import { HttpServerPanel } from "./Tabs/HttpServer";
import { GeneralPanel } from "./Tabs/General";
import { useForm } from "@mantine/form";
import api from "@api/index";
import { useState, useEffect } from "react";
import { modals } from "@mantine/modals";

export type SettingsFormProps = {
  value: TauriTypes.Settings & { has_error?: boolean; hide_save_button?: boolean };
  onSubmit: (value: TauriTypes.Settings) => void | Promise<void>;
  paperProps?: PaperProps;
};

export function SettingsForm({ onSubmit, value }: SettingsFormProps) {
  // Translate general
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
  });

  const [defaultSettings, setDefaultSettings] = useState<TauriTypes.Settings | null>(null);

  useEffect(() => {
    api.app.getDefaultSettings().then(setDefaultSettings).catch(console.error);
  }, []);

  const isNotDefault =
    defaultSettings &&
    JSON.stringify({ ...value, has_error: undefined, hide_save_button: undefined }) !== JSON.stringify(defaultSettings);

  const showButtons = isNotDefault || form.isDirty();

  const resetSettingsDialogTitle = useTranslateCommon("dialogs.reset_settings.title");
  const resetSettingsDialogMessage = useTranslateCommon("dialogs.reset_settings.message");
  const resetDefaultsLabel = useTranslateCommon("buttons.reset_defaults.label");
  const cancelLabel = useTranslateCommon("buttons.cancel.label");
  const saveLabel = useTranslateCommon("buttons.save.label");

  const handleReset = () => {
    modals.openConfirmModal({
      title: resetSettingsDialogTitle,
      children: <Text size="sm">{resetSettingsDialogMessage}</Text>,
      labels: { confirm: resetDefaultsLabel, cancel: cancelLabel },
      confirmProps: { color: "red" },
      onConfirm: async () => {
        try {
          const defaults = await api.app.getDefaultSettings();
          form.setValues(defaults);
          await onSubmit(defaults);
        } catch (e) {
          console.error("Failed to reset to defaults", e);
        }
      },
    });
  };

  const tabs = [
    {
      label: useTranslateTabs("general.title"),
      component: <GeneralPanel form={form} />,
      id: "general",
    },
    {
      label: useTranslateTabs("live_scraper.title"),
      component: <LiveTradingPanel form={form} />,
      id: "live_scraper",
    },
    {
      label: useTranslateTabs("themes.title"),
      component: <ThemesPanel value={value} onSubmit={(v) => onSubmit({ ...value, ...v })} />,
      id: "themes",
    },
    {
      label: useTranslateTabs("notifications.title"),
      component: <NotificationsPanel form={form} />,
      id: "notifications",
    },
    {
      label: useTranslateTabs("advanced.title"),
      component: <AdvancedPanel form={form} />,
      id: "advanced",
    },
    {
      label: useTranslateTabs("summary.title"),
      component: <SummaryPanel form={form} />,
      id: "summary",
    },
    {
      label: useTranslateTabs("http_server.title"),
      component: <HttpServerPanel form={form} />,
      id: "http_server",
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
      <Group justify="flex-end" mt="md" display={showButtons ? "" : "none"}>
        {isNotDefault && (
          <Button pos="absolute" bottom={10} right={130} onClick={handleReset} color="red" variant="outline">
            {resetDefaultsLabel}
          </Button>
        )}
        <Button pos="absolute" bottom={10} right={10} onClick={() => onSubmit(form.values)} color="green">
          {saveLabel}
        </Button>
      </Group>
    </Container>
  );
}

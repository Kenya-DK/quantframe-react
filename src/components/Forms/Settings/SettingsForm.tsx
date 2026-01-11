import { PaperProps, Container, Tabs, Button, Group } from "@mantine/core";
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

export type SettingsFormProps = {
  value: TauriTypes.Settings & { has_error?: boolean; hide_save_button?: boolean };
  onSubmit: (value: TauriTypes.Settings) => void;
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
      has_error: (value) => {
        if (value == undefined) return null;
        return value ? "Form has errors" : null;
      },
    },
  });

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
      <Group justify="flex-end" mt="md" display={form.isDirty() ? "" : "none"}>
        <Button pos="absolute" bottom={10} right={10} onClick={() => onSubmit(form.values)} color="green">
          {useTranslateCommon("buttons.save.label")}
        </Button>
      </Group>
    </Container>
  );
}

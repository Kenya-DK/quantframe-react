import { PaperProps, Container, Tabs } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { AdvancedPanel } from "./Tabs/Advanced";
import { NotificationsPanel } from "./Tabs/Notifications";
import { SummaryPanel } from "./Tabs/Summary";
import { ThemesPanel } from "./Tabs/Themes";
import { LiveTradingPanel } from "./Tabs/LiveTrading";
import { useLocalStorage } from "@mantine/hooks";

export type SettingsFormProps = {
  value: TauriTypes.Settings;
  onSubmit: (value: TauriTypes.Settings) => void;
  paperProps?: PaperProps;
};

export function SettingsForm({ onSubmit, value }: SettingsFormProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("live_trading.title"),
      component: <LiveTradingPanel value={value.live_scraper} onSubmit={(v) => onSubmit({ ...value, live_scraper: v })} />,
      id: "live_trading",
    },
    {
      label: useTranslateTabs("themes.title"),
      component: <ThemesPanel value={value} onSubmit={(v) => onSubmit({ ...value, ...v })} />,
      id: "themes",
    },
    {
      label: useTranslateTabs("notifications.title"),
      component: <NotificationsPanel value={value.notifications} onSubmit={(v) => onSubmit({ ...value, notifications: v })} />,
      id: "notifications",
    },
    {
      label: useTranslateTabs("advanced.title"),
      component: <AdvancedPanel value={value.advanced_settings} onSubmit={(v) => onSubmit({ ...value, advanced_settings: v })} />,
      id: "advanced",
    },
    {
      label: useTranslateTabs("summary.title"),
      component: <SummaryPanel value={value.summary_settings} onSubmit={(v) => onSubmit({ ...value, summary_settings: v })} />,
      id: "summary",
    },
  ];

  const [activeTab, setActiveTab] = useLocalStorage<string>({
    key: "settings.activeTab",
    defaultValue: tabs[0].id,
  });

  return (
    <Container size={"100%"} h={"85vh"} p={0}>
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
    </Container>
  );
}

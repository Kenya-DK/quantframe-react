import { PaperProps, Container, Tabs } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { GeneralPanel } from "./Tabs/General";
import { LogPanel } from "./Tabs/Log";
import { LiveTradingPanel } from "./Tabs/LiveTrading";
import { NotificationPanel } from "./Tabs/Notification";
import { AnalyticPanel } from "./Tabs/Analytic";
import { SummaryPanel } from "./Tabs/Summary/Summary";
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
      label: useTranslateTabs("general.title"),
      component: (
        <GeneralPanel
          value={value}
          onSubmit={(v) => {
            onSubmit({ ...value, wf_log_path: v.wf_log_path });
          }}
        />
      ),
      id: "general",
    },
    {
      label: useTranslateTabs("live_trading.title"),
      component: (
        <LiveTradingPanel
          value={value.live_scraper}
          onSubmit={(v) => {
            onSubmit({ ...value, live_scraper: v });
          }}
        />
      ),
      id: "live_trading",
    },
    {
      label: useTranslateTabs("notification.title"),
      component: (
        <NotificationPanel
          value={value.notifications}
          onSubmit={(v) => {
            onSubmit({ ...value, notifications: v });
          }}
        />
      ),
      id: "notification",
    },
    {
      label: useTranslateTabs("analytics.title"),
      component: (
        <AnalyticPanel
          value={value.analytics}
          onSubmit={(v) => {
            onSubmit({ ...value, analytics: v });
          }}
        />
      ),
      id: "analytics",
    },
    {
      label: useTranslateTabs("summary.title"),
      component: (
        <SummaryPanel
          value={value.summary_settings}
          onSubmit={(v) => {
            onSubmit({ ...value, summary_settings: v });
          }}
        />
      ),
      id: "summary",
    },
    {
      label: useTranslateTabs("log.title"),
      component: <LogPanel />,
      id: "log",
    },
  ];

  const [activeTab, setActiveTab] = useLocalStorage<string>({
    key: "settings.activeTab",
    defaultValue: tabs[1].id,
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

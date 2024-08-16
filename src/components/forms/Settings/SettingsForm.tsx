import { PaperProps, Container, Tabs } from '@mantine/core';
import { useTranslateForms } from '@hooks/useTranslate.hook';
import { Settings } from '@api/types';
import { GeneralPanel } from './Tabs/General';
import { LogPanel } from './Tabs/Log';
import { LiveTradingPanel } from './Tabs/LiveTrading';
import { NotificationPanel } from './Tabs/Notification';
import { AnalyticPanel } from './Tabs/Analytic';

export type SettingsFormProps = {
  value: Settings
  onSubmit: (value: Settings) => void;
  paperProps?: PaperProps;
}


export function SettingsForm({ onSubmit, value }: SettingsFormProps) {

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`settings.${key}`, { ...context }, i18Key)
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`tabs.${key}`, { ...context }, i18Key)

  const tabs = [
    { label: useTranslateTabs("general.title"), component: <GeneralPanel />, id: "general" },
    {
      label: useTranslateTabs("live_trading.title"), component: <LiveTradingPanel value={value.live_scraper} onSubmit={(v) => {
        onSubmit({ ...value, live_scraper: v })
      }} />, id: "live_trading"
    },
    {
      label: useTranslateTabs("notification.title"), component: <NotificationPanel value={value.notifications} onSubmit={(v) => {
        onSubmit({ ...value, notifications: v })
      }} />, id: "notification"
    },
    {
      label: useTranslateTabs("analytic.title"), component: <AnalyticPanel value={value.analytics} onSubmit={(v) => {
        onSubmit({ ...value, analytics: v })
      }} />, id: "notification"
    },
    {
      label: useTranslateTabs("log.title"), component: <LogPanel />, id: "log"
    },

  ];
  return (
    <Container size={"100%"} h={"85vh"} p={0}>
      <Tabs defaultValue={tabs[1].id}>
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
    </Container >
  );
}
import { PaperProps, Container, Tabs } from '@mantine/core';
import { useTranslateForms } from '@hooks/index';
import { Settings } from '@api/types';
import { GeneralPanel } from './Tabs/General';
import { LiveTradingPanel } from './Tabs/LiveTrading';
import { NotificationPanel } from './Tabs/Notification';

export type SettingsFormProps = {
  value: Settings
  onSubmit: (values: { email: string; password: string }) => void;
  paperProps?: PaperProps;
}


export function SettingsForm({ value }: SettingsFormProps) {


  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`settings.${key}`, { ...context }, i18Key)
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`tabs.${key}`, { ...context }, i18Key)

  const tabs = [
    { label: useTranslateTabs("general.title"), component: <GeneralPanel />, id: "general" },
    { label: useTranslateTabs("live_trading.title"), component: <LiveTradingPanel value={value.live_scraper} onSubmit={() => { }} />, id: "live_trading" },
    { label: useTranslateTabs("notification.title"), component: <NotificationPanel value={value.notifications} onSubmit={() => { }} />, id: "notification" },

  ];
  return (
    <Container size={"100%"}>
      <Tabs defaultValue={tabs[0].id}>
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
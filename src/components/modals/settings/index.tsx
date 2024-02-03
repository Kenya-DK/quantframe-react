import { Box, Tabs } from "@mantine/core";
import { GeneralPanel } from "./general.panel";
import { LiveScraperPanel } from "./liveScraper.panel";
import { useTranslateModal } from "@hooks/index";
import { DeepPartial, Settings, Wfm } from "$types/index";
import { NotificationsPanel } from "./notifications.panel";
import { useState } from "react";
import { modals } from "@mantine/modals";
import { LoggingPanel } from "./logging.panel";

interface SettingsModalProps {
  settings: Settings | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: DeepPartial<Settings>) => void;
}

export function SettingsModal({ tradable_items, settings: settingsIn, updateSettings }: SettingsModalProps) {
  const useTranslateSettingsPanels = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.${key}`, { ...context })
  const [settings, setSettings] = useState<Settings | undefined>(settingsIn);

  const handleUpdateSettings = async (settingsData: DeepPartial<Settings>) => {
    if (!settings) return;
    const data = { ...settings, ...settingsData } as Settings;
    setSettings((a) => a = { ...a, ...data });
    updateSettings(data);
    modals.closeAll();
  }
  return (
    <Tabs defaultValue="live_scraper">
      <Tabs.List>
        <Tabs.Tab value="general">{useTranslateSettingsPanels("general.title")}</Tabs.Tab>
        <Tabs.Tab value="live_scraper">{useTranslateSettingsPanels("live_trading.title")}</Tabs.Tab>
        <Tabs.Tab value="notifications">{useTranslateSettingsPanels("notifications.title")}</Tabs.Tab>
        <Tabs.Tab value="logging">{useTranslateSettingsPanels("logging.title")}</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="general" pt="xs">
        <Box h={"75vh"} sx={{ position: "relative" }}>
          <GeneralPanel settings={settings} updateSettings={updateSettings} />
        </Box>
      </Tabs.Panel>

      <Tabs.Panel value="live_scraper" pt="xs">
        <Box h={"75vh"} sx={{ position: "relative" }}>
          <LiveScraperPanel settings={settings?.live_scraper} updateSettings={(set) => {
            handleUpdateSettings({ live_scraper: set })
          }} tradable_items={tradable_items} />
        </Box>
      </Tabs.Panel>
      <Tabs.Panel value="logging" pt="xs">
        <Box h={"75vh"} sx={{ position: "relative" }}>
          <LoggingPanel settings={settings} updateSettings={(set) => {
            handleUpdateSettings({ ...set })
          }} />
        </Box>
      </Tabs.Panel>
      <Tabs.Panel value="notifications" pt="xs">
        <Box h={"75vh"} sx={{ position: "relative" }}>
          <NotificationsPanel settings={settings?.notifications} updateSettings={(set) => {
            handleUpdateSettings({ notifications: set })
          }} tradable_items={tradable_items} />
        </Box>
      </Tabs.Panel>
    </Tabs>
  );
}
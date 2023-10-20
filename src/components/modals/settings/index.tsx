import { Box, Tabs } from "@mantine/core";
import { GeneralPanel } from "./general.panel";
import { LiveScraperPanel } from "./liveScraper.panel";
import { useTranslateModal } from "@hooks/index";
import { DeepPartial, Settings, Wfm } from "$types/index";
import { WhisperScraperPanel } from "./whisperScraper.panel";
import { useState } from "react";
import { modals } from "@mantine/modals";

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
        <Tabs.Tab value="whisper_scraper">{useTranslateSettingsPanels("whisper_scraper.title")}</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="general" pt="xs">
        <Box h={"75vh"} w={"75vw"} sx={{ position: "relative" }}>
          <GeneralPanel settings={settings} updateSettings={updateSettings} />
        </Box>
      </Tabs.Panel>

      <Tabs.Panel value="live_scraper" pt="xs">
        <Box h={"75vh"} w={"75vw"} sx={{ position: "relative" }}>
          <LiveScraperPanel settings={settings?.live_scraper} updateSettings={(set) => {
            handleUpdateSettings({ live_scraper: set })
          }} tradable_items={tradable_items} />
        </Box>
      </Tabs.Panel>
      <Tabs.Panel value="whisper_scraper" pt="xs">
        <Box h={"75vh"} w={"75vw"} sx={{ position: "relative" }}>
          <WhisperScraperPanel settings={settings?.whisper_scraper} updateSettings={(set) => {
            handleUpdateSettings({ whisper_scraper: set })
          }} tradable_items={tradable_items} />
        </Box>
      </Tabs.Panel>
    </Tabs>
  );
}
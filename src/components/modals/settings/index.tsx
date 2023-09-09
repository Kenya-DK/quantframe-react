import { Tabs } from "@mantine/core";
import { GeneralPanel } from "./general.panel";
import { LiveScraperPanel } from "./liveScraper.panel";
import { useTranslateModal } from "@hooks/index";
import { DeepPartial, Settings, Wfm } from "$types/index";
import { WhisperScraperPanel } from "./whisperScraper.panel";
import { useEffect } from "react";

interface SettingsModalProps {
  settings: Settings | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: DeepPartial<Settings>) => void;
}

export function SettingsModal({ tradable_items, settings, updateSettings }: SettingsModalProps) {
  const useTranslateSettingsPanels = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.${key}`, { ...context })

  useEffect(() => {
    if (!settings) return;
    console.log(settings);
  }, [settings]);
  return (
    <Tabs defaultValue="live_scraper">
      {JSON.stringify(settings, null, 0)}
      <Tabs.List>
        <Tabs.Tab value="general">{useTranslateSettingsPanels("general.title")}</Tabs.Tab>
        <Tabs.Tab value="live_scraper">{useTranslateSettingsPanels("live_trading.title")}</Tabs.Tab>
        <Tabs.Tab value="whisper_scraper">{useTranslateSettingsPanels("whisper_scraper.title")}</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="general" pt="xs">
        <GeneralPanel settings={settings} updateSettings={updateSettings} />
      </Tabs.Panel>

      <Tabs.Panel value="live_scraper" pt="xs">
        <LiveScraperPanel settings={settings?.live_scraper} updateSettings={(set) => {
          console.log("updateSettingslive_scraper", set, settings?.whisper_scraper)
          updateSettings({ live_scraper: set })
        }} tradable_items={tradable_items} />
      </Tabs.Panel>
      <Tabs.Panel value="whisper_scraper" pt="xs">
        <WhisperScraperPanel settings={settings?.whisper_scraper} updateSettings={(set) => {
          console.log("updateSettingswhisper_scraper", set, settings?.live_scraper)
          updateSettings({ whisper_scraper: set })
        }} tradable_items={tradable_items} />
      </Tabs.Panel>
    </Tabs>


  );
}
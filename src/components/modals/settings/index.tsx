import { Tabs } from "@mantine/core";
import { GeneralPanel } from "./general.panel";
import { LiveScraperPanel } from "./liveScraper.panel";
import { useTranslateModal } from "@hooks/index";
import { Settings, Wfm } from "$types/index";

interface SettingsModalProps {
  settings: Settings | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: Partial<Settings>) => void;
}

export function SettingsModal({ tradable_items, settings, updateSettings }: SettingsModalProps) {
  const useTranslateSettingsPanels = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.${key}`, { ...context })
  return (
    <Tabs defaultValue="live_scraper">
      <Tabs.List>
        <Tabs.Tab value="general">{useTranslateSettingsPanels("general.title")}</Tabs.Tab>
        <Tabs.Tab value="live_scraper">{useTranslateSettingsPanels("live_trading.title")}</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="general" pt="xs">
        <GeneralPanel settings={settings} updateSettings={updateSettings} />
      </Tabs.Panel>

      <Tabs.Panel value="live_scraper" pt="xs">
        <LiveScraperPanel settings={settings} updateSettings={updateSettings} tradable_items={tradable_items} />
      </Tabs.Panel>
    </Tabs>


  );
}
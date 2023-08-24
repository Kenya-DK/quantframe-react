import { Tabs } from "@mantine/core";
import { GeneralPanel } from "./general.panel";
import { LiveScraperPanel } from "./liveScraper.panel";
import { useTranslateModal } from "@hooks/index";

export function SettingsModal() {
  const useTranslateSettingsPanels = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.${key}`, { ...context })
  return (
    <Tabs defaultValue="gallery">
      <Tabs.List>
        <Tabs.Tab value="general">{useTranslateSettingsPanels("general.title")}</Tabs.Tab>
        <Tabs.Tab value="live_scraper">{useTranslateSettingsPanels("live_trading.title")}</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="general" pt="xs">
        <GeneralPanel />
      </Tabs.Panel>

      <Tabs.Panel value="live_scraper" pt="xs">
        <LiveScraperPanel />
      </Tabs.Panel>
    </Tabs>


  );
}
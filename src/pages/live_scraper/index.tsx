import { LiveScraperControl } from "@components/Forms/LiveScraperControl";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { Box, Container, Tabs } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import classes from "./LiveScraper.module.css";
import { ItemPanel, RivenPanel, SyndicatePanel, WishListPanel } from "./Tabs";

export default function LiveScraperPage() {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`live_scraper.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("item.title"),
      component: (isActive: boolean) => <ItemPanel isActive={isActive} />,
      id: "item",
    },
    {
      label: useTranslateTabs("riven.title"),
      component: (isActive: boolean) => <RivenPanel isActive={isActive} />,
      id: "riven",
    },
    {
      label: useTranslateTabs("wish_list.title"),
      component: (isActive: boolean) => <WishListPanel isActive={isActive} />,
      id: "wish_list",
    },
    {
      label: useTranslateTabs("syndicate.title"),
      component: (isActive: boolean) => <SyndicatePanel isActive={isActive} />,
      hide: !import.meta.env.DEV,
      id: "syndicate",
    },
  ];

  const [activeTab, setActiveTab] = useLocalStorage<string>({
    key: "live_scraper.active_tab",
    defaultValue: tabs[0].id,
  });

  return (
    <Container size={"100%"}>
      <Box data-has-alert={useHasAlert()} className={classes.liveScraper}>
        <LiveScraperControl />
      </Box>
      <Tabs value={activeTab} onChange={(value) => setActiveTab(value || tabs[0].id)}>
        <Tabs.List>
          {tabs
            .filter((tab) => !tab.hide)
            .map((tab) => (
              <Tabs.Tab value={tab.id} key={tab.id}>
                {tab.label}
              </Tabs.Tab>
            ))}
        </Tabs.List>
        {tabs.map((tab) => (
          <Tabs.Panel value={tab.id} key={tab.id}>
            {activeTab === tab.id && tab.component(true)}
          </Tabs.Panel>
        ))}
      </Tabs>
    </Container>
  );
}

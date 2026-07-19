import { useMemo } from "react";
import { Box, Container, Tabs } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { ItemPanel, RivenPanel, WishListPanel } from "./Tabs";
import { useLocalStorage } from "@mantine/hooks";
import { LiveScraperControl } from "@components/Forms/LiveScraperControl";
import classes from "./LiveScraper.module.css";
import { useHasAlert } from "@hooks/useHasAlert.hook";

export default function LiveScraperPage() {
  const hasAlert = useHasAlert();
  const itemTitle = useTranslatePages("live_scraper.tabs.item.title");
  const rivenTitle = useTranslatePages("live_scraper.tabs.riven.title");
  const wishListTitle = useTranslatePages("live_scraper.tabs.wish_list.title");

  const tabs = useMemo(() => [
    { label: itemTitle, component: (isActive: boolean) => <ItemPanel isActive={isActive} />, id: "item" },
    { label: rivenTitle, component: (isActive: boolean) => <RivenPanel isActive={isActive} />, id: "riven" },
    { label: wishListTitle, component: (isActive: boolean) => <WishListPanel isActive={isActive} />, id: "wish_list" },
  ], [itemTitle, rivenTitle, wishListTitle]);

  const [activeTab, setActiveTab] = useLocalStorage<string>({
    key: "live_scraper.active_tab",
    defaultValue: tabs[0].id,
  });

  return (
    <Container size={"100%"}>
      <Box data-has-alert={hasAlert} className={classes.liveScraper}>
        <LiveScraperControl />
      </Box>
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
            {activeTab === tab.id && tab.component(true)}
          </Tabs.Panel>
        ))}
      </Tabs>
    </Container>
  );
}

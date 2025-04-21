import { Container, Tabs } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { OverviewPanel } from "./tabs/overview";
import { ItemPanel } from "./tabs/Item";
import { SyndicatesPanel } from "./tabs/syndicates";
import { RivenPanel } from "./tabs/riven";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "./Prices.module.css";
export default function PricesPage() {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`prices.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    { label: useTranslateTabs("overview.title"), component: <OverviewPanel />, id: "overview", icon: <div>Stocks</div> },
    { label: useTranslateTabs("item.title"), component: <ItemPanel />, id: "item", icon: <div>Stocks</div> },
    { label: useTranslateTabs("syndicate.title"), component: <SyndicatesPanel />, id: "riven" },
    { label: useTranslateTabs("riven.title"), component: <RivenPanel />, id: "wish_list" },
  ];
  return (
    <Container size={"100%"} data-has-alert={useHasAlert()} className={classes.context}>
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
    </Container>
  );
}

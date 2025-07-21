import { Container, Tabs } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { LoggingPanel } from "./tabs/logging";
import { StatesPanel } from "./tabs/states";
import classes from "./Debug.module.css";
import { useHasAlert } from "@hooks/useHasAlert.hook";
export default function DebugPage() {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`debug.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    { label: useTranslateTabs("logging.title"), component: <LoggingPanel />, id: "tr", icon: <div>Stocks</div> },
    { label: useTranslateTabs("states.title"), component: <StatesPanel />, id: "states", icon: <div>States</div> },
  ];
  return (
    <Container p={20} size={"900%"} className={`${classes.container} ${useHasAlert() ? classes.alert : ""}`}>
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

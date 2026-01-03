import { Container, Tabs } from "@mantine/core";
import { TradePanel, LoginPanel, PurchasePanel } from "./Tabs";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { invoke } from "@tauri-apps/api/core";
import { Loading } from "@components/Shared/Loading";
import { useTauriDragDrop } from "@hooks/useTauriDragDrop.hook";
import { useState } from "react";

interface WarframeGDPRParserProps {}

export const WarframeGDPRParser = ({}: WarframeGDPRParserProps = {}) => {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.wfgdpr.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("trade.title"),
      component: () => <TradePanel />,
      id: "trade",
    },
    {
      label: useTranslateTabs("purchase.title"),
      component: () => <PurchasePanel />,
      id: "purchase",
    },
    {
      label: useTranslateTabs("login.title"),
      component: () => <LoginPanel />,
      id: "login",
    },
  ];

  const [loading, setLoading] = useState(false);

  useTauriDragDrop({
    onDrop: async (path) => {
      try {
        setLoading(true);
        await invoke("wfgdpr_load", { filePath: path });
      } catch (err) {
        console.error("Error parsing WFGDPR data:", err);
      } finally {
        setLoading(false);
      }
    },
  });

  return (
    <Container size={"100%"} h={"95vh"} p={0} pos={"relative"}>
      {loading && <Loading />}
      <Tabs defaultValue={tabs[0].id} orientation="vertical">
        <Tabs.List>
          {tabs.map((tab) => (
            <Tabs.Tab value={tab.id} key={tab.id}>
              {tab.label}
            </Tabs.Tab>
          ))}
        </Tabs.List>
        {tabs.map((tab) => (
          <Tabs.Panel value={tab.id} key={tab.id}>
            {tab.component()}
          </Tabs.Panel>
        ))}
      </Tabs>
    </Container>
  );
};

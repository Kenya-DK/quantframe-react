import { Center, Container, Overlay, Tabs, Text } from "@mantine/core";
import { TradePanel, LoginPanel, PurchasePanel, TransactionPanel } from "./Tabs";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { invoke } from "@tauri-apps/api/core";
import { Loading } from "@components/Shared/Loading";
import { useTauriDragDrop } from "@hooks/useTauriDragDrop.hook";
import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";

interface WarframeGDPRParserProps {
  isActive?: boolean;
}

export const WarframeGDPRParser = ({ isActive }: WarframeGDPRParserProps = {}) => {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.wfgdpr.${key}`, { ...context }, i18Key);

  const { data, refetch } = useQuery({
    queryKey: ["get_state"],
    queryFn: () => api.log_parser.getState(),
    retry: false,
  });

  const tabs = [
    {
      label: useTranslateTabs("trade.title"),
      component: (isActive: boolean) => (
        <TradePanel isActive={isActive && activeTab === "trade" && data?.was_initialized} year_list={data?.trade_years} />
      ),
      id: "trade",
    },
    {
      label: useTranslateTabs("purchase.title"),
      component: (isActive: boolean) => <PurchasePanel isActive={isActive && activeTab === "purchase" && data?.was_initialized} />,
      id: "purchase",
    },
    {
      label: useTranslateTabs("login.title"),
      component: (isActive: boolean) => <LoginPanel isActive={isActive && activeTab === "login" && data?.was_initialized} />,
      id: "login",
    },
    {
      label: useTranslateTabs("transaction.title"),
      component: (isActive: boolean) => <TransactionPanel isActive={isActive && activeTab === "transaction" && data?.was_initialized} />,
      id: "transaction",
    },
  ];

  const [activeTab, setActiveTab] = useState<string | null>(tabs[0].id);
  const [loading, setLoading] = useState(false);

  useTauriDragDrop({
    onDrop: async (path) => {
      try {
        setLoading(true);
        await invoke("wfgdpr_load", { filePath: path });
        await refetch();
      } catch (err) {
        console.error("Error parsing WFGDPR data:", err);
      } finally {
        setLoading(false);
      }
    },
  });

  return (
    <Container size={"100%"} h={"95vh"} p={0} pos={"relative"}>
      {!data?.was_initialized && (
        <Overlay>
          <Center h={"95vh"}>
            <Text size="lg" fw={700} fs={"italic"}>
              {useTranslateTabs("drag_and_drop_message")}
            </Text>
          </Center>
        </Overlay>
      )}
      {loading && <Loading />}
      <Tabs defaultValue={tabs[0].id} orientation="vertical" value={activeTab} onChange={setActiveTab} h={"100%"}>
        <Tabs.List>
          {tabs.map((tab) => (
            <Tabs.Tab value={tab.id} key={tab.id}>
              {tab.label}
            </Tabs.Tab>
          ))}
        </Tabs.List>
        {tabs.map((tab) => (
          <Tabs.Panel value={tab.id} key={tab.id}>
            {tab.component(isActive !== false)}
          </Tabs.Panel>
        ))}
      </Tabs>
    </Container>
  );
};

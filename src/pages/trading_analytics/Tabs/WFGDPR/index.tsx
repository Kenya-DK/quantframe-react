import { Center, Container, Group, Overlay, Table, Tabs, Text } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { invoke } from "@tauri-apps/api/core";
import { Loading } from "@components/Shared/Loading";
import { useTauriDragDrop } from "@hooks/useTauriDragDrop.hook";
import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TauriTypes } from "$types";
import { TradePanel, LoginPanel, PurchasePanel, TransactionPanel } from "./Tabs";
import { TimerStamp } from "@components/Shared/TimerStamp";
import { OverviewPanel } from "./Tabs/Overview";

enum View {
  Details = "details",
  Accounts = "accounts",
}

interface WarframeGDPRParserProps {
  isActive?: boolean;
}
export const WarframeGDPRParser = ({}: WarframeGDPRParserProps = {}) => {
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
  const { data: accounts } = useQuery({
    queryKey: ["get_accounts"],
    queryFn: () => api.log_parser.getAccounts(),
    retry: false,
    enabled: !!data?.was_initialized,
  });

  const [loading, setLoading] = useState(false);
  const [selectedAccount, setSelectedAccount] = useState<TauriTypes.WFGDPRAccount | null>(null);
  const [view, setView] = useState<View>(View.Accounts);
  const tabs = [
    {
      label: useTranslateTabs("overview.title"),
      component: () => <OverviewPanel value={selectedAccount} />,
      id: "overview",
    },
    {
      label: useTranslateTabs("trade.title"),
      component: () => <TradePanel value={selectedAccount} />,
      id: "trade",
    },
    {
      label: useTranslateTabs("purchase.title"),
      component: () => <PurchasePanel value={selectedAccount} />,
      id: "purchase",
    },
    {
      label: useTranslateTabs("login.title"),
      component: () => <LoginPanel value={selectedAccount} />,
      id: "login",
    },
    {
      label: useTranslateTabs("transaction.title"),
      component: () => <TransactionPanel value={selectedAccount} />,
      id: "transaction",
    },
  ];
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
      {view === View.Accounts && (
        <Table.ScrollContainer minWidth={800}>
          <Table verticalSpacing="sm">
            <Table.Thead>
              <Table.Tr>
                <Table.Th>{useTranslateTabs("columns.account")}</Table.Th>
                <Table.Th>{useTranslateTabs("columns.logins")}</Table.Th>
                <Table.Th>{useTranslateTabs("columns.purchases")}</Table.Th>
                <Table.Th>{useTranslateTabs("columns.trades")}</Table.Th>
                <Table.Th>{useTranslateTabs("columns.transactions")}</Table.Th>
                <Table.Th>{useTranslateTabs("columns.creation_date")}</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {accounts?.map((account, i) => (
                <Table.Tr
                  key={i}
                  onClick={() => {
                    setSelectedAccount(account);
                    setView(View.Details);
                  }}
                  style={{ cursor: "pointer" }}
                >
                  <Table.Td>
                    <Group gap="sm">
                      <div>
                        <Text fz="sm" fw={500}>
                          {account.display_name}
                        </Text>
                        <Text fz="xs" c="dimmed">
                          {account.email}
                        </Text>
                      </div>
                    </Group>
                  </Table.Td>
                  <Table.Td>
                    <Text fz="sm">{account.logins.length}</Text>
                  </Table.Td>
                  <Table.Td>
                    <Text fz="sm">{account.purchases.length}</Text>
                  </Table.Td>
                  <Table.Td>
                    <Text fz="sm">{account.trades.length}</Text>
                  </Table.Td>
                  <Table.Td>
                    <Text fz="sm">{account.transactions.length}</Text>
                  </Table.Td>
                  <Table.Td>
                    <TimerStamp date={account.account_creation_date} />
                  </Table.Td>
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        </Table.ScrollContainer>
      )}
      {view === View.Details && selectedAccount && (
        <Tabs defaultValue={tabs[0].id} orientation="vertical" h={"100%"}>
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
      )}
    </Container>
  );
};

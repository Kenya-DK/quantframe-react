import { Box, Container, Divider, Group, Tabs, Text } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { OverviewTab, AnalyticsTab, WFMTab } from "./Tabs/index";
import { Loading } from "@components/Shared/Loading";
import { ItemName } from "@components/DataDisplay/ItemName";

export type StockRivenDetailsModalProps = {
  value: number;
};

export function StockRivenDetailsModal({ value }: StockRivenDetailsModalProps) {
  const { data } = useQuery({
    queryKey: ["stock_riven", value],
    queryFn: () => api.stock_riven.getById(value),
  });

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`stock_riven_details.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    { label: useTranslateTabs("overview.title"), component: <OverviewTab value={data} />, id: "overview" },
    { label: useTranslateTabs("analytics.title"), component: <AnalyticsTab value={data} />, id: "analytics" },
    { label: useTranslateTabs("wfm.title"), component: <WFMTab value={data} />, id: "wfm" },
  ];
  return (
    <Container size={"100%"} p={0}>
      {!data && (
        <Box p={"lg"} h={"50vh"}>
          <Loading text="Loading..." />
        </Box>
      )}

      {data && (
        <>
          <Group justify="space-between" mb={"md"}>
            <Text fw={500}>
              <ItemName value={data.stock} />
            </Text>
          </Group>
          <Divider />
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
        </>
      )}
    </Container>
  );
}

import { Box, Container, Divider, Group, Stack, Tabs, Text } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { OverviewTab, AnalyticsTab, WFMTab } from "./Tabs/index";
import { modals } from "@mantine/modals";
import { faClose } from "@fortawesome/free-solid-svg-icons";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { Loading } from "@components/Shared/Loading";

export type WishListItemDetailsModalProps = {
  value: number;
};

export function WishListItemDetailsModal({ value }: WishListItemDetailsModalProps) {
  const { data } = useQuery({
    queryKey: ["wish_list", value],
    queryFn: () => api.wish_list.getById(value),
  });

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`wish_list_details.${key}`, { ...context }, i18Key);
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
            <Stack gap={0}>
              <Text fw={500}>TODO</Text>
            </Stack>
            <Group>
              <ActionWithTooltip color="red.7" icon={faClose} tooltip={useTranslate("buttons.refresh")} onClick={() => modals.closeAll()} />
            </Group>
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

import { Box, Container, Divider, Group, Tabs } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { OverviewTab } from "./Tabs/index";
import { Loading } from "@components/Shared/Loading";
import { ItemName } from "../../DataDisplay/ItemName/ItemName";

export type WFMOrderDetailsModalProps = {
  value: string;
};
export function WFMOrderDetailsModal({ value }: WFMOrderDetailsModalProps) {
  const { data } = useQuery({
    queryKey: ["wfm_order", value],
    queryFn: () => api.order.getById(value),
  });

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`wfm_order_details.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [{ label: useTranslateTabs("overview.title"), component: <OverviewTab value={data} />, id: "overview" }];
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
            <ItemName value={data.order_info} />
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

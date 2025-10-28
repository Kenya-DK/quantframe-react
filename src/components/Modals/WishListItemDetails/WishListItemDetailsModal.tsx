import { Box, Container, Divider, Group, Tabs, Text } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { EditTab, OverviewTab, WFMTab } from "./Tabs/index";
import { Loading } from "@components/Shared/Loading";
import { TauriTypes } from "$types";
import { ItemName } from "@components/DataDisplay/ItemName";

export type WishListItemDetailsModalProps = {
  value: number;
  onUpdate?: (data: TauriTypes.UpdateWishListItem) => void;
};

export function WishListItemDetailsModal({ value, onUpdate }: WishListItemDetailsModalProps) {
  const { data } = useQuery({
    queryKey: ["wish_list", value],
    queryFn: () => api.wish_list.getById(value),
  });

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`wish_list_details.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    { label: useTranslateTabs("overview.title"), component: <OverviewTab value={data} />, id: "overview" },
    { label: useTranslateTabs("wfm.title"), component: <WFMTab value={data} />, id: "wfm" },
    { label: useTranslateTabs("edit.title"), component: <EditTab value={data} onUpdate={(data) => onUpdate?.(data)} />, id: "edit" },
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

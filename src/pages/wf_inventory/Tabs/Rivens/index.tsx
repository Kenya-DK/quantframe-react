import { ActionIcon, Box, Divider, Group, Pagination, ScrollArea, Select, SimpleGrid, Text } from "@mantine/core";
import { useQueries } from "./queries";
import { useMutations } from "./mutations";
import { RivenPreview } from "@components/DataDisplay/RivenPreview";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../WFInventory.module.css";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faAdd, faArrowDown, faArrowUp } from "@fortawesome/free-solid-svg-icons";
import { useLocalStorage } from "@mantine/hooks";
import { TauriTypes } from "$types";
import { SearchField } from "@components/Forms/SearchField";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
interface RivenPanelProps {
  isActive: boolean;
}
export const RivenPanel = ({ isActive }: RivenPanelProps) => {
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.VeiledRivenControllerGetListParams>({
    key: "veiled_riven_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50 },
  });

  // Queries
  const { veiledRivensQuery } = useQueries({ queryData, isActive });

  // Mutations
  const { createMutation } = useMutations({
    refetchQueries: () => {},
    setLoadingRows: () => {},
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`wf_inventory.tabs.riven.${key}`, { ...context }, i18Key);

  const AddRivenToStock = (riven: TauriTypes.VeiledRiven) => {
    createMutation.mutate({
      raw: riven.unique_name,
      mod_name: riven.mod_name,
      mastery_rank: riven.mastery_rank,
      re_rolls: riven.rerolls,
      polarity: riven.polarity,
      attributes: riven.attributes,
      rank: riven.rank,
      bought: 0,
    });
  };

  return (
    <Box p={"md"}>
      <SearchField value={queryData.query || ""} onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))} />
      <Group gap={"sm"} mt={"md"} justify="space-between">
        <Group>TEST</Group>
        <Group gap="xs">
          <Select
            value={queryData.sort_by}
            allowDeselect={false}
            onChange={(value) => setQueryData((prev) => ({ ...prev, sort_by: value || "platinum" }))}
            data={["created_at", "platinum", "updated_at", "order_type"].map((key) => ({ label: useTranslateCommon(`sort_by.${key}`), value: key }))}
            size="xs"
          />
          <ActionIcon
            variant="light"
            color="blue"
            size="lg"
            onClick={() => {
              const direction = queryData.sort_direction === "asc" ? "desc" : "asc";
              setQueryData((prev) => ({ ...prev, sort_direction: direction }));
            }}
          >
            {queryData.sort_direction === "asc" ? <FontAwesomeIcon icon={faArrowUp} /> : <FontAwesomeIcon icon={faArrowDown} />}
          </ActionIcon>
        </Group>
      </Group>

      <ScrollArea mt={"md"} className={classes.veiledRivens} data-has-alert={useHasAlert()}>
        {veiledRivensQuery.isFetching && <></>}
        <SimpleGrid cols={{ base: 4 }} spacing="sm">
          {veiledRivensQuery.data?.results?.map((riven, i) => (
            <RivenPreview
              key={i}
              riven={riven}
              type="withoutBackground"
              footerCenter={{
                i18nKey: useTranslate("riven_card.footer_center", undefined, true),
                values: { endo: riven.endo, kuva: riven.kuva },
                components: {
                  add: (
                    <ActionWithTooltip
                      icon={faAdd}
                      tooltip="Add To stock"
                      onClick={() => AddRivenToStock(riven)}
                      actionProps={{ size: "sm" }}
                      iconProps={{ size: "xs" }}
                    />
                  ),
                },
              }}
            />
          ))}
        </SimpleGrid>
      </ScrollArea>
      <Divider mt={"md"} />
      <Group grow mt={"md"}>
        <Text>
          {useTranslateCommon("pagination_total_items", {
            start: (queryData.page - 1) * queryData.limit + 1,
            end: Math.min(queryData.page * queryData.limit, veiledRivensQuery.data?.total || 0),
            total: veiledRivensQuery.data?.total || 0,
          })}
        </Text>
        <Group justify="flex-end">
          <Pagination
            value={queryData.page}
            onChange={(page) => setQueryData((prev) => ({ ...prev, page }))}
            total={Math.ceil((veiledRivensQuery.data?.total || 0) / queryData.limit)}
          />
        </Group>
      </Group>
    </Box>
  );
};

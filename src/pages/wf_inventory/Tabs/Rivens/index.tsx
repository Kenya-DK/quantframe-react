import { Box, Divider, Group, Menu, Pagination, ScrollArea, Select, SimpleGrid, Text, Tooltip } from "@mantine/core";
import { useQueries } from "./queries";
import { useMutations } from "./mutations";
import { RivenPreview } from "@components/DataDisplay/RivenPreview";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../WFInventory.module.css";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { faRefresh } from "@fortawesome/free-solid-svg-icons";
import { useLocalStorage } from "@mantine/hooks";
import { TauriTypes } from "$types";
import { SearchField } from "@components/Forms/SearchField";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faStockFound, getPolarityIcon } from "@icons";
import { CompactSortSelect } from "@components/Shared/CompactSortSelect";
interface RivenPanelProps {
  isActive: boolean;
}
export const RivenPanel = ({ isActive }: RivenPanelProps) => {
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.WFItemControllerGetListParams>({
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

  return (
    <Box p={"md"}>
      <SearchField value={queryData.query || ""} onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))} />
      <Group mt={"md"} justify="flex-end">
        <CompactSortSelect
          value={queryData.sort_by || "platinum"}
          direction={queryData.sort_direction === "asc" ? "asc" : "desc"}
          onChange={(value) => setQueryData((prev) => ({ ...prev, sort_by: value }))}
          onDirectionChange={(direction) => setQueryData((prev) => ({ ...prev, sort_direction: direction }))}
          data={["disposition", "endo", "riven_grade"].map((key) => ({ label: useTranslateCommon(`sort_by.${key}`), value: key }))}
        />
      </Group>

      <ScrollArea mt={"md"} className={classes.veiledRivens} data-has-alert={useHasAlert()}>
        <SimpleGrid cols={{ base: 4 }} spacing="sm">
          {veiledRivensQuery.data?.results?.map((riven, i) => (
            <RivenPreview
              key={i}
              value={riven}
              type="withoutBackground"
              footerLeft={{
                i18nKey: useTranslate("riven_card.footer_left", undefined, true),
                values: {
                  rank: riven.sub_type?.rank || 0,
                  rerolls: riven.re_rolls,
                },
                components: {
                  polarity: <FontAwesomeIcon size="sm" icon={getPolarityIcon(riven.polarity)} />,
                  refresh: <FontAwesomeIcon size="sm" icon={faRefresh} />,
                },
              }}
              footerCenter={{
                i18nKey: useTranslate("riven_card.footer_center", undefined, true),
                values: {},
                components: {
                  stock: (
                    <Menu shadow="md">
                      <Menu.Target>
                        <Tooltip label={useTranslate(`riven_card.stock_status.${riven.properties?.is_in_stock ? "found" : "not_found"}`)} withArrow>
                          <FontAwesomeIcon
                            icon={faStockFound}
                            size="xl"
                            cursor="pointer"
                            color={riven.properties?.is_in_stock ? "var(--mantine-color-green-6)" : "var(--mantine-color-red-6)"}
                          />
                        </Tooltip>
                      </Menu.Target>

                      <Menu.Dropdown>
                        <Menu.Item
                          onClick={async () => {
                            await createMutation.mutateAsync({
                              raw: riven.name,
                              mod_name: riven.mod_name,
                              attributes: riven.attributes,
                              mastery_rank: riven.mastery_rank,
                              re_rolls: riven.re_rolls,
                              polarity: riven.polarity,
                              rank: riven.sub_type?.rank || 0,
                              bought: 0,
                            });
                          }}
                        >
                          Add
                        </Menu.Item>
                      </Menu.Dropdown>
                    </Menu>
                  ),
                },
              }}
              footerRight={{
                i18nKey: useTranslate("riven_card.footer_right", undefined, true),
                values: { mastery: riven.mastery_rank },
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
          <Select
            size="xs"
            w={90}
            allowDeselect={false}
            value={String(queryData.limit || 50)}
            data={["25", "50", "100", "200"]}
            onChange={(value) => {
              if (!value) return;
              setQueryData((prev) => ({ ...prev, page: 1, limit: Number(value) }));
            }}
          />
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

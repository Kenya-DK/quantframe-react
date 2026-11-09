import { ItemRiven, TauriTypes } from "$types";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { Box, Chip, Divider, Group, ScrollArea, SegmentedControl, SimpleGrid } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { SearchField } from "../../../components/Forms/SearchField";
import { CompactSortSelect } from "../../../components/Shared/CompactSortSelect";
import { PaginationFooter } from "../../../components/Shared/PaginationFooter";
import classes from "../WFInventory.module.css";
import { RIVEN_ITEM_TYPES } from "../filters";
import { RivenPreviewCard } from "./RivenPreviewCard";
import { useModals } from "./modals";
import { useMutations } from "./mutations";
import { useQueries } from "./queries";

const sameSet = (a?: string[], b?: string[]) => !!a && !!b && a.length === b.length && a.every((t) => b.includes(t));

export default function WFInventoryRivenPage() {
  // Translate general
  const useTranslateFilters = (key: string) => useTranslatePages(`wf_inventory.tabs.riven.filters.${key}`);

  // States
  const [queryData, setQueryParams] = useLocalStorage<TauriTypes.WFItemControllerGetListParams>({
    key: "wf_inv_riven_query",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50, properties: { riven_type: "veiled" } },
  });
  const activeCategory = Object.keys(RIVEN_ITEM_TYPES).find((key) => sameSet(RIVEN_ITEM_TYPES[key], queryData.item_types)) || "all";

  // Queries
  const { veiledRivensQuery: rivensQuery, refreshVeiledRivens } = useQueries({ queryData, isActive: true });

  // Mutations (add veiled riven to stock, mirrors the classic Rivens tab)
  const { createMutation } = useMutations({ refetchQueries: () => refreshVeiledRivens(), setLoadingRows: () => {} });
  const { OpenBoughtModal } = useModals({ createMutation });

  const HandleAddRiven = async (riven: ItemRiven) => {
    if (riven.riven_type !== "veiled") return;
    OpenBoughtModal({
      raw: riven.name,
      mod_name: riven.mod_name,
      attributes: riven.attributes,
      mastery_rank: riven.mastery_rank,
      re_rolls: riven.re_rolls,
      polarity: riven.polarity,
      rank: riven.sub_type?.rank || 0,
      bought: 0,
    });
  };

  return (
    <Box p="md">
      <SearchField value={queryData.query || ""} onChange={(text) => setQueryParams((prev) => ({ ...prev, page: 1, query: text }))} />
      <Group mt="md" justify="space-between" wrap="nowrap">
        <Group gap="xs">
          <Chip.Group multiple={false} value={activeCategory}>
            <Group gap="xs">
              {Object.keys(RIVEN_ITEM_TYPES).map((key) => (
                <Chip
                  key={key}
                  value={key}
                  onChange={() =>
                    setQueryParams((prev) => ({ ...prev, page: 1, item_types: activeCategory === key ? undefined : RIVEN_ITEM_TYPES[key] }))
                  }
                >
                  {useTranslateFilters(key)}
                </Chip>
              ))}
            </Group>
          </Chip.Group>
          <SegmentedControl
            size="xs"
            value={queryData.properties?.riven_type || "veiled"}
            onChange={(value) => setQueryParams((prev) => ({ ...prev, page: 1, properties: { ...prev.properties, riven_type: value } }))}
            data={[
              { value: "veiled", label: useTranslateFilters("veiled") },
              { value: "unveiled", label: useTranslateFilters("unveiled") },
            ]}
          />
        </Group>
        <CompactSortSelect
          value={queryData.sort_by || "platinum"}
          direction={(queryData.sort_direction || "desc") === "desc" ? "desc" : "asc"}
          onChange={(value) => setQueryParams((prev) => ({ ...prev, sort_by: value }))}
          onDirectionChange={(direction) => setQueryParams((prev) => ({ ...prev, sort_direction: direction }))}
          data={["disposition", "endo", "riven_grade"].map((key) => ({
            label: useTranslateCommon(`sort_by.${key}`),
            value: key,
          }))}
        />
      </Group>
      <ScrollArea mt="md" className={classes.veiledRivens} data-has-alert={useHasAlert()}>
        <SimpleGrid cols={{ base: 4 }} spacing="sm">
          {rivensQuery.data?.results?.map((riven, i) => (
            <RivenPreviewCard key={i} riven={riven} onAdd={HandleAddRiven} />
          ))}
        </SimpleGrid>
      </ScrollArea>
      <Divider mt="md" />
      <Box mt="md">
        <PaginationFooter
          page={queryData.page}
          limit={queryData.limit || 50}
          total={rivensQuery.data?.total || 0}
          onPageChange={(page) => setQueryParams((prev) => ({ ...prev, page }))}
          onLimitChange={(limit) => setQueryParams((prev) => ({ ...prev, page: 1, limit }))}
        />
      </Box>
    </Box>
  );
}

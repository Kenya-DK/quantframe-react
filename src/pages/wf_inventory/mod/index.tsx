import { TauriTypes } from "$types";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { Box, Divider, Group, ScrollArea, SimpleGrid } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { SearchField } from "../../../components/Forms/SearchField";
import { CompactSortSelect } from "../../../components/Shared/CompactSortSelect";
import { PaginationFooter } from "../../../components/Shared/PaginationFooter";
import classes from "../WFInventory.module.css";
import { ModPreviewCard } from "./ModPreviewCard";
import { useQueries } from "./queries";

const sameSet = (a?: string[], b?: string[]) => !!a && !!b && a.length === b.length && a.every((t) => b.includes(t));

export default function WFInventoryModsPage() {
  // Translate general
  const useTranslateFilters = (key: string) => useTranslatePages(`wf_inventory.tabs.mods.filters.${key}`);
  const useTranslateSort = (key: string) => useTranslatePages(`wf_inventory.tabs.mods.sort.${key}`);

  // States
  const [queryData, setQueryParams] = useLocalStorage<TauriTypes.WFItemControllerGetListParams>({
    key: "wf_inv_mods_query",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50 },
  });
  // Queries
  const { modsQuery } = useQueries({ queryData, isActive: true });

  return (
    <Box p="md">
      <SearchField value={queryData.query || ""} onChange={(text) => setQueryParams((prev) => ({ ...prev, page: 1, query: text }))} />
      <Group mt="md" justify="space-between" wrap="nowrap">
        <CompactSortSelect
          value={queryData.sort_by || "name"}
          direction={(queryData.sort_direction || "asc") === "desc" ? "desc" : "asc"}
          onChange={(value) => setQueryParams((prev) => ({ ...prev, sort_by: value }))}
          onDirectionChange={(direction) => setQueryParams((prev) => ({ ...prev, sort_direction: direction }))}
          data={["name", "quantity", "mod_rank"].map((key) => ({ label: useTranslateSort(key), value: key }))}
        />
      </Group>
      <ScrollArea mt="md" className={classes.veiledRivens} data-has-alert={useHasAlert()}>
        <SimpleGrid cols={{ base: 4 }} spacing="sm">
          {modsQuery.data?.results?.map((mod, i) => (
            <ModPreviewCard key={i} mod={mod as any} />
          ))}
        </SimpleGrid>
      </ScrollArea>
      <Divider mt="md" />
      <Box mt="md">
        <PaginationFooter
          page={queryData.page}
          limit={queryData.limit || 50}
          total={modsQuery.data?.total || 0}
          onPageChange={(page) => setQueryParams((prev) => ({ ...prev, page }))}
          onLimitChange={(limit) => setQueryParams((prev) => ({ ...prev, page: 1, limit }))}
        />
      </Box>
    </Box>
  );
}

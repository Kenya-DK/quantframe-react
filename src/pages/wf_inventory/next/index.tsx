import { Box, Chip, Divider, Group, ScrollArea, SegmentedControl, SimpleGrid } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useParams } from "react-router-dom";
import { SearchField } from "@components/Forms/SearchField";
import { CompactSortSelect } from "@components/Shared/CompactSortSelect";
import { PaginationFooter } from "@components/Shared/PaginationFooter";
import classes from "../WFInventory.module.css";
import { ItemRiven, TauriTypes } from "$types";
import { MOD_ITEM_TYPES, RIVEN_ITEM_TYPES } from "../filters";
import { useQueries as useModsQueries } from "../Tabs/Mods/queries";
import { useQueries as useRivensQueries } from "../Tabs/Rivens/queries";
import { useMutations } from "../Tabs/Rivens/mutations";
import { useModals } from "../Tabs/Rivens/modals";
import { ModPreviewCard } from "../Tabs/Mods/ModPreviewCard";
import { RivenPreviewCard } from "../Tabs/Rivens/RivenPreviewCard";

type Section = "mods" | "rivens";

const sameSet = (a?: string[], b?: string[]) => !!a && !!b && a.length === b.length && a.every((t) => b.includes(t));

/**
 * "Next" WF inventory page, hosted by the WFInventoryLayout (icon nav rail):
 * toolbar with search/facets/sort, then the card wall. Reuses the classic
 * tabs' queries and cards — this page owns only the content and query state.
 */
export default function WfInventoryNextPage() {
  // Section comes from the layout's navigation (/wf_inventory_next/mods or /rivens).
  const { section: urlSection } = useParams();
  const section: Section = urlSection === "rivens" ? "rivens" : "mods";

  // Translate general
  const useTranslateRivenFilters = (key: string) => useTranslatePages(`wf_inventory.tabs.riven.filters.${key}`);
  const useTranslateModFilters = (key: string) => useTranslatePages(`wf_inventory.tabs.mods.filters.${key}`);

  // States
  const [modsParams, setModsParams] = useLocalStorage<TauriTypes.WFItemControllerGetListParams>({
    key: "next_mods_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50 },
  });
  const [rivenParams, setRivenParams] = useLocalStorage<TauriTypes.WFItemControllerGetListParams>({
    key: "next_riven_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50, properties: { riven_type: "veiled" } },
  });
  const params = section === "mods" ? modsParams : rivenParams;
  const setParams = section === "mods" ? setModsParams : setRivenParams;

  // Queries
  const { modsQuery } = useModsQueries({ queryData: modsParams, isActive: section === "mods" });
  const { veiledRivensQuery: rivensQuery, refreshVeiledRivens } = useRivensQueries({ queryData: rivenParams, isActive: section === "rivens" });

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

  // Facets of the active section
  const categoryTypes = section === "mods" ? MOD_ITEM_TYPES : RIVEN_ITEM_TYPES;
  const activeCategory = Object.keys(categoryTypes).find((key) => sameSet(categoryTypes[key], params.item_types)) || "all";

  return (
    <Box p="md">
      <SearchField value={params.query || ""} onChange={(text) => setParams((prev) => ({ ...prev, page: 1, query: text }))} />
      <Group mt="md" justify="space-between" wrap="nowrap">
        <Group gap="xs">
          <Chip.Group multiple={false} value={activeCategory}>
            {Object.keys(categoryTypes).map((key) => (
              <Chip
                key={key}
                value={key}
                onChange={() =>
                  setParams((prev) => ({ ...prev, page: 1, item_types: activeCategory === key ? undefined : categoryTypes[key] }))
                }
              >
                {section === "mods" ? useTranslateModFilters(key) : useTranslateRivenFilters(key)}
              </Chip>
            ))}
          </Chip.Group>
          {section === "rivens" && (
            <SegmentedControl
              size="xs"
              value={rivenParams.properties?.riven_type || "veiled"}
              onChange={(value) => setRivenParams((prev) => ({ ...prev, page: 1, properties: { ...prev.properties, riven_type: value } }))}
              data={[
                { value: "veiled", label: useTranslateRivenFilters("veiled") },
                { value: "unveiled", label: useTranslateRivenFilters("unveiled") },
              ]}
            />
          )}
        </Group>
        <CompactSortSelect
          value={params.sort_by || (section === "mods" ? "name" : "platinum")}
          direction={(params.sort_direction || (section === "mods" ? "asc" : "desc")) === "desc" ? "desc" : "asc"}
          onChange={(value) => setParams((prev) => ({ ...prev, sort_by: value }))}
          onDirectionChange={(direction) => setParams((prev) => ({ ...prev, sort_direction: direction }))}
          data={(section === "mods" ? ["name", "quantity", "mod_rank"] : ["disposition", "endo", "riven_grade"]).map((key) => ({
            label: section === "mods" ? useTranslatePages(`wf_inventory.tabs.mods.sort.${key}`) : useTranslateCommon(`sort_by.${key}`),
            value: key,
          }))}
        />
      </Group>
      <ScrollArea mt="md" className={classes.veiledRivens} data-has-alert={useHasAlert()}>
        <SimpleGrid cols={{ base: 4 }} spacing="sm">
          {section === "mods"
            ? modsQuery.data?.results?.map((mod, i) => <ModPreviewCard key={i} mod={mod} />)
            : rivensQuery.data?.results?.map((riven, i) => <RivenPreviewCard key={i} riven={riven} onAdd={HandleAddRiven} />)}
        </SimpleGrid>
      </ScrollArea>
      <Divider mt="md" />
      <Box mt="md">
        <PaginationFooter
          page={params.page}
          limit={params.limit || 50}
          total={(section === "mods" ? modsQuery.data?.total : rivensQuery.data?.total) || 0}
          onPageChange={(page) => setParams((prev) => ({ ...prev, page }))}
          onLimitChange={(limit) => setParams((prev) => ({ ...prev, page: 1, limit }))}
        />
      </Box>
    </Box>
  );
}

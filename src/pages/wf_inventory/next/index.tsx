import { AppShell, Box, Chip, Divider, Group, ScrollArea, SegmentedControl, SimpleGrid, Text } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { Header } from "@components/Layouts/Shared/Header";
import { SearchField } from "@components/Forms/SearchField";
import { CompactSortSelect } from "@components/Shared/CompactSortSelect";
import { PaginationFooter } from "@components/Shared/PaginationFooter";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
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
 * "Next" WF inventory layout: persistent facet drawer (navbar) + edge-to-edge
 * card wall. Reuses the classic tabs' queries and cards — this page owns only
 * the layout and the shared query state.
 */
export default function WfInventoryNextPage() {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`wf_inventory.next.${key}`, { ...context }, i18Key);
  const useTranslateRivenFilters = (key: string) => useTranslatePages(`wf_inventory.tabs.riven.filters.${key}`);
  const useTranslateModFilters = (key: string) => useTranslatePages(`wf_inventory.tabs.mods.filters.${key}`);

  // States
  const [section, setSection] = useLocalStorage<Section>({ key: "next_wf_section", getInitialValueInEffect: false, defaultValue: "mods" });
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
    <AppShell header={{ height: 65 }} navbar={{ width: 250, breakpoint: "sm" }}>
      <AppShell.Header withBorder={false}>
        <Header
          showClock={false}
          userMenuBackTo="/wf_inventory"
          center={
            <Group gap="xs" w={420}>
              <SearchField value={params.query || ""} onChange={(text) => setParams((prev) => ({ ...prev, page: 1, query: text }))} />
            </Group>
          }
        />
      </AppShell.Header>

      <AppShell.Navbar withBorder={false} p="md">
        <Text fw={600} mb="xs">
          {useTranslate("sections." + section)}
        </Text>
        <SegmentedControl
          fullWidth
          mb="md"
          value={section}
          onChange={(value) => setSection(value as Section)}
          data={[
            { value: "mods", label: useTranslate("sections.mods") },
            { value: "rivens", label: useTranslate("sections.rivens") },
          ]}
        />
        <Text fz="xs" tt="uppercase" c="dimmed" mb="xs">
          {useTranslate("filters.category")}
        </Text>
        <Box mb="md">
          <Chip.Group multiple={false} value={activeCategory}>
            <Chip value="all" onChange={() => setParams((prev) => ({ ...prev, page: 1, item_types: undefined }))}>
              {section === "mods" ? useTranslateModFilters("all") : useTranslateRivenFilters("all")}
            </Chip>
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
        </Box>
        {section === "rivens" && (
          <>
            <Text fz="xs" tt="uppercase" c="dimmed" mb="xs">
              {useTranslate("filters.state")}
            </Text>
            <SegmentedControl
              fullWidth
              size="xs"
              value={rivenParams.properties?.riven_type || "veiled"}
              onChange={(value) => setRivenParams((prev) => ({ ...prev, page: 1, properties: { ...prev.properties, riven_type: value } }))}
              data={[
                { value: "veiled", label: useTranslateRivenFilters("veiled") },
                { value: "unveiled", label: useTranslateRivenFilters("unveiled") },
              ]}
            />
          </>
        )}
      </AppShell.Navbar>

      <AppShell.Main pt="md">
        <Group px="md" pb="sm" justify="flex-end">
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
        <ScrollArea px="md" className={classes.veiledRivens} data-has-alert={useHasAlert()}>
          <SimpleGrid cols={{ base: 4 }} spacing="sm">
            {section === "mods"
              ? modsQuery.data?.results?.map((mod, i) => <ModPreviewCard key={i} mod={mod} />)
              : rivensQuery.data?.results?.map((riven, i) => <RivenPreviewCard key={i} riven={riven} onAdd={HandleAddRiven} />)}
          </SimpleGrid>
        </ScrollArea>
        <Divider mt="md" mx="md" />
        <Box px="md" py="md">
          <PaginationFooter
            page={params.page}
            limit={params.limit || 50}
            total={(section === "mods" ? modsQuery.data?.total : rivensQuery.data?.total) || 0}
            onPageChange={(page) => setParams((prev) => ({ ...prev, page }))}
            onLimitChange={(limit) => setParams((prev) => ({ ...prev, page: 1, limit }))}
          />
        </Box>
      </AppShell.Main>
    </AppShell>
  );
}

import { Box, Divider, Group, Image, ScrollArea, SimpleGrid, Text } from "@mantine/core";
import { PaginationFooter } from "@components/Shared/PaginationFooter";
import { useQueries } from "./queries";
import classes from "../../WFInventory.module.css";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useLocalStorage } from "@mantine/hooks";
import { TauriTypes } from "$types";
import { SearchField } from "@components/Forms/SearchField";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faInfinity } from "@fortawesome/free-solid-svg-icons";
import { CompactSortSelect } from "@components/Shared/CompactSortSelect";
import { ExpandableButton } from "@components/Shared/ExpandableButton";
import { PreviewCard } from "@components/Shared/PreviewCard";

interface ModsPanelProps {
  isActive: boolean;
}

interface ExpandableButtonExtProps {
  img?: string;
  icon?: React.ReactNode;
  text: string;
  currentTypes: string[];
  types: string[];
  onClick: (types: string[] | undefined) => () => void;
}

const ExpandableButtonExt: React.FC<ExpandableButtonExtProps> = ({ img, icon, text, currentTypes, types, onClick }) => (
  <ExpandableButton
    p={3}
    collapsedWidth={35}
    icon={img ? <Image src={img} w="25px" fit="contain" left={4} /> : icon}
    selected={types.length == 0 ? true : types.some((type) => currentTypes?.includes(type))}
    onClick={onClick(types.length == 0 ? undefined : types)}
  >
    <Text>{text}</Text>
  </ExpandableButton>
);

export const ModsPanel = ({ isActive }: ModsPanelProps) => {
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.WFItemControllerGetListParams>({
    key: "mods_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50 },
  });

  // Queries
  const { modsQuery } = useQueries({ queryData, isActive });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`wf_inventory.tabs.mods.${key}`, { ...context }, i18Key);

  return (
    <Box p={"md"}>
      <SearchField value={queryData.query || ""} onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))} />
      <Group mt={"md"} justify="space-between">
        <Group gap={3}>
          <ExpandableButton
            p={3}
            collapsedWidth={32}
            icon={<FontAwesomeIcon icon={faInfinity} size="xl" />}
            selected={queryData.item_types === undefined}
            onClick={() => setQueryData((prev) => ({ ...prev, item_types: undefined }))}
          >
            <Text>{useTranslate("filters.all")}</Text>
          </ExpandableButton>
          <ExpandableButtonExt
            img="/imgs/categories/category_warframe.png"
            text={useTranslate("filters.warframe")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Warframe/", "/Lotus/Upgrades/Mods/Aura/"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_rifle.png"
            text={useTranslate("filters.primary")}
            currentTypes={queryData.item_types || []}
            types={[
              "/Lotus/Upgrades/Mods/Rifle/",
              "/Lotus/Upgrades/Mods/Shotgun/",
              "/Lotus/Upgrades/Mods/Pistol/",
              "/Lotus/Upgrades/Mods/Bow/",
              "/Lotus/Upgrades/Mods/DualSource/",
            ]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_melee.png"
            text={useTranslate("filters.melee")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Melee/", "/Lotus/Upgrades/Mods/Hoverboard/"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_archwing.png"
            text={useTranslate("filters.archwing")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Archwing/", "/Lotus/Upgrades/Mods/Railjack/", "/Lotus/Upgrades/Mods/Necromech/"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
        </Group>
        <CompactSortSelect
          value={queryData.sort_by || "name"}
          direction={queryData.sort_direction === "desc" ? "desc" : "asc"}
          onChange={(value) => setQueryData((prev) => ({ ...prev, sort_by: value }))}
          onDirectionChange={(direction) => setQueryData((prev) => ({ ...prev, sort_direction: direction }))}
          data={["name", "quantity", "mod_rank"].map((key) => ({ label: useTranslate(`sort.${key}`), value: key }))}
        />
      </Group>
      <ScrollArea mt={"md"} className={classes.veiledRivens}>
        <SimpleGrid cols={{ base: 4 }} spacing="sm">
          {modsQuery.data?.results?.map((mod, i) => (
            <PreviewCard
              key={i}
              pos={"relative"}
              value={mod}
              headerCenter={
                <Text fz={"lg"} truncate>
                  {mod.name || mod.unique_name.split("/").pop()?.replace(/(?<!^)([A-Z])/g, " $1")}
                </Text>
              }
              renderBody={() => (
                <Group justify="center" h={"100%"}>
                  <Text fz={"lg"}>{`x${mod.quantity || 1}`}</Text>
                </Group>
              )}
              footerLeft={
                <Text fz={"lg"}>{useTranslate("mod_card.rank", { rank: mod.properties?.mod_rank ?? mod.sub_type?.rank ?? 0 })}</Text>
              }
              footerCenter={
                <Group gap={3}>
                  {mod.wfm_url && (
                    <Text fz={"sm"} c="var(--mantine-color-blue-6)">
                      WFM
                    </Text>
                  )}
                </Group>
              }
            />
          ))}
        </SimpleGrid>
      </ScrollArea>
      <Divider mt={"md"} />
      <PaginationFooter
        page={queryData.page}
        limit={queryData.limit || 50}
        total={modsQuery.data?.total || 0}
        onPageChange={(page) => setQueryData((prev) => ({ ...prev, page }))}
        onLimitChange={(limit) => setQueryData((prev) => ({ ...prev, page: 1, limit }))}
      />
    </Box>
  );
};

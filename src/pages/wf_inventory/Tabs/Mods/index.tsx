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
import { MOD_ITEM_TYPES } from "../../filters";
import { ModPreviewCard } from "./ModPreviewCard";

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
            types={MOD_ITEM_TYPES.warframe}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_rifle.png"
            text={useTranslate("filters.primary")}
            currentTypes={queryData.item_types || []}
            types={MOD_ITEM_TYPES.primary}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_melee.png"
            text={useTranslate("filters.melee")}
            currentTypes={queryData.item_types || []}
            types={MOD_ITEM_TYPES.melee}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_archwing.png"
            text={useTranslate("filters.archwing")}
            currentTypes={queryData.item_types || []}
            types={MOD_ITEM_TYPES.archwing}
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
            <ModPreviewCard key={i} mod={mod} />
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

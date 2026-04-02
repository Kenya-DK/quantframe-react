import { Box, Center, Divider, Group, Image, ScrollArea, SimpleGrid, Text } from "@mantine/core";
import { PaginationFooter } from "@components/Shared/PaginationFooter";
import { useQueries } from "./queries";
import { useMutations } from "./mutations";
import { useModals } from "./modals";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../WFInventory.module.css";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { faInfinity, faEyeSlash, faEye, faAdd } from "@fortawesome/free-solid-svg-icons";
import { upperFirst, useLocalStorage } from "@mantine/hooks";
import { ItemRiven, TauriTypes } from "$types";
import { SearchField } from "@components/Forms/SearchField";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { CompactSortSelect } from "@components/Shared/CompactSortSelect";
import { ExpandableButton } from "@components/Shared/ExpandableButton";
import { PreviewCard } from "@components/Shared/PreviewCard/PreviewCard";
import { RivenAttribute } from "@components/DataDisplay/RivenAttribute";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { RivenGrade } from "@components/DataDisplay/RivenGrade";

interface RivenPanelProps {
  isActive: boolean;
}

interface ExpandableButtonExtProps {
  img: string;
  text: string;
  currentTypes: string[];
  types: string[];
  onClick: (types: string[] | undefined) => () => void;
}

const ExpandableButtonExt: React.FC<ExpandableButtonExtProps> = ({ img, text, currentTypes, types, onClick }) => (
  <ExpandableButton
    p={3}
    collapsedWidth={35}
    icon={<Image src={img} w="25px" fit="contain" left={4} />}
    selected={types.length == 0 ? true : types.some((type) => currentTypes?.includes(type))}
    onClick={onClick(types.length == 0 ? undefined : types)}
  >
    <Text>{text}</Text>
  </ExpandableButton>
);

export const RivenPanel = ({ isActive }: RivenPanelProps) => {
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.WFItemControllerGetListParams>({
    key: "veiled_riven_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50, properties: { riven_type: "veiled" } },
  });

  // Queries
  const { veiledRivensQuery, refetchQueries } = useQueries({ queryData, isActive });

  // Mutations
  const { createMutation } = useMutations({
    refetchQueries,
    setLoadingRows: () => {},
  });

  // Modals
  const { OpenBoughtModal } = useModals({
    createMutation,
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`wf_inventory.tabs.riven.${key}`, { ...context }, i18Key);

  const HandleAddRiven = async (riven: ItemRiven) => {
    switch (riven.riven_type) {
      case "veiled":
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
        break;
      case "unveiled":
        console.warn("Unveiled rivens cannot be added directly. Please add the veiled version of the riven and then unveil it.");
        break;
      default:
        console.error("Unknown riven type:", riven.riven_type);
    }
  };

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
            img="/imgs/categories/category_rifle.png"
            text={useTranslate("filters.rifle")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Randomized/LotusRifleRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawRifleRandomMod"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_shotgun.png"
            text={useTranslate("filters.shotgun")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Randomized/LotusShotgunRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawShotgunRandomMod"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_pistol.png"
            text={useTranslate("filters.pistol")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Randomized/LotusPistolRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawPistolRandomMod"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_melee.png"
            text={useTranslate("filters.melee")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Randomized/LotusMeleeRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawMeleeRandomMod"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_modular.png"
            text={useTranslate("filters.zaw")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Randomized/LotusZawRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawZawRandomMod"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
          <ExpandableButtonExt
            img="/imgs/categories/category_archwing.png"
            text={useTranslate("filters.arch")}
            currentTypes={queryData.item_types || []}
            types={["/Lotus/Upgrades/Mods/Randomized/LotusArchgunRandomModRare", "/Lotus/Upgrades/Mods/Randomized/RawArchgunRandomMod"]}
            onClick={(types) => () => setQueryData((prev) => ({ ...prev, item_types: types }))}
          />
        </Group>
        <Group gap={3}>
          <ExpandableButton
            p={3}
            collapsedWidth={32}
            icon={<FontAwesomeIcon icon={faEyeSlash} size="xl" />}
            selected={queryData.properties?.riven_type === "veiled" || queryData.properties?.riven_type === undefined}
            onClick={() => setQueryData((prev) => ({ ...prev, properties: { ...prev.properties, riven_type: "veiled" } }))}
          >
            <Text>{useTranslate("filters.veiled")}</Text>
          </ExpandableButton>
          <ExpandableButton
            p={3}
            collapsedWidth={32}
            icon={<FontAwesomeIcon icon={faEye} size="xl" />}
            selected={queryData.properties?.riven_type === "unveiled"}
            onClick={() => setQueryData((prev) => ({ ...prev, properties: { ...prev.properties, riven_type: "unveiled" } }))}
          >
            <Text>{useTranslate("filters.unveiled")}</Text>
          </ExpandableButton>
        </Group>
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
            <PreviewCard
              key={i}
              pos={"relative"}
              value={riven}
              headerLeft={{
                hide: riven.properties?.price === undefined,
                fz: "lg",
                i18nKey: `components.riven_preview.types.${riven.riven_type}.header_left`,
                values: {
                  price: riven.properties?.price || 0,
                },
              }}
              headerCenter={{
                fz: "lg",
                i18nKey: "components.riven_preview.riven_name",
                values: { name: riven.name, mod_name: upperFirst(riven.mod_name) },
              }}
              headerRight={
                <>
                  {riven.riven_type == "veiled" && <RivenGrade value={riven.properties.grade || "unknown"} pos={"absolute"} right={10} size={35} />}
                </>
              }
              renderBody={() => (
                <>
                  {riven.riven_type == "veiled" &&
                    riven.attributes.map((attr) => (
                      <RivenAttribute key={attr.url_name} i18nKey="full" groupProps={{ p: 1 }} value={attr} hideDetails centered hideGrade />
                    ))}
                  {riven.riven_type != "veiled" && (
                    <Center h={"100%"}>
                      <Text>{riven.properties.challenge_description_with_complication}</Text>
                    </Center>
                  )}
                </>
              )}
              footerLeft={{
                fz: "lg",
                i18nKey: `components.riven_preview.types.${riven.riven_type}.footer_left`,
                values: {
                  quantity: riven.quantity || 1,
                  rank: riven.sub_type?.rank || 0,
                  required: riven.properties?.required || 0,
                  progress: riven.properties?.progress || 0,
                },
              }}
              footerCenter={
                <Group gap={3}>
                  {riven.riven_type === "veiled" && (
                    <ActionWithTooltip
                      icon={faAdd}
                      color={riven.properties?.is_in_stock ? "var(--mantine-color-green-6)" : "var(--mantine-color-red-6)"}
                      actionProps={{ size: "sm" }}
                      iconProps={{ size: "xs" }}
                      tooltip={useTranslate(`riven_card.stock_status.${riven.properties?.is_in_stock ? "found" : "not_found"}`)}
                      onClick={async () => await HandleAddRiven(riven)}
                    />
                  )}
                </Group>
              }
              footerRight={{
                hide: riven.riven_type != "veiled",
                fz: "lg",
                i18nKey: `components.riven_preview.types.${riven.riven_type}.footer_right`,
                values: { mastery: riven.mastery_rank },
              }}
            />
          ))}
        </SimpleGrid>
      </ScrollArea>
      <Divider mt={"md"} />
      <PaginationFooter
        page={queryData.page}
        limit={queryData.limit || 50}
        total={veiledRivensQuery.data?.total || 0}
        onPageChange={(page) => setQueryData((prev) => ({ ...prev, page }))}
        onLimitChange={(limit) => setQueryData((prev) => ({ ...prev, page: 1, limit }))}
      />
    </Box>
  );
};

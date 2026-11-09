import { TauriTypes } from "$types";
import { PreviewCard } from "@components/Shared/PreviewCard";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { Box, Group, Text } from "@mantine/core";
import { TextTranslate } from "../../../components/Shared/TextTranslate";
import { readableModPath } from "../filters";

type ItemProperties = {
  max_rank: number;
};
/**
 * A single rendered owned mod, extracted from the Mods tab so other WF
 * inventory surfaces (e.g. the "next" layout) can reuse it.
 */
export const ModPreviewCard = ({ mod }: { mod: TauriTypes.WFInvItemBase<ItemProperties> }) => {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`wf_inventory.tabs.mods.${key}`, { ...context }, i18Key);
  const GetProperty = (key: string): any => {
    return mod[key as keyof typeof mod] ?? mod.properties?.[key];
  };
  return (
    <PreviewCard
      pos={"relative"}
      value={mod}
      headerCenter={
        <Text fz={"lg"} truncate>
          {mod.name || readableModPath(mod.unique_name)}
        </Text>
      }
      headerRight={{ i18nKey: useTranslate("mod_card.quantity", undefined, true), size: "lg", values: { quantity: mod.quantity } }}
      renderBody={() => (
        <Group grow>
          <Group justify="flex-end">
            <Box>
              {mod.sub_type?.rank != undefined && (
                <TextTranslate
                  size="lg"
                  i18nKey={useTranslate("mod_card.rank", undefined, true)}
                  values={{ mod_rank: mod.sub_type?.rank, max_rank: GetProperty("max_rank") ?? "?" }}
                />
              )}
            </Box>
          </Group>
        </Group>
      )}
      // footerLeft={<Text fz={"lg"}>{useTranslate("mod_card.rank", { rank: mod.properties?.mod_rank ?? mod.sub_type?.rank ?? 0 })}</Text>}
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
  );
};

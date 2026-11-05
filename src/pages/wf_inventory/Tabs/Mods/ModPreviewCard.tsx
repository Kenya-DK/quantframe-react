import { Group, Text } from "@mantine/core";
import { PreviewCard } from "@components/Shared/PreviewCard";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { readableModPath } from "../../filters";
import { TauriTypes } from "$types";

/**
 * A single rendered owned mod, extracted from the Mods tab so other WF
 * inventory surfaces (e.g. the "next" layout) can reuse it.
 */
export const ModPreviewCard = ({ mod }: { mod: TauriTypes.WFInvItemBase<{ mod_rank: number }> }) => {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`wf_inventory.tabs.mods.${key}`, { ...context }, i18Key);

  return (
    <PreviewCard
      pos={"relative"}
      value={mod}
      headerCenter={
        <Text fz={"lg"} truncate>
          {mod.name || readableModPath(mod.unique_name)}
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
  );
};

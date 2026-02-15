import { memo, useEffect, useState } from "react";
import { PaperProps } from "@mantine/core";
import { RivenAttribute as RType, WFMarketTypes } from "$types/index";
import { TauriTypes } from "$types";
import { useCacheContext } from "@contexts/cache.context";
import { WithBackground } from "./WithBackground";
import { WithoutBackground } from "./WithoutBackground";
import { TextTranslateProps } from "@components/Shared/TextTranslate";
export type RivenProps = {
  weapon: TauriTypes.CacheRivenWeapon;
  polarity: string;
  modName: string;
  attributes: RType[];
  mastery: number;
  reRolls: number;
  rank: number;
  grade?: string;
};

export type RivenPreviewProps = {
  riven: WFMarketTypes.Auction | TauriTypes.StockRiven | TauriTypes.RivenSummary | TauriTypes.VeiledRiven;
  compact?: boolean;
  type: "withBackground" | "withoutBackground";
  paperProps?: PaperProps;
  headerLeft?: TextTranslateProps;
  headerRight?: TextTranslateProps;
  footerLeft?: TextTranslateProps;
  footerCenter?: TextTranslateProps;
  footerRight?: TextTranslateProps;
};

export const RivenPreview = memo(function RivenPreview(props: RivenPreviewProps) {
  // State
  const [rivenData, setRivenData] = useState<RivenProps | null>(null);
  const { weapons } = useCacheContext();
  // Fetch data from cache context
  useEffect(() => {
    const { riven } = props;
    if ("buyout_price" in riven) {
      // riven is WFMarketTypes.Auction
      console.log("Auction riven", riven);
    } else if ("wfm_weapon_id" in riven) {
      // riven is TauriTypes.StockRiven
    } else if ("stat_with_weapons" in riven) {
      let weapon = weapons.find((w) => w.unique_name === riven.unique_name);
      if (!weapon) return;
      // riven is TauriTypes.RivenSummary
      setRivenData({
        weapon: weapon,
        modName: riven.mod_name,
        attributes: riven.attributes,
        mastery: riven.mastery_rank,
        reRolls: riven.rerolls,
        polarity: riven.polarity,
        rank: riven.rank,
        grade: riven.grade,
      });
    } else if ("weapon_name" in riven) {
      let weapon = weapons.find((w) => w.unique_name === riven.unique_name);
      if (!weapon) return;
      // riven is TauriTypes.VeiledRiven
      setRivenData({
        weapon: weapon,
        modName: riven.mod_name,
        attributes: riven.attributes,
        mastery: riven.mastery_rank,
        reRolls: riven.rerolls,
        polarity: riven.polarity,
        rank: riven.rank,
        grade: riven.grade,
      });
    }
  }, [props.riven, weapons]);
  if (!rivenData) return null;
  return (
    <>
      {props.type == "withBackground" && <WithBackground {...props} value={rivenData} />}
      {props.type == "withoutBackground" && <WithoutBackground {...props} value={rivenData} />}
    </>
  );
});

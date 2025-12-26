import { Box, Collapse, PaperProps, Text } from "@mantine/core";
import classes from "./RivenPreview.module.css";
import { RivenAttribute, WFMarketTypes } from "$types/index";
import { TauriTypes } from "$types";
import { useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowsRotate } from "@fortawesome/free-solid-svg-icons";
import { useHover } from "@mantine/hooks";
import { getPolarityIcon } from "@icons";

export type RivenPreviewProps = {
  riven: WFMarketTypes.Auction | TauriTypes.StockRiven;
  paperProps?: PaperProps;
};

interface RivenAttributeWithUnits extends RivenAttribute {
  effect: string;
  units: string;
  symbol: string;
}

export function RivenPreview({ paperProps, riven }: RivenPreviewProps) {
  // State
  const { hovered, ref } = useHover();
  const [weapon, setWeapon] = useState<TauriTypes.CacheRivenWeapon | undefined>(undefined);
  const [polarity, setPolarity] = useState<string>("");
  const [modName, setModName] = useState<string>("");
  const [attributes, setAttributes] = useState<RivenAttributeWithUnits[]>([]);
  const [mastery, setMastery] = useState<number>(0);
  const [reRolls, setReRolls] = useState<number>(0);
  const [rank, setRank] = useState<number>(0);
  // Fetch data from rust side
  const { data: weapons } = useQuery<TauriTypes.CacheRivenWeapon[], Error>({
    queryKey: ["cache_riven_weapons"],
    queryFn: () => api.cache.getRivenWeapons(),
  });
  const { data: allAttributes } = useQuery<TauriTypes.CacheRivenAttribute[], Error>({
    queryKey: ["cache_riven_attributes"],
    queryFn: () => api.cache.getRivenAttributes(),
  });
  const GetUnitSymbol = (unit: string | undefined) => {
    if (!unit) return "";
    if (unit == "multiply") return "+";
    if (unit == "percent") return "%";
    if (unit == "seconds") return "sec";
    return "";
  };
  useEffect(() => {
    if (!weapons || !allAttributes) return;
    let weapon_url_name = "";
    // Check id type
    if (typeof riven.id == "string") {
      const auction = riven as WFMarketTypes.Auction;
      if (!auction.item.attributes) return;
      weapon_url_name = auction.item.weapon_url_name;
      setPolarity(auction.item.polarity);
      setModName(auction.item.name);
      setAttributes(
        auction.item.attributes?.map((item) => {
          const attribute = allAttributes?.find((attribute) => attribute.url_name == item.url_name);
          let symbol = GetUnitSymbol(attribute?.unit);
          return {
            ...item,
            effect: attribute?.short || "",
            units: attribute?.unit || "",
            symbol,
          };
        })
      );
      setMastery(auction.item.mastery_level);
      setReRolls(auction.item.re_rolls);
      setRank(auction.item.mod_rank);
    }
    if (typeof riven.id == "number") {
      // const stockRiven = riven as TauriTypes.StockRiven;
      // weapon_url_name = stockRiven.wfm_weapon_url;
      // setPolarity(stockRiven.polarity);
      // setModName(stockRiven.mod_name);
      // if (stockRiven.attributes)
      //   setAttributes(
      //     stockRiven.attributes.map((item) => {
      //       const attribute = allAttributes?.find((attribute) => attribute.url_name == item.url_name);
      //       let symbol = GetUnitSymbol(attribute?.unit);
      //       return {
      //         ...item,
      //         effect: attribute?.effect || "",
      //         units: attribute?.unit || "",
      //         symbol,
      //       };
      //     })
      //   );
      // setMastery(stockRiven.mastery_rank);
      // setReRolls(stockRiven.re_rolls);
      // setRank(stockRiven.sub_type?.rank || 0);
    }
    if (weapons && weapon_url_name != "") setWeapon(weapons.find((item) => item.wfm_url_name == weapon_url_name));
  }, [riven, weapons]);

  return (
    <Box {...paperProps} className={classes.root} ref={ref}>
      {polarity != "" && (
        <>
          {/* <FontAwesomeIcon className={classes.polarity} icon={faEnvelope} />, */}
          <FontAwesomeIcon className={classes.polarity} icon={getPolarityIcon(polarity)} />
          <Text className={classes.weapon}>{weapon?.name}</Text>
          <Text className={classes.mod_name}>{modName}</Text>
          <Box
            className={classes.attributes}
            style={{
              display: "flex",
              alignItems: "center",
              flexDirection: "column",
            }}
          >
            {attributes.map((item, index) => {
              return (
                <Text maw={"215"} truncate="end" key={index} className={classes.attribute_text}>
                  {item.units == "multiply" && `${item.symbol}`}
                  {item.value}
                  {item.units != "multiply" && `${item.symbol}`}
                  {" " + item.effect}
                </Text>
              );
            })}
          </Box>
          <Text className={classes.mastery}>MR {mastery > 16 ? 16 : mastery}</Text>
          {reRolls > 0 && (
            <Text className={classes.reroll}>
              <FontAwesomeIcon icon={faArrowsRotate} />
              <Text component="span" ml={5}>
                {reRolls}
              </Text>
            </Text>
          )}
        </>
      )}
      <Box className={classes.rank}>
        {Array.from(Array(Math.min(rank, 8))).map((_, i) => {
          return (
            <Text key={i} className={classes.circle} size="sm" component="span">
              ●
            </Text>
          );
        })}
      </Box>
      <Collapse in={hovered} className={classes.hover}>
        <Text size="sm" c="gray" className={classes.hover_text}></Text>
      </Collapse>
    </Box>
  );
}

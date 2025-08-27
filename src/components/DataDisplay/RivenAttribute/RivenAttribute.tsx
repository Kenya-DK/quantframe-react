import { Box, Text } from "@mantine/core";
import type { RivenAttribute, TauriTypes } from "$types";
import classes from "./RivenAttribute.module.css";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TextTranslate } from "../../Shared/TextTranslate";
import { useMemo } from "react";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
export type RivenAttributeProps = {
  value: RivenAttribute;
};

export function RivenAttribute({ value }: RivenAttributeProps) {
  // Helper for translating attribute names (e.g., "Range", "Critical Chance").
  const useTranslateRivenAttributeEffect = (key: string, context?: { [key: string]: any }, i18nKey?: boolean) =>
    useTranslateComponent(`riven_attribute.${key}`, { ...context }, i18nKey);

  // Fetches detailed attribute metadata (like unit types) from the cache.
  // This query runs once and its data is cached by React Query.
  const { data: cacheAttributes } = useQuery<TauriTypes.CacheRivenAttribute[]>({
    queryKey: ["cache_riven_attributes"],
    queryFn: () => api.cache.getRivenAttributes(),
    enabled: true, // Always load metadata for consistent formatting.
  });

  // Optimized lookup for attribute name maps and the current attribute's unit type.
  const { nameMap, displayUnit } = useMemo(() => {
    const map: { [key: string]: string } = {};
    let foundUnit: string | undefined = undefined;

    if (cacheAttributes) {
      cacheAttributes.forEach((item) => {
        map[item.url_name] = item.effect;
        if (item.url_name === value.url_name) {
          foundUnit = item.unit;
        }
      });
    }
    return { nameMap: map, displayUnit: foundUnit };
  }, [cacheAttributes, value.url_name]);

  // Formats the attribute's numerical value with its sign (+/-) and unit suffix (%, x).
  const absoluteValue = Math.abs(value.value);
  let numericalString: string;
  const signPrefix = value.positive ? "+" : "-";

  numericalString = `${signPrefix}${absoluteValue}`;
  if (displayUnit === "percent") {
    numericalString += "%";
  } else if (displayUnit === "multiply") {
    numericalString += "x";
  }

  // Gets the attribute name string for translation (e.g., "Range").
  const attributeDisplayName = nameMap[value.url_name] || value.effect || value.url_name;

  // Renders the full attribute phrase using TextTranslate.
  return (
    <Box data-positive={value.positive} className={classes.root}>
      <TextTranslate
        i18nKey={useTranslateRivenAttributeEffect("effect", undefined, true)} // Uses "riven_attribute.effect" as the key
        color="gray" // Sets overall text color to white
        values={{
          value: numericalString, // Injects the formatted number string
          name: attributeDisplayName, // Injects the attribute name string
        }}
        // Provides a custom component for the numerical part's color.
        components={{
          num_color_tag: <Text component="span" c={value.positive ? "var(--qf-positive-color)" : "var(--qf-negative-color)"} />,
        }}
      />
    </Box>
  );
}

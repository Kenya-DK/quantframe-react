import { Group, GroupProps, Image, Progress, Text } from "@mantine/core";
import type { RivenAttribute, TauriTypes } from "$types";
import classes from "./RivenAttribute.module.css";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect, useState } from "react";
import { LocalizedDynamicMessage } from "@components/Shared/LocalizedDynamicMessage";
import { RivenGrade } from "../RivenGrade/RivenGrade";
export type RivenAttributeProps = {
  value: RivenAttribute;
  hideDetails?: boolean;
  hideGrade?: boolean;
  compact?: boolean;
  groupProps?: GroupProps;
  centered?: boolean;
  i18nKey?: "full" | "short" | "text";
  textDecoration?: React.CSSProperties["textDecoration"];
};
export function RivenAttribute({ value, groupProps, hideDetails, hideGrade, compact, centered, i18nKey, textDecoration }: RivenAttributeProps) {
  // Fetches detailed attribute metadata (like unit types) from the cache.
  // This query runs once and its data is cached by React Query.
  const { data: cacheAttributes } = useQuery<TauriTypes.CacheRivenAttribute[]>({
    queryKey: ["cache_riven_attributes"],
    queryFn: () => api.cache.getRivenAttributes(),
    enabled: true, // Always load metadata for consistent formatting.
  });

  const [urlMapper, setUrlMapper] = useState<{ [key: string]: TauriTypes.CacheRivenAttribute }>({});

  useEffect(() => {
    if (!cacheAttributes) return;
    setUrlMapper(() =>
      cacheAttributes.reduce(
        (acc, attr) => {
          acc[attr.url_name] = attr;
          return acc;
        },
        {} as { [key: string]: TauriTypes.CacheRivenAttribute },
      ),
    );
  }, [cacheAttributes]);

  const GetValueDisplay = (v: number) => {
    return `${v > 0 ? "+" : ""}${v.toFixed(value.url_name.includes("damage_vs") ? 2 : 1).replace(/\.0$/, "")}`;
  };
  const calculateProgress = (min: number, current: number, max: number) => {
    return ((current - min) / (max - min)) * 100;
  };
  const getLocalizedText = () => {
    if (value.localized_text) return value.localized_text;
    if (!urlMapper[value.url_name]) return value.url_name;
    return urlMapper[value.url_name][i18nKey || "full"];
  };
  return (
    <Group
      className={classes.root}
      data-compact={compact ? "true" : "false"}
      data-hide-details={hideDetails ? "true" : "false"}
      data-positive={value.positive}
      gap={compact ? "sm" : "md"}
      p={compact ? "2" : "8px 12px"}
      {...groupProps}
    >
      <Group flex={1} style={{ justifyContent: centered ? "center" : "flex-start" }}>
        {!hideGrade && value.properties?.grade && <RivenGrade value={value.properties.grade} size={25} />}
        <LocalizedDynamicMessage
          data-hide-details={hideDetails ? "true" : "false"}
          textProps={{
            size: "md",
            fw: 600,
            td: textDecoration,
            lh: "1.2rem",
            className: classes.attributeText,
            "data-hide-details": hideDetails ? "true" : "false",
          }}
          tokens={[
            {
              pattern: /\|STAT1\|/g,
              render: () => GetValueDisplay(value.value) || "0",
            },
            {
              pattern: /\|val\|/g,
              render: () => GetValueDisplay(value.value) || "0",
            },
            {
              pattern: /<([A-Z0-9_]+)>/,
              render: (m) => <Image src={`/damageTypes/${m[1]}.png`} h={16} w="auto" fit="contain" mr={2} />,
            },
          ]}
          message={getLocalizedText()}
        />
      </Group>
      {!hideDetails && (
        <Group gap="md" style={{ flex: "0 0 auto" }}>
          <Group gap={0}>
            <Text size="md" fw={700} w={10} ta="center">
              {value.properties?.letter_grade}
            </Text>

            <Text size="sm" fw={500} w={50} ta="right">
              {value.properties?.min?.toFixed(1).replace(/\.0$/, "")}
            </Text>
          </Group>

          {value.properties?.min !== undefined && value.value !== undefined && value.properties?.max !== undefined && (
            <Progress
              classNames={{
                root: classes.progressRoot,
                section: classes.progressSection,
              }}
              w={200}
              value={calculateProgress(value.properties.min, value.value, value.properties.max)}
              size="lg"
              radius="md"
            />
          )}
          <Text size="sm" fw={500} w={40} ta="left">
            {value.properties?.max?.toFixed(1).replace(/\.0$/, "")}
          </Text>
        </Group>
      )}
    </Group>
  );
}

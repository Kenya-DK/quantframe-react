import { Group, Image, Progress, Text } from "@mantine/core";
import type { RivenAttribute, TauriTypes } from "$types";
import classes from "./RivenAttribute.module.css";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect, useState } from "react";
import { LocalizedDynamicMessage } from "@components/Shared/LocalizedDynamicMessage";
export type RivenAttributeProps = {
  value: RivenAttribute;
  hideDetails?: boolean;
  compact?: boolean;
};
const IMAGE_SIZE = 25;
const grades: Record<string, React.ReactNode> = {
  decisive: <Image src="/grades/gradePerfect.png" h={IMAGE_SIZE} w="auto" fit="contain" />,
  good: <Image src="/grades/gradeGreen.png" h={IMAGE_SIZE} w="auto" fit="contain" />,
  not_helping: <Image src="/question.png" h={IMAGE_SIZE} w="auto" fit="contain" />,
  bad: <Image src="/grades/gradeRed.png" h={IMAGE_SIZE} w="auto" fit="contain" />,
  unknown: <Image src="/question.png" h={IMAGE_SIZE} w="auto" fit="contain" />,
};
export function RivenAttribute({ value, hideDetails, compact }: RivenAttributeProps) {
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
      cacheAttributes.reduce((acc, attr) => {
        acc[attr.url_name] = attr;
        return acc;
      }, {} as { [key: string]: TauriTypes.CacheRivenAttribute })
    );
  }, [cacheAttributes]);

  const GetValueDisplay = (value: number) => {
    return `${value > 0 ? "+" : ""}${value.toFixed(1).replace(/\.0$/, "")}`;
  };
  const calculateProgress = (min: number, current: number, max: number) => {
    return ((current - min) / (max - min)) * 100;
  };
  return (
    <Group
      className={classes.root}
      data-compact={compact ? "true" : "false"}
      data-hide-details={hideDetails ? "true" : "false"}
      data-positive={value.positive}
      gap={compact ? "sm" : "md"}
      p={compact ? "2" : "8px 12px"}
    >
      <Group gap="xs" style={{ flex: 1 }}>
        {value.grade && grades[value.grade]}
        <LocalizedDynamicMessage
          data-hide-details={hideDetails ? "true" : "false"}
          textProps={{ size: "md", fw: 600, className: classes.attributeText, "data-hide-details": hideDetails ? "true" : "false" }}
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
          message={urlMapper[value.url_name]?.full || value.url_name}
        />
      </Group>
      {!hideDetails && (
        <Group gap="md" style={{ flex: "0 0 auto" }}>
          <Group gap={0}>
            <Text size="md" fw={700} w={10} ta="center">
              {value.letterGrade}
            </Text>

            <Text size="sm" fw={500} w={50} ta="right">
              {value.minValue?.toFixed(1).replace(/\.0$/, "")}
            </Text>
          </Group>

          {value.minValue !== undefined && value.value !== undefined && value.maxValue !== undefined && (
            <Progress
              classNames={{
                root: classes.progressRoot,
                section: classes.progressSection,
              }}
              w={200}
              value={calculateProgress(value.minValue, value.value, value.maxValue)}
              size="lg"
              radius="md"
            />
          )}
          <Text size="sm" fw={500} w={40} ta="left">
            {value.maxValue?.toFixed(1).replace(/\.0$/, "")}
          </Text>
        </Group>
      )}
    </Group>
  );
}

import { Box, Group, Image, Progress, useMantineTheme, Text } from "@mantine/core";
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

export function RivenAttribute({ value, hideDetails, compact }: RivenAttributeProps) {
  const theme = useMantineTheme();

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
    return `${value > 0 ? "+" : ""}${value}`;
  };
  const calculateProgress = (min: number, current: number, max: number) => {
    return ((current - min) / (max - min)) * 100;
  };
  return (
    <Group
      className={classes.root}
      data-positive={value.positive}
      gap={compact ? "sm" : "md"}
      p={compact ? "2" : "4"}
      // bg={value.positive ? theme.other.riven.positiveTrait : theme.other.riven.negativeTrait}
      style={{ borderRadius: "20px" }}
    >
      <Group gap="xs" style={{ flex: 1 }}>
        {value.grade && <Image src="/DT_CORROSIVE_COLOR.png" h={28} w="auto" fit="contain" />}
        {value.grade && <Box w={24} />}
        <LocalizedDynamicMessage
          textProps={{ size: "md", fw: 600, color: value.positive ? theme.other.riven.positiveTrait : theme.other.riven.negativeTrait }}
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
            <Text size="xs" fw={600} w={10} ta="center">
              {value.letterGrade}
            </Text>

            <Text size="sm" fw={500} w={40} ta="right">
              {value.minValue}
            </Text>
          </Group>

          {value.minValue !== undefined && value.value !== undefined && value.maxValue !== undefined && (
            <Progress
              w={200}
              value={calculateProgress(value.minValue, value.value, value.maxValue)}
              size="lg"
              color={value.positive ? theme.other.riven.positiveLight : theme.other.riven.negativeLight}
              radius="md"
            />
          )}
          <Text size="sm" fw={500} w={40} ta="left">
            {value.maxValue}
          </Text>
        </Group>
      )}
    </Group>
  );
}

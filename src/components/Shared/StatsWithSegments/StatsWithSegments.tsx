import { Box, Group, NumberFormatter, Progress, SimpleGrid, Text, Tooltip, Flex, StyleProp, MantineSpacing } from "@mantine/core";
import classes from "./StatsWithSegments.module.css";
import React from "react";

export type Segment = {
  label: string;
  count: number | string;
  color: string;
  hide?: boolean;
  decimalScale?: number;
  part?: number | string | null;
  tooltip?: string;
  hideInProgress?: boolean;
  prefix?: string;
  suffix?: string;
};
export type StatsWithSegmentsProps = {
  p?: StyleProp<MantineSpacing>;
  showPercent?: boolean;
  hidePercentBar?: boolean;
  segments: Segment[];
  percentSymbol?: string;
  orientation?: "horizontal" | "vertical";
  header?: React.ReactNode;
  footer?: React.ReactNode;
  h?: StyleProp<React.CSSProperties["height"]>;
};

export function StatsWithSegments({
  p = "md",
  segments: segmentsIn,
  hidePercentBar,
  showPercent,
  percentSymbol,
  header,
  footer,
  h,
  orientation = "horizontal",
}: StatsWithSegmentsProps) {
  const total = segmentsIn
    .filter((segment) => !segment.hideInProgress && !segment.hide)
    .reduce((sum, segment) => sum + Math.abs(Number(segment.count) || 0), 0);

  // const getPercentage = (segment: Segment) => (segment.part ? segment.part : total ? Math.round((Math.abs(segment.count ) / total) * 100) : 0);

  const getPercentage = (segment: Segment) => {
    if (typeof segment.part === "number" || segment.part == null || segment.part === undefined)
      return segment.part ? segment.part : total ? Math.round((Math.abs(Number(segment.count) || 0) / total) * 100) : 0;
    else if (typeof segment.part === "string") {
      const partValue = Number(segment.part) || 0;
      return partValue ? partValue : total ? Math.round((Math.abs(Number(segment.count) || 0) / total) * 100) : 0;
    } else return 0;
  };

  const progressSegments = segmentsIn
    .filter((segment) => !segment.hideInProgress && !segment.hide)
    .map((segment) => {
      const percentage = getPercentage(segment);
      return (
        <Progress.Section value={percentage} color={segment.color} key={segment.label}>
          {percentage > 10 && (
            <Progress.Label>
              <NumberFormatter value={percentage} className={classes.statCount} decimalScale={0} suffix="%" />
            </Progress.Label>
          )}
        </Progress.Section>
      );
    });

  const descriptions = segmentsIn
    .filter((segment) => !segment.hide)
    .map((stat) => (
      <Box key={stat.label} style={{ borderBottomColor: stat.color }} className={classes.stat}>
        <Text tt="uppercase" fz="xs" c="dimmed" fw={700}>
          {stat.label}
        </Text>

        <Group justify="space-between" align="flex-end" gap={0}>
          <Text fw={700}>{typeof stat.count === "number" ? <NumberFormatter value={stat.count} thousandSeparator /> : stat.count}</Text>
          {showPercent && stat.part !== null && (
            <Tooltip label={stat.tooltip} withArrow disabled={!stat.tooltip}>
              <span>
                {typeof stat.part === "number" ? (
                  <NumberFormatter
                    value={getPercentage(stat)}
                    className={classes.statCount}
                    decimalScale={stat.decimalScale ?? 0}
                    style={{ color: stat.color }}
                    prefix={stat.prefix}
                    suffix={stat.suffix ?? percentSymbol}
                    thousandSeparator=","
                    thousandsGroupStyle="thousand"
                  />
                ) : (
                  <Text className={classes.statCount} c={stat.color}>
                    {stat.part}
                  </Text>
                )}
              </span>
            </Tooltip>
          )}
        </Group>
      </Box>
    ));

  const IsEmpty = () => segmentsIn.filter((segment) => segment.hideInProgress !== false).length === 0;

  const isVertical = orientation === "vertical";

  return (
    <Box p={p}>
      <Flex direction={isVertical ? "row" : "column"} gap="md" align={isVertical ? "flex-start" : "stretch"}>
        {showPercent && !hidePercentBar && (
          <Box style={{ position: "relative", flex: isVertical ? "0 0 auto" : "1" }} className={classes.progressWrapper}>
            <Progress.Root size={34} h={h} data-empty={IsEmpty()} classNames={{ label: classes.progressLabel }} orientation={orientation}>
              {progressSegments}
            </Progress.Root>
          </Box>
        )}

        <SimpleGrid cols={isVertical ? 1 : { base: 3, md: segmentsIn.length }} style={{ flex: isVertical ? 1 : "auto" }}>
          {header && <Box>{header}</Box>}
          {descriptions}
          {footer && <Box>{footer}</Box>}
        </SimpleGrid>
      </Flex>
    </Box>
  );
}

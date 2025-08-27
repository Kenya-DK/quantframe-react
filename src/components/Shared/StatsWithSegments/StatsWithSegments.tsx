import { Box, Group, NumberFormatter, Progress, SimpleGrid, Text, Tooltip } from "@mantine/core";
import classes from "./StatsWithSegments.module.css";

export type Segment = {
  label: string;
  count: number;
  color: string;
  part?: number;
  tooltip?: string;
};
export type StatsWithSegmentsProps = {
  showPercent?: boolean;
  hidePercentBar?: boolean;
  segments: Segment[];
  percentSymbol?: string;
};

export function StatsWithSegments({ segments: segmentsIn, hidePercentBar, showPercent, percentSymbol }: StatsWithSegmentsProps) {
  const total = segmentsIn.reduce((acc, curr) => acc + Math.abs(curr.count), 0);

  const getPercentage = (segment: Segment) => (segment.part ? segment.part : total ? Math.round((Math.abs(segment.count) / total) * 100) : 0);

  const progressSegments = segmentsIn.map((segment) => {
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

  const descriptions = segmentsIn.map((stat) => (
    <Box key={stat.label} style={{ borderBottomColor: stat.color }} className={classes.stat}>
      <Text tt="uppercase" fz="xs" c="dimmed" fw={700}>
        {stat.label}
      </Text>

      <Group justify="space-between" align="flex-end" gap={0}>
        <Text fw={700}>
          <NumberFormatter value={stat.count} thousandSeparator />
        </Text>
        {showPercent && (
          <Tooltip label={stat.tooltip} withArrow disabled={!stat.tooltip}>
            <span>
              <NumberFormatter
                value={getPercentage(stat)}
                className={classes.statCount}
                decimalScale={0}
                style={{ color: stat.color }}
                suffix={percentSymbol}
              />
            </span>
          </Tooltip>
        )}
      </Group>
    </Box>
  ));

  const IsEmpty = () => segmentsIn.every((segment) => segment.count === 0);

  return (
    <Box p="md">
      {showPercent && !hidePercentBar && (
        <Box style={{ position: "relative" }} className={classes.progressWrapper}>
          <Progress.Root size={34} data-empty={IsEmpty()} classNames={{ label: classes.progressLabel }}>
            {progressSegments}
          </Progress.Root>
        </Box>
      )}

      <SimpleGrid cols={{ base: 3, md: segmentsIn.length }} mt="xl">
        {descriptions}
      </SimpleGrid>
    </Box>
  );
}

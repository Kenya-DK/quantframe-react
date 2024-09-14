import { Box, Group, NumberFormatter, Progress, SimpleGrid, Text } from '@mantine/core';
import classes from './StatsWithSegments.module.css';

export type Segment = {
	label: string;
	count: number;
	part: number;
	color: string;
}
export type StatsWithSegmentsProps = {
	segments: Segment[];
}

export function StatsWithSegments({ segments: segmentsIn }: StatsWithSegmentsProps) {
	const segments = segmentsIn.map((segment) => (
		<Progress.Section value={segment.part} color={segment.color} key={segment.color}>
			{segment.part > 10 && <Progress.Label><NumberFormatter value={segment.part} className={classes.statCount} decimalScale={0} suffix='%' /></Progress.Label>}
		</Progress.Section>
	));

	const descriptions = segmentsIn.map((stat) => (
		<Box key={stat.label} style={{ borderBottomColor: stat.color }} className={classes.stat}>
			<Text tt="uppercase" fz="xs" c="dimmed" fw={700}>
				{stat.label}
			</Text>

			<Group justify="space-between" align="flex-end" gap={0}>
				<Text fw={700}>{stat.count}</Text>
				<NumberFormatter value={stat.part} className={classes.statCount} decimalScale={0} style={{ color: stat.color }} suffix='%' />
			</Group>
		</Box>
	));
	return (
		<Box p="md">
			<Progress.Root size={34} classNames={{ label: classes.progressLabel }} >
				{segments}
			</Progress.Root>
			<SimpleGrid cols={{ base: 1, xs: 3 }} mt="xl">
				{descriptions}
			</SimpleGrid>
		</Box>
	);
}
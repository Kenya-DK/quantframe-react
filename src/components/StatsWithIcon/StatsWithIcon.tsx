import { Paper, Text, Center, Grid, Stack, Title, Divider, Group } from '@mantine/core';
import classes from './StatsWithIcon.module.css';

export type StatsWithIconProps = {
	count: number | string;
	color: string;
	title: string;
	icon: React.ReactNode;
	footer?: React.ReactNode;
}

export function StatsWithIcon({ count, color, footer, title, icon }: StatsWithIconProps) {

	return (
		<Paper className={classes.root}>
			<Grid p={15}>
				<Grid.Col span={6}>
					<Center className={classes.icon}
						display="flex"
						style={{ background: color }}>
						{icon}
					</Center>
				</Grid.Col>
				<Grid.Col display="flex" style={{ justifyContent: "flex-end" }} span={6}>
					<Stack align="center" gap={"1"} >
						<Title order={5}>{title}</Title>
						<Text variant="body1" c="text.secondary">
							{count}
						</Text>
					</Stack>
				</Grid.Col>
			</Grid>
			{footer && (<Divider />)}
			{footer && (
				<Group p={15}  >
					{footer}
				</Group>
			)}
		</Paper>
	);
}
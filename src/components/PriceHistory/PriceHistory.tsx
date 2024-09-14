import { Group, Paper, Text } from '@mantine/core';
import dayjs from 'dayjs';
import classes from './PriceHistory.module.css';
import { PriceHistory } from '@api/types';
export type PriceHistoryListItemProps = {
	history: PriceHistory;
}

export function PriceHistoryListItem({ history }: PriceHistoryListItemProps) {

	return (
		<Paper mt={5} classNames={classes} p={5}>
			<Group justify="space-between" >
				<Group w={100}>
					<Text c="blue.5">{history.price} </Text>
				</Group>
				<Group justify="right">
					<Text c="gray.4">{dayjs(history.created_at).format("DD/MM/YYYY HH:mm:ss")}</Text>
				</Group>
			</Group>
		</Paper>
	);
}
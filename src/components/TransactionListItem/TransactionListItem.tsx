import { Group, Paper, Text } from '@mantine/core';
import { TransactionDto, TransactionItemType } from '@api/types';
import dayjs from 'dayjs';
import classes from './TransactionListItem.module.css';
import { GetSubTypeDisplay } from '@utils/helper';
export type TransactionListItemProps = {
	transaction: TransactionDto;
}

export function TransactionListItem({ transaction }: TransactionListItemProps) {
	// Functions
	const GetTransactionDisplay = () => {
		switch (transaction.item_type) {
			case TransactionItemType.Riven:
				return transaction.item_name + " " + transaction.properties["mod_name"];
			case TransactionItemType.Item:
				return transaction.item_name;
		}
	};
	return (
		<Paper mt={5} classNames={classes} p={5} data-trade-type={transaction.transaction_type} data-color-mode='box-shadow'>
			<Group justify="space-between" >
				<Group ml={10} gap={"sm"} w={"50%"}>
					<Text c="gray.4">{GetTransactionDisplay()}</Text>
					{transaction.quantity > 1 && <Text c="gray.4">{transaction.quantity}x</Text>}
					<Text c="blue.5">{GetSubTypeDisplay(transaction.sub_type)} </Text>
				</Group>
				<Group w={100}>
					<Text c="blue.5">{transaction.price} </Text>
				</Group>
				<Group justify="right">
					<Text c="gray.4">{dayjs(transaction.created_at).format("DD/MM/YYYY HH:mm:ss")}</Text>
				</Group>
			</Group>
		</Paper>
	);
};
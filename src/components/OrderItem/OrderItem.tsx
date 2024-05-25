import { Image, Group, Paper, Stack, Divider, Text, Avatar, Title } from '@mantine/core';
import classes from './OrderItem.module.css';
import { Wfm } from '$types/index';
import { WFMThumbnail } from '@api/index';
import { useTranslateComponent, useTranslateEnums } from '@hooks/index';
import { TextTranslate } from '../TextTranslate';

export type OrderItemProps = {
	order: Wfm.OrderDto;
	show_user?: boolean;
	footer?: React.ReactNode;
}

export function OrderItem({ order, footer, show_user }: OrderItemProps) {

	// Translate general
	const useTranslateStockItemInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`order_item.${key}`, { ...context }, i18Key)
	const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateStockItemInfo(`fields.${key}`, { ...context }, i18Key)
	const useTranslateUserStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateEnums(`user_status.${key}`, { ...context }, i18Key)

	return (
		<Paper mt={5} classNames={classes} p={7} data-order-type={order.order_type}>
			<Stack gap={3}>
				<Group justify='space-between'>
					<Group>
						<Title order={4}>{order.item?.en?.item_name}</Title>
					</Group>
					<Group>
						<TextTranslate i18nKey={useTranslateFields("quantity", undefined, true)} values={{ quantity: order.quantity }} />
					</Group>
				</Group>
				<Divider />
				<Group justify='space-between'>
					<Group>
						<Image width={48} height={48} fit="contain" src={WFMThumbnail(order.item?.icon || "")} />
						<Text size="md">{order.platinum}</Text>
					</Group>
					<Group>

					</Group>
				</Group>
				{(show_user || footer) && <Divider />}
				<Group align='center' grow>
					{show_user && (
						<Group>
							<Avatar size={"sm"} src={WFMThumbnail(order.user.avatar || "https://cataas.com/cat")} alt="no image here" />
							<Group>
								<Text> {order.user.ingame_name}</Text>
								<Text data-color-mode='text' data-user-status={order.user.status}> {useTranslateUserStatus(order.user.status)}</Text>
							</Group>
						</Group>
					)}
					{footer}
				</Group>
			</Stack>
		</Paper>
	);
}

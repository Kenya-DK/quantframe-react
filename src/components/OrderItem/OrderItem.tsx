import { Image, Group, Paper, Stack, Divider, Text, Avatar, Rating, Box, useMantineTheme, PaperProps } from '@mantine/core';
import classes from './OrderItem.module.css';
import { Wfm } from '$types/index';
import { WFMThumbnail } from '@api/index';
import { useTranslateComponent, useTranslateEnums } from '@hooks/index';
import { TextTranslate } from '../TextTranslate';
import { upperFirst } from '@mantine/hooks';
import { SvgIcon, SvgType } from '../SvgIcon';

export type OrderItemProps = {
	order: Wfm.OrderDto;
	show_user?: boolean;
	footer?: React.ReactNode;
	show_border?: boolean;
	paperProps?: PaperProps;
}

export function OrderItem({ show_border, paperProps, order, footer, show_user }: OrderItemProps) {
	// State
	const theme = useMantineTheme();


	// Translate general
	const useTranslateStockItemInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`order_item.${key}`, { ...context }, i18Key)
	const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateStockItemInfo(`fields.${key}`, { ...context }, i18Key)
	const useTranslateUserStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateEnums(`user_status.${key}`, { ...context }, i18Key)

	return (
		<Paper {...paperProps} classNames={classes} p={7} data-border={show_border} data-color-mode='box-shadow' data-order-type={order.order_type}>
			<Stack gap={3}>
				<Group ml={"xs"} justify='space-between'>
					<Group>
						<Text size='lg' fw={700}>{order.item?.en?.item_name}</Text>
					</Group>
					<Group>
						<TextTranslate size='md' i18nKey={useTranslateFields("quantity", undefined, true)} values={{ quantity: order.quantity }} />
					</Group>
				</Group>
				<Divider />
				<Group align="center" grow p={"sm"}>
					<Group >
						<Image w={"50%"} ml={"sm"} width={64} height={64} fit="contain" src={WFMThumbnail(order.item?.icon || "")} />
					</Group>
					<Group justify="flex-end">
						<Box>
							{(order.mod_rank != undefined) && (<TextTranslate size='lg' i18nKey={useTranslateFields("mod_rank", undefined, true)} values={{ mod_rank: order.mod_rank, mod_max_rank: order.item?.mod_max_rank || 0 }} />)}
							{(order.amber_stars != undefined) && (<Rating fullSymbol={<SvgIcon svgProp={{ width: 16, height: 16, fill: theme.colors.blue[5] }} iconType={SvgType.Default} iconName={"amber_star"} />} value={order.amber_stars} count={order.amber_stars} readOnly />)}
							{(order.cyan_stars != undefined) && (<Rating fullSymbol={<SvgIcon svgProp={{ width: 16, height: 16, fill: theme.colors.yellow[7] }} iconType={SvgType.Default} iconName={"cyan_star"} />} value={order.cyan_stars} count={order.cyan_stars} readOnly />)}
							{(order.subtype) && (<TextTranslate size='lg' i18nKey={useTranslateFields("subtype", undefined, true)} values={{ sub_type: order.subtype ? `${upperFirst(order.subtype)}` : "" }} />)}
						</Box>
					</Group>
				</Group>
				<Divider />
				<Group align='center' grow p={3}>
					<Group>
						<TextTranslate size='lg' i18nKey={useTranslateFields("platinum", undefined, true)} values={{ platinum: order.platinum }} />
					</Group>
					<Group gap={"sm"} justify="flex-end">
						{footer}
					</Group>
				</Group>
				{show_user && (
					<Group>
						<Avatar size={"sm"} src={WFMThumbnail(order.user.avatar || "https://cataas.com/cat")} alt="no image here" />
						<Group>
							<Text> {order.user.ingame_name}</Text>
							<Text data-color-mode='text' data-user-status={order.user.status}> {useTranslateUserStatus(order.user.status)}</Text>
						</Group>
					</Group>
				)}
			</Stack>
		</Paper>
	);
}

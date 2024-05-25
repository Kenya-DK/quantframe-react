import { Image, Group, Paper, Stack, Divider, Text, Avatar, Title } from '@mantine/core';
import classes from './OrderItem.module.css';
import { ActionWithTooltip } from '../ActionWithTooltip';
import { faCartShopping, faFilter, faPen, faTrashCan } from '@fortawesome/free-solid-svg-icons';
import { Wfm } from '../../types';


export type OrderItemProps = {
	order: Wfm.OrderDto;
}

export function OrderItem({ order }: OrderItemProps) {
	// State

	return (
		<Paper mt={5} classNames={classes} p={7} data-order-type={order.order_type}>
			<Stack gap={3}>
				<Group justify='space-between'>
					<Group>
						<Title order={4}>{order.item?.en.item_name}</Title>
					</Group>
					<Group>
						<ActionWithTooltip
							tooltip='Mark as favorite'
							icon={faFilter}
							onClick={() => { }}
						/>
					</Group>
				</Group>
				<Divider />
				<Group justify='space-between'>
					<Group>
						<Image width={48} height={48} fit="contain" src={"https://warframe.market/static/assets/items/images/en/thumbs/glaive.7b1c251b4f1fcc0afe64ce5233a39891.128x128.png"} />
						<Text size="md">{order.platinum}</Text>
					</Group>
					<Group>

					</Group>
				</Group>
				<Divider />
				<Group align='center' grow>
					<Group>
						<Avatar size={"sm"} src={"https://warframe.market/static/assets/user/avatar/5b855e84f7af4800475db904.png?72ba463726786a0826b0bbd622675f1c"} alt="no image here" />
					</Group>
					<Group gap={"xs"} justify='flex-end'>
						<ActionWithTooltip
							tooltip='Mark as favorite'
							color="blue.7"
							icon={faPen}
							onClick={() => { }}
						/>
						<ActionWithTooltip
							tooltip='Mark as favorite'
							color="green.7"
							icon={faCartShopping}
							onClick={() => { }}
						/>
						<ActionWithTooltip
							tooltip='Mark as favorite'
							color="red.7"
							icon={faTrashCan}
							onClick={() => { }}
						/>
					</Group>
				</Group>
			</Stack>
		</Paper>
	);
}

export interface Auction {
	buyout_price: number;
	note: string;
	visible: boolean;
	item: Item;
	starting_price: number;
	minimal_reputation: number;
	owner: Owner;
	platform: string;
	closed: boolean;
	top_bid: null;
	winner: null;
	is_marked_for: null;
	marked_operation_at: null;
	created: Date;
	updated: Date;
	note_raw: string;
	is_direct_sell: boolean;
	id: string;
	private: boolean;
}

export interface Item {
	type: string;
	mod_rank: number;
	weapon_url_name: string;
	attributes: Attribute[];
	name: string;
	re_rolls: number;
	polarity: string;
	mastery_level: number;
}

export interface Attribute {
	value: number;
	positive: boolean;
	url_name: string;
}

export interface Owner {
	reputation: number;
	locale: string;
	avatar: string;
	last_seen: Date;
	ingame_name: string;
	status: string;
	id: string;
	region: string;
}

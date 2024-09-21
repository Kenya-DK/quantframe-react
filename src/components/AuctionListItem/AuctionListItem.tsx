import { PaperProps, Image } from '@mantine/core';
import { useEffect, useState } from 'react';
import { Wfm } from '$types/index';
import { AuctionRivenListItem } from '../AuctionListRivenItem';


export type AuctionListItemItemProps = {
	compacted?: boolean;
	showOwner?: boolean;
	auction: Wfm.Auction<any>;
	show_border?: boolean;
	show_image?: boolean;
	header?: React.ReactNode;
	paperProps?: PaperProps;
	overrideMode?: Wfm.AuctionStatus;
}
export function AuctionListItem(props: AuctionListItemItemProps) {
	// Props
	const { auction } = props;
	// State
	const [itemType, setItemType] = useState<Wfm.AuctionItemType | undefined>(undefined);

	useEffect(() => {

switch (auction.item.type) {
			case Wfm.AuctionItemType.Riven:
				setItemType(Wfm.AuctionItemType.Riven);
				break;
			case Wfm.AuctionItemType.Lich:
				setItemType(Wfm.AuctionItemType.Lich);
				break;
			case Wfm.AuctionItemType.Sister:
				setItemType(Wfm.AuctionItemType.Sister);
				break;
		}
	}, [auction]);

	return (
		<>
			{itemType === Wfm.AuctionItemType.Riven && (<AuctionRivenListItem {...props} />)}
			{itemType === Wfm.AuctionItemType.Sister && (<Image fit="contain" w="auto" h={150} radius="md" src={"https://cataas.com/cat"} />)}
			{itemType === Wfm.AuctionItemType.Lich && (<Image fit="contain" w="auto" h={150} radius="md" src={"https://cataas.com/cat"} />)}
		</>
	);
}
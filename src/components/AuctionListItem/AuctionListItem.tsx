import { Image, Group, Paper, Stack, Divider, Text, Avatar, Box, Skeleton } from '@mantine/core';
import classes from './AuctionListItem.module.css';
import { ActionWithTooltip } from '../ActionWithTooltip';
import { faFilter } from '@fortawesome/free-solid-svg-icons';
import { useEffect, useState } from 'react';
import { Wfm } from '$types/index';
import api, { WFMThumbnail } from '@api/index';
import { RivenAttributeCom, SvgIcon, SvgType, TextTranslate } from '@components';
import { useQuery } from '@tanstack/react-query';
import { CacheRivenWeapon } from '@api/types';
import { useTranslateComponent, useTranslateEnums } from '@hooks/index';
import { getCssVariable } from '../../utils';
export type AuctionListItemItemProps = {
	compacted?: boolean;
	auction: Wfm.Auction<any>;
}
export function AuctionListItem({ compacted, auction }: AuctionListItemItemProps) {
	// State
	const [mode, setMode] = useState('public');
	const [weapon, setWeapon] = useState<CacheRivenWeapon | undefined>(undefined);

	// Translate general
	const useTranslateBase = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`auction_list_item.${key}`, { ...context }, i18Key)
	const useTranslateUserStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateEnums(`user_status.${key}`, { ...context }, i18Key)

	// Fetch data from rust side
	const { data, isFetching } = useQuery({
		queryKey: ['cache_riven_weapons'],
		queryFn: () => api.cache.getRivenWeapons(),
	})

	useEffect(() => {
		if (auction.private) {
			setMode('private');
		} else
			setMode('private');

		if (data)
			setWeapon(data.find((item) => item.wfm_url_name == auction.item.weapon_url_name));
	}, [auction, data]);

	return (
		<Paper mt={5} classNames={classes} p={5} data-mode={mode}>
			<Skeleton visible={isFetching} >
				{compacted ? (
					<Group>
						<TextTranslate i18nKey={useTranslateBase("weapon_name", undefined, true)} values={{
							weapon: weapon?.i18n["en"].name || "",
							mod_name: auction.item.name
						}} />
						<TextTranslate i18nKey={useTranslateBase("footer", undefined, true)} values={{
							mastery_level: auction.item.mastery_level,
							mod_rank: auction.item.mod_rank,
							re_rolls: auction.item.re_rolls,
							polarity: auction.item.polarity,
						}} />

					</Group>
				) : (
					<Stack gap={3}>
						<Group justify='space-between'>
							<Group>
								<Image width={32} height={32} fit="contain" src={WFMThumbnail(weapon?.wfm_icon || "")} />
								<TextTranslate textProps={{
									fw: 700,
									fs: "lg",
								}} color='gray.4' i18nKey={useTranslateBase("weapon_name", undefined, true)} values={{
									weapon: weapon?.i18n["en"].name || "",
									mod_name: auction.item.name
								}} />
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
								<Box>
									<Group mt={"sm"}>
										{auction.item.attributes.map((attr, index) => <RivenAttributeCom key={index} value={attr} />)}
									</Group>
									<Group mt={"sm"}>
										<TextTranslate i18nKey={useTranslateBase("footer", undefined, true)} values={{
											mastery_level: auction.item.mastery_level,
											mod_rank: auction.item.mod_rank,
											re_rolls: auction.item.re_rolls,
										}}
											components={{
												"polarity": <SvgIcon svgProp={{ width: 16, height: 16, fill: getCssVariable("--mantine-color-gray-7") }} iconType={SvgType.Polarity} iconName={auction.item.polarity} />
											}}
										/>
									</Group>
								</Box>
							</Group>
							<Group>
								{auction.is_direct_sell ?
									(
										<Text size={"sm"}>Selling price: {auction.starting_price}</Text>
									) : (
										<Group>
											<Box style={{ display: "flex", flexDirection: "column", alignItems: "flex-end" }}>
												<Text size={"sm"}>Buyout Price: {auction.buyout_price}</Text>
												<Text size={"sm"}>Starting Price: {auction.starting_price}</Text>
											</Box>
											<Group>
												<Text size={"sm"}>Top bid: {auction.top_bid || 0}</Text>
											</Group>
										</Group>
									)}
							</Group>
						</Group>
						<Divider />
						<Group align='center'>
							<Avatar size={"sm"} src={auction.owner.avatar ? WFMThumbnail(auction.owner.avatar) : "https://cataas.com/cat"} alt="no image here" />
							<Group>
								<Text> {auction.owner.ingame_name}</Text>
								<Text data-color-mode='text' data-user-status={auction.owner.status}> {useTranslateUserStatus(auction.owner.status)}</Text>
							</Group>
						</Group>
					</Stack>
				)}
			</Skeleton>
		</Paper>
	);
}
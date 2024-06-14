import { Box, Collapse, PaperProps, Text } from '@mantine/core';
import classes from './RivenPreview.module.css';
import { Wfm } from '$types/index';
import { CacheRivenWeapon, CacheRivenAttribute, RivenAttribute, StockRiven } from '@api/types';
import { useEffect, useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import api from '@api/index';
import { SvgIcon, SvgType } from '../SvgIcon';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faArrowsRotate } from '@fortawesome/free-solid-svg-icons';
import { useHover } from '@mantine/hooks';

export type RivenPreviewProps = {
	riven: Wfm.Auction<Wfm.AuctionOwner> | Wfm.Auction<string> | StockRiven;
	paperProps?: PaperProps;
}

interface RivenAttributeWithUnits extends RivenAttribute {
	effect: string;
	units: string;
	symbol: string;
}

export function RivenPreview({ paperProps, riven }: RivenPreviewProps) {
	// State
	const { hovered, ref } = useHover();
	const [weapon, setWeapon] = useState<CacheRivenWeapon | undefined>(undefined);
	const [polarity, setPolarity] = useState<string>("");
	const [modName, setModName] = useState<string>("");
	const [attributes, setAttributes] = useState<RivenAttributeWithUnits[]>([]);
	const [mastery, setMastery] = useState<number>(0);
	const [reRolls, setReRolls] = useState<number>(0);
	const [rank, setRank] = useState<number>(0);
	// Fetch data from rust side
	const { data: weapons } = useQuery<CacheRivenWeapon[], Error>({
		queryKey: ['cache_riven_weapons'],
		queryFn: () => api.cache.getRivenWeapons(),
	})
	const { data: allAttributes } = useQuery<CacheRivenAttribute[], Error>({
		queryKey: ['cache_riven_attributes'],
		queryFn: () => api.cache.getRivenAttributes(),
	})
	useEffect(() => {
		if (!weapons || !allAttributes) return;
		let weapon_url_name = "";
		// Check id type
		if (typeof (riven.id) == "string") {
			const auction = riven as Wfm.Auction<Wfm.AuctionOwner>;
			weapon_url_name = auction.item.weapon_url_name;
			setPolarity(auction.item.polarity)
			setModName(auction.item.name)
			setAttributes(auction.item.attributes.map((item) => {
				const attribute = allAttributes?.find((attribute) => attribute.url_name == item.url_name);
				let symbol = "";
				if (attribute?.units == "multiply") symbol = "+";
				if (attribute?.units == "percent") symbol = "%";
				return {
					...item,
					effect: attribute?.effect || "",
					units: attribute?.units || "",
					symbol,
				}
			}));
			setMastery(auction.item.mastery_level);
			setReRolls(auction.item.re_rolls);
			setRank(auction.item.mod_rank);
		}
		if (typeof (riven.id) == "number") {
			weapon_url_name = (riven as StockRiven).wfm_weapon_url;

		}
		if (weapons && weapon_url_name != "")
			setWeapon(weapons.find((item) => item.wfm_url_name == weapon_url_name));
	}, [riven, weapons]);
	return (
		<Box {...paperProps} className={classes.root} ref={ref}>
			{polarity != "" && (
				<>
					<SvgIcon className={classes.polarity} svgProp={{
						width: 20,
						height: 20,
					}} iconType={SvgType.Polarity} iconName={polarity} />
					<Text className={classes.weapon}>{weapon?.i18n["en"].name}</Text>
					<Text className={classes.mod_name}>{modName}</Text>
					<Box className={classes.attributes} style={{
						display: 'flex',
						alignItems: 'center',
						flexDirection: 'column',
					}}>
						{attributes.map((item, index) => {
							return (
								<Text maw={"215"} truncate="end" key={index} className={classes.attribute_text}>
									{item.value}{item.symbol} {item.effect}
								</Text>
							)
						})}
					</Box>
					<Text className={classes.mastery}>MR {mastery}</Text>
					{reRolls > 0 &&
						<Text className={classes.reroll}>
							<FontAwesomeIcon icon={faArrowsRotate} />
							<Text component="span" ml={5}>
								{reRolls}
							</Text>
						</Text>
					}
				</>
			)}
			<Box className={classes.rank}>
				{Array.from(Array(rank).keys()).map((i) => {
					return <Text key={i} className={classes.circle} size="sm" component="span">●</Text>
				})}
			</Box>
			<Collapse in={hovered} className={classes.hover}>
				<Text size="sm" c="gray" className={classes.hover_text}>
					<pre>					</pre>
				</Text>
			</Collapse>
		</Box>
	);
}

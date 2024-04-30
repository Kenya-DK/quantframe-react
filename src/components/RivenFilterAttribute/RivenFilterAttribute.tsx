import { Box, Switch, Text, Tooltip } from '@mantine/core';
import classes from './RivenFilterAttribute.module.css';
import { StockRivenFilterAttribute } from '@api/types';
import { useEffect, useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import api, { } from "@api/index";
import { useTranslateComponent } from '@hooks/index';

export type RivenFilterAttributeProps = {
	value: StockRivenFilterAttribute;
	onChanges: (value: StockRivenFilterAttribute) => void;
}
export function RivenFilterAttribute({ value, onChanges }: RivenFilterAttributeProps) {

	const [nameMap, setNameMap] = useState<{ [key: string]: string }>({});

	// Fetch data from rust side
	const { data } = useQuery({
		queryKey: ['cache_riven_attributes'],
		queryFn: () => api.cache.getRivenAttributes(),
	})

	// Set name map
	useEffect(() => {
		if (data) {
			const map: { [key: string]: string } = {};
			data.forEach((item) => {
				map[item.url_name] = item.effect;
			});
			setNameMap(map);
		}
	}, [data]);
	// Translate general
	const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`riven_filter_attribute.${key}`, { ...context }, i18Key)
	const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)
	return (
		<Box data-positive={!value.is_negative} className={classes.root}>
			<Text >{nameMap[value.url_name]}</Text>
			<Tooltip label={useTranslateFormFields("is_required.label")} position='left'>
				<span>
					<Switch color='blue' checked={value.is_required}
						onChange={(event) => { onChanges({ ...value, is_required: event.currentTarget.checked }); }}
						className={classes.switch} />
				</span>
			</Tooltip>
		</Box>
	);
}
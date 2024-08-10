import { Box, Button, Grid, Group, NumberInput, Select, Text, SelectProps } from '@mantine/core';
import { useTranslateForms } from '@hooks/useTranslate.hook';
import { useForm } from '@mantine/form';
import { CacheRivenAttribute, StockRiven } from '@api/types';
import api from '@api/index';
import { useQuery } from '@tanstack/react-query';
import { SvgIcon, SvgType } from '@components/SvgIcon';
import { groupBy } from '@utils/helper';
import { upperFirst } from '@mantine/hooks';
import { RivenPreview } from '@components/RivenPreview'
import { CreateRivenAttributes } from '../CreateRivenAttributes';
import { useEffect, useState } from 'react';

export type CreateRivenProps = {
	value?: StockRiven;
	onSubmit: (values: StockRiven) => void;
}

const icons: Record<string, React.ReactNode> = {
	madurai: <SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Polarity} iconName={"madurai"} />,
	naramon: <SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Polarity} iconName={"naramon"} />,
	vazarin: <SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Polarity} iconName={"vazarin"} />,
};

const renderSelectOption: SelectProps['renderOption'] = ({ option, checked }) => (
	<Group flex="1" gap="xs" style={{ fontWeight: checked ? 700 : 400 }}>
		{icons[option.value]}
		{option.label}
	</Group>
);
export function CreateRiven({ value, onSubmit }: CreateRivenProps) {
	// State
	const [availableAttributes, setAvailableAttributes] = useState<CacheRivenAttribute[]>([]);
	const [modNames, setModNames] = useState<string[]>([]);

	// Translate general
	const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`create_riven.${key}`, { ...context }, i18Key)
	const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)
	const useTranslateButton = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)

	// Fetch data from rust side
	const { data: weapons } = useQuery({
		queryKey: ['cache_riven_weapons'],
		queryFn: () => api.cache.getRivenWeapons(),
	})

	const { data: attributes } = useQuery({
		queryKey: ['cache_riven_attributes'],
		queryFn: () => api.cache.getRivenAttributes(),
	})


	// Helper functions
	const getAvailableWeapons = () => {
		if (!weapons)
			return [];

		const group = groupBy("wfm_group", weapons);
		return Object.entries(group).map(([key, value]) => {
			return {
				group: upperFirst(key),
				items: value.map((item) => ({ label: item.name, value: item.wfm_url_name })),
			}
		});

	};

	// User form
	const form = useForm({
		initialValues: {
			...value,
			id: value?.id || 0,
			mastery_rank: value?.mastery_rank || 8,
			re_rolls: value?.re_rolls || 0,
			sub_type: {
				...value?.sub_type,
				rank: value?.sub_type?.rank || 0,
			},
			polarity: value?.polarity || "madurai",
		},
		validate: {
			mod_name: (value) => !value ? useTranslateFormFields('mod_name.error') : null,
			attributes: (value) => !value || value.length === 0 ? useTranslateFormFields('attributes.error') : null,
		},
	});


	// Effects 
	useEffect(() => {
		if (!attributes)
			return;

		const weapon = weapons?.find((item) => item.wfm_url_name == form.values.wfm_weapon_url);

		const avAttributes = attributes.filter((item) => !item.exclusive_to || item.exclusive_to.includes(weapon?.wfm_group || ""));
		setAvailableAttributes(avAttributes);
	}, [form.values, attributes]);
	useEffect(() => {

		const rivenIds: { [key: string]: CacheRivenAttribute } = {};
		const filteredArray = form.values.attributes?.filter(entry => entry !== null && entry.positive);
		if (!filteredArray || filteredArray.length === 0)
			return
		availableAttributes.forEach((item) => { rivenIds[item.url_name] = { ...item } });
		function generatePermutations(inputArray: string[]): string[][] {
			let currentIndex: string, swapIndex: number;
			const arrayLength = inputArray.length;
			let permutations = [inputArray.slice()];
			const counters = new Array(arrayLength).fill(0);

			for (let index = 1; index < arrayLength;) {
				if (counters[index] < index) {
					swapIndex = index % 2 ? counters[index] : 0;

					// Swap elements
					currentIndex = inputArray[index];
					inputArray[index] = inputArray[swapIndex];
					inputArray[swapIndex] = currentIndex;

					counters[index]++;
					index = 1;
					permutations.push(inputArray.slice());
				} else {
					counters[index] = 0;
					index++;
				}
			}

			return permutations;
		}
		let selectedIds = generatePermutations(filteredArray.map((item) => item.url_name));

		let modNames: string[] = [];
		selectedIds.forEach((item) => {
			if (2 === item.length) {
				modNames.push(`${rivenIds[item[0]].prefix}${rivenIds[item[1]].suffix.toLowerCase()}`);
			} else if (3 === item.length) {
				modNames.push(`${rivenIds[item[0]].prefix}-${rivenIds[item[1]].prefix.toLowerCase()}${rivenIds[item[2]].suffix.toLowerCase()}`);
			}
		})
		setModNames(modNames);
	}, [form.values.attributes, availableAttributes]);

	return (
		<Box w={"100%"}>
			<form onSubmit={form.onSubmit((data) => {
				onSubmit(data as StockRiven);
			})}>
				<Grid mb={75} >
					<Grid.Col span={6}>
						<RivenPreview riven={form.values as StockRiven} />
					</Grid.Col>
					<Grid.Col span={6}>
						<Group gap="md" grow>
							<Select
								searchable
								limit={5}
								required
								w={"150"}
								allowDeselect={false}
								label={useTranslateFormFields("weapon.label")}
								value={form.values.wfm_weapon_url}
								onChange={(event) => form.setFieldValue('wfm_weapon_url', event || "")}
								data={getAvailableWeapons()}
							/>
						</Group>
						<CreateRivenAttributes attributes={availableAttributes} value={form.values.attributes || []} onSubmit={(values) => form.setFieldValue('attributes', values)} />
						{form.errors.attributes && <Text color="red">{form.errors.attributes}</Text>}
						<Group grow>
							<Select
								required
								allowDeselect={false}
								label={useTranslateFormFields("mod_name.label")}
								value={form.values.mod_name}
								onChange={(event) => form.setFieldValue('mod_name', event || "")}
								error={form.errors.mod_name && useTranslateFormFields('mod_name.error')}
								data={modNames}
								renderOption={renderSelectOption}
							/>
						</Group>
						<Group gap="md">
							<NumberInput
								required
								w={"20%"}
								max={16}
								min={8}
								label={useTranslateFormFields('mastery_rank.label')}
								placeholder={useTranslateFormFields('mastery_rank.placeholder')}
								value={form.values.mastery_rank}
								onChange={(event) => form.setFieldValue('mastery_rank', Number(event))}
								error={form.errors.mastery_rank && useTranslateFormFields('mastery_rank.error')}
								radius="md"
							/>
							<NumberInput
								w={"13%"}
								required
								min={0}
								label={useTranslateFormFields('re_rolls.label')}
								placeholder={useTranslateFormFields('re_rolls.placeholder')}
								value={form.values.re_rolls}
								onChange={(event) => form.setFieldValue('re_rolls', Number(event))}
								error={form.errors.re_rolls && useTranslateFormFields('re_rolls.error')}
								radius="md"
							/>
							<NumberInput
								required
								w={"13%"}
								min={0}
								label={useTranslateFormFields('bought.label')}
								placeholder={useTranslateFormFields('bought.placeholder')}
								value={form.values.bought || 0}
								onChange={(event) => form.setFieldValue('bought', Number(event))}
								error={form.errors.bought && useTranslateFormFields('bought.error')}
								radius="md"
							/>
							<NumberInput
								required
								w={"13%"}
								max={8}
								min={0}
								label={useTranslateFormFields('rank.label')}
								placeholder={useTranslateFormFields('rank.placeholder')}
								value={form.values.sub_type?.rank || 0}
								onChange={(event) => form.setFieldValue('sub_type.rank', Number(event))}
								error={form.errors.rank && useTranslateFormFields('rank.error')}
								radius="md"
							/>
							<Select
								required
								w={"30%"}
								allowDeselect={false}
								label={useTranslateFormFields("polarity.label")}
								value={form.values.polarity}
								onChange={(event) => form.setFieldValue('polarity', event || "")}
								leftSection={<SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Polarity} iconName={form.values.polarity} />}
								data={[
									{ value: "madurai", label: "Madurai" },
									{ value: "naramon", label: "Naramon" },
									{ value: "vazarin", label: "Vazarin" },
								]}
								renderOption={renderSelectOption}
							/>
						</Group>

					</Grid.Col>
				</Grid>
				<Group justify="flex-end" style={{
					position: "absolute",
					bottom: 25,
					right: 25,
				}}>
					<Button type="submit" variant="light" color="blue">
						{useTranslateButton('submit.label')}
					</Button>
				</Group>
			</form>
		</Box>
	);
}
import { Box, Grid, Group, MultiSelect, Paper, RangeSlider, Text } from '@mantine/core';
import { CacheTradableItem } from '@api/types';
import { useTranslateComponent } from '@hooks/useTranslate.hook';
import { DataTable, DataTableSortStatus } from 'mantine-datatable';
import { useEffect, useState } from 'react';
import { useForm } from '@mantine/form';
import { paginate } from "@utils/helper";
import { SearchField } from '@components/SearchField';
import { ActionWithTooltip } from '@components/ActionWithTooltip';
import { faAdd } from '@fortawesome/free-solid-svg-icons';
import { sortArray } from '@utils/sorting.helper';

export type TradableItemListProps = {
	availableItems: CacheTradableItem[];
	onAddAll?: (items: CacheTradableItem[]) => void;
	onAddItem?: (item: CacheTradableItem) => void;
}


export function TradableItemList({ onAddItem, onAddAll, availableItems }: TradableItemListProps) {
	// States For DataGrid
	const [page, setPage] = useState(1);
	const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
	const [pageSize, setPageSize] = useState(pageSizes[4]);
	const [rows, setRows] = useState<CacheTradableItem[]>([]);
	const [totalRecords, setTotalRecords] = useState<number>(0);
	const [sortStatus, setSortStatus] = useState<DataTableSortStatus<CacheTradableItem>>({ columnAccessor: 'name', direction: 'desc' });

	// Translate general
	const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`tradableItem_list.${key}`, { ...context }, i18Key)
	const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`datatable.columns.${key}`, { ...context }, i18Key)
	const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`fields.${key}`, { ...context }, i18Key)
	const useTranslateSearchButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`buttons.${key}`, { ...context }, i18Key)

	const availableItemTags = [
		{
			label: useTranslateFormFields("tags.options.set"),
			value: 'set',
		},
		{
			label: useTranslateFormFields("tags.options.prime"),
			value: 'prime',
		},
		{
			label: useTranslateFormFields("tags.options.arcane_enhancement"),
			value: 'arcane_enhancement',
		},
		{
			label: useTranslateFormFields("tags.options.tax_1m"),
			value: 'tax_1000000',
		},
		{
			label: useTranslateFormFields("tags.options.tax_2m"),
			value: 'tax_2100000',
		},
	]

	// Form for filtering
	const filterForm = useForm({
		initialValues: {
			search: '',
			tags: [] as string[],
			tradeTaxRange: [0, 2100000] as [number, number],
			mrRequirementRange: [0, 15] as [number, number],
		},
	});

	const GetFilteredItems = () => {
		if (!availableItems)
			return;
		let itemFilter = availableItems;

		if (filterForm.values.search) {
			const search = filterForm.values.search.toLowerCase();
			itemFilter = itemFilter.filter((item) => {
				return item.name.toLowerCase().includes(search);
			});
		}

		if (filterForm.values.tags.length > 0) {
			itemFilter = itemFilter.filter((item) => {
				return item.tags.some((tag) => filterForm.values.tags.includes(tag));
			});
		}

		itemFilter = itemFilter.filter((item) => {
			return item.trade_tax >= filterForm.values.tradeTaxRange[0] && item.trade_tax <= filterForm.values.tradeTaxRange[1];
		});

		itemFilter = itemFilter.filter((item) => {
			return item.mr_requirement >= filterForm.values.mrRequirementRange[0] && item.mr_requirement <= filterForm.values.mrRequirementRange[1];
		});

		itemFilter = sortArray([{
			field: sortStatus.columnAccessor,
			direction: sortStatus.direction
		}], itemFilter);
		return itemFilter;
	}


	// Update DataGrid Rows
	useEffect(() => {
		let itemFilter = GetFilteredItems();
		if (!itemFilter)
			return;
		setTotalRecords(itemFilter.length);
		itemFilter = paginate(itemFilter, page, pageSize);
		setRows(itemFilter);
	}, [availableItems, sortStatus, filterForm])

	return (
		<Box>
			<Box>
				<SearchField
					value={filterForm.values.search}
					onChange={(e) => filterForm.setFieldValue('search', e)}
					filter={
						<Paper radius="md" p={"sm"} mt={"md"}>
							<Grid>
								<Grid.Col span={6}>
									<MultiSelect
										multiple
										label={useTranslateFormFields('tags.label')}
										placeholder={useTranslateFormFields('tags.placeholder')}
										data={availableItemTags}
										value={filterForm.values.tags}
										onChange={(value) => filterForm.setFieldValue('tags', value)}
										clearable
									/>
								</Grid.Col>
								<Grid.Col span={6}>
									<Box>
										<Text size="sm" mt="xl">{useTranslateFormFields('trade_tax.label', { min: filterForm.values.tradeTaxRange[0], max: filterForm.values.tradeTaxRange[1] })}</Text>
										<RangeSlider
											w={300}
											color="blue"
											value={filterForm.values.tradeTaxRange}
											onChange={(value) => filterForm.setFieldValue('tradeTaxRange', value)}
											step={1000}
											min={0}
											max={2100000}
										/>
									</Box>
									<Box>
										<Text size="sm" mt="xl">{useTranslateFormFields('mr_requirement.label', { min: filterForm.values.mrRequirementRange[0], max: filterForm.values.mrRequirementRange[1] })}</Text>
										<RangeSlider
											w={300}
											step={1}
											value={filterForm.values.mrRequirementRange}
											onChange={(value) => filterForm.setFieldValue('mrRequirementRange', value)}
											color="blue"
											minRange={1}
											min={0}
											max={15}
										/>
									</Box>
								</Grid.Col>
							</Grid>
						</Paper>
					}
					rightSectionWidth={75}
					rightSection={
						<Group gap={5}>
							<ActionWithTooltip
								tooltip={useTranslateSearchButtons('add_all.tooltip')}
								icon={faAdd}
								onClick={() => {
									if (onAddAll) {
										const items = GetFilteredItems();
										if (!items) return;
										onAddAll(items);
									}
								}}
							/>
						</Group>
					}
				/>
			</Box>
			<DataTable
				height={"50vh"}
				mt={"md"}
				records={rows}
				totalRecords={totalRecords}
				withTableBorder
				withColumnBorders
				page={page}
				recordsPerPage={pageSize}
				idAccessor={"wfm_id"}
				onPageChange={(p) => setPage(p)}
				recordsPerPageOptions={pageSizes}
				onRecordsPerPageChange={setPageSize}
				sortStatus={sortStatus}
				onSortStatusChange={setSortStatus}
				onRowClick={(row) => {
					if (onAddItem)
						onAddItem(row.record);
				}}
				// define columns
				columns={[
					{
						accessor: 'name',
						title: useTranslateDataGridColumns('name'),
						sortable: true,
					},
					{
						accessor: 'trade_tax',
						title: useTranslateDataGridColumns('trade_tax'),
						sortable: true,
					},
					{
						accessor: 'mr_requirement',
						title: useTranslateDataGridColumns('mr_requirement'),
						sortable: true,
					}
				]}
			/>
		</Box>
	);
}
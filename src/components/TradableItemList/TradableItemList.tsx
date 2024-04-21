import { Box, Group, Paper, Text } from '@mantine/core';
import { CacheTradableItem } from '@api/types';
import classes from './TradableItemList.module.css';
import { useTranslateComponent } from '@hooks/index';
import { DataTable, DataTableSortStatus } from 'mantine-datatable';
import { useEffect, useState } from 'react';
import { useForm } from '@mantine/form';
import { sortArray, paginate } from "@utils/index";

export type TradableItemListProps = {
	availableItems: CacheTradableItem[];
}


export function TradableItemList({ availableItems }: TradableItemListProps) {
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
	// const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`fields.${key}`, { ...context }, i18Key)

	// Form for filtering
	const filterForm = useForm({
		initialValues: {
			search: '',
		},
	});

	// Update DataGrid Rows
	useEffect(() => {
		if (!availableItems)
			return;

		let itemFilter = availableItems;


		setTotalRecords(itemFilter.length);
		itemFilter = sortArray([{
			field: sortStatus.columnAccessor,
			direction: sortStatus.direction
		}], itemFilter);



		itemFilter = paginate(itemFilter, page, pageSize);
		setRows(itemFilter);


	}, [availableItems, pageSize, page, sortStatus, filterForm])

	return (
		<Box>
			<Group>
				<Paper shadow="xs" className={classes.paper}>
					<form onSubmit={filterForm.onSubmit(() => { })}>
						<Group>
							<Text>Search</Text>
							<Text>Sort</Text>
						</Group>
					</form>
				</Paper>
			</Group>
			<DataTable
				height={`calc(100vh - 420px)`}
				mt={"md"}
				records={rows}
				totalRecords={totalRecords}
				withTableBorder
				withColumnBorders
				page={page}
				recordsPerPage={pageSize}
				idAccessor={"id"}
				onPageChange={(p) => setPage(p)}
				recordsPerPageOptions={pageSizes}
				onRecordsPerPageChange={setPageSize}
				sortStatus={sortStatus}
				onSortStatusChange={setSortStatus}
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
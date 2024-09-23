import { Box } from "@mantine/core";
import { DataTable, DataTableProps, DataTableSortStatus } from "mantine-datatable";
import { SortingField } from "$types/index";
import { useEffect, useState } from "react";
import { sortArray } from "@utils/sorting.helper";
import { paginate } from "@utils/helper";
import { SearchField, SearchFieldProps } from "../SearchField";
import { searchProperties,Query } from "@utils/search.helper";

export type DataTableSearchProps<T> = {
  filters?: Query;
  sorting?: SortingField;
  query?: string;
  onSearchChange?: (query: string) => void;
} & DataTableProps<T> & Omit<SearchFieldProps, "value" | "onChange">;
export const DataTableSearch = <T,>({ query, onSearchChange, records, rightSection, rightSectionWidth, filters, filter, sorting, ...otherProps }: DataTableSearchProps<T>) => {

  // States For DataGrid
  const [page, setPage] = useState(1);
  const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, setPageSize] = useState(pageSizes[4]);
  const [rows, setRows] = useState<T[]>([]);
  const [totalRecords, setTotalRecords] = useState<number>(0);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus<T>>({ columnAccessor: sorting?.field || "", direction: sorting?.direction || "asc" });
  // Update Database Rows
  useEffect(() => {
    if (!records)
      return;

    let filteredRecords = records;
    console.log("DataTableSearch: Filter Status", filteredRecords.map((x) => (x as any).id));

    filteredRecords = sortArray([{
      field: sortStatus.columnAccessor as string,
      direction: sortStatus.direction
    }], filteredRecords);
    if (filters)
      filteredRecords = searchProperties(records, filters, false);
    setTotalRecords(filteredRecords.length);
    filteredRecords = paginate(filteredRecords, page, pageSize);
    setRows(filteredRecords);
  }, [filters, page, pageSize, sortStatus]);
  return (
    <Box pos="relative" >
      {(onSearchChange && query != undefined) && <SearchField value={query} onChange={(t) => onSearchChange(t)} rightSection={rightSection} rightSectionWidth={rightSectionWidth} filter={filter} />}
      <DataTable
        mt={5}
        records={rows}
        totalRecords={totalRecords}
        page={page}
        recordsPerPage={pageSize}
        onPageChange={(p) => setPage(p)}
        recordsPerPageOptions={pageSizes}
        onRecordsPerPageChange={setPageSize}
        sortStatus={sortStatus}
        onSortStatusChange={setSortStatus}
        {...otherProps}
      />

    </Box>
  )
}
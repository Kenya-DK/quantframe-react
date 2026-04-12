import { Box } from "@mantine/core";
import { DataTable, DataTableProps, DataTableSortStatus } from "mantine-datatable";
import React, { useEffect, useState } from "react";
import { paginate } from "@utils/helper";
import { SearchFieldProps } from "@components/Forms/SearchField";
import { SearchField } from "@components/Forms/SearchField";
import { Sort, SortItems } from "@utils/sorting.helper";
import { ComplexFilter, ApplyFilter } from "@utils/filter.helper";

export type DataTableSearchProps<T> = {
  filters?: ComplexFilter;
  sorting?: Sort;
  query?: string;
  onSearchChange?: (query: string) => void;
  onFilterItems?: (items: T[]) => void;
  context?: React.ReactNode;
  hideComponents?: string[];
} & DataTableProps<T> &
  Omit<SearchFieldProps, "value" | "onChange">;
export const DataTableSearch = <T,>({
  query,
  onSearchChange,
  records,
  rightSection,
  rightSectionWidth,
  filters,
  filter,
  sorting,
  onFilterItems,
  hideComponents,
  context,
  ...otherProps
}: DataTableSearchProps<T>) => {
  // States For DataGrid
  const [page, setPage] = useState(1);
  const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, setPageSize] = useState(pageSizes[4]);
  const [rows, setRows] = useState<T[]>([]);
  const [totalRecords, setTotalRecords] = useState<number>(0);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus<T>>({
    columnAccessor: sorting?.field || "",
    direction: sorting?.direction || "asc",
  });
  // Update Database Rows
  useEffect(() => {
    if (!records) return;

    let filteredRecords = records;

    filteredRecords = SortItems(filteredRecords, {
      field: sortStatus.columnAccessor as string,
      direction: sortStatus.direction,
    });
    if (filters) filteredRecords = ApplyFilter(records, filters);
    if (onFilterItems) onFilterItems(filteredRecords);
    setTotalRecords(filteredRecords.length);
    filteredRecords = paginate(filteredRecords, page, pageSize);
    setRows(filteredRecords);
  }, [filters, page, pageSize, sortStatus]);
  return (
    <Box pos="relative">
      {onSearchChange && (
        <SearchField
          value={query || ""}
          onChange={(t) => onSearchChange(t)}
          rightSection={rightSection}
          rightSectionWidth={rightSectionWidth}
          filter={filter}
        />
      )}
      {hideComponents?.includes("context") ? null : context}
      {hideComponents?.includes("table") ? null : (
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
      )}
    </Box>
  );
};

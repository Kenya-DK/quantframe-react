import { Box, Group, Stack } from "@mantine/core";
import { DataTable, DataTableColumn, DataTableSortStatus } from "mantine-datatable";
import { useMemo, useState } from "react";
import { paginate } from "@utils/helper";
import { SearchField } from "@components/Forms/SearchField";
import classes from "./GenericItemList.module.css";
import { ApplyFilter, ComplexFilter } from "../../../utils/filter.helper";
import { SortItems } from "../../../utils/sorting.helper";
import { ActionWithTooltip } from "../../Shared/ActionWithTooltip";
import { faAdd } from "@fortawesome/free-solid-svg-icons";
import { useTranslateForms } from "../../../hooks/useTranslate.hook";

export type GenericItemListProps<T> = {
  items: T[];
  onAddAll?: (items: T[]) => void;
  onAddItem?: (item: T) => void;
  columns: DataTableColumn<T>[];
  idAccessor: keyof T | ((item: T) => string | number);
  pageSizes?: number[];

  // 🔍 Search support
  searchable?: boolean;
  searchValue?: string;
  onSearchChange?: (val: string) => void;
  onSearch?: (val: string) => void;
  searchFilter?: React.ReactNode;
  searchRightSectionWidth?: number;
  searchRightSection?: React.ReactNode;
  filter?: ComplexFilter;
};

export function GenericItemList<T>({
  items,
  onAddItem,
  onAddAll,
  columns,
  idAccessor,
  pageSizes = [5, 10, 20, 50, 100],
  searchable,
  searchValue,
  onSearchChange,
  onSearch,
  searchFilter,
  searchRightSection,
  searchRightSectionWidth,
  filter,
}: GenericItemListProps<T>) {
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(pageSizes[3]);
  const [is_filter_open, setIsFilterOpen] = useState(false);

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`generic_item_list.${key}`, { ...context }, i18Key);

  // const [rows, setRows] = useState<T[]>([]);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus<T>>({
    columnAccessor: columns[0].accessor as string,
    direction: "asc",
  });

  // Apply filtering
  const filteredItems = useMemo(() => {
    let result = items;
    if (filter) result = result ? ApplyFilter(items, filter) : items;
    return result;
  }, [items, searchValue, searchFilter, filter, sortStatus]);

  const rows = useMemo(() => {
    let result = SortItems<T>(filteredItems, { field: sortStatus.columnAccessor as string, direction: sortStatus.direction });
    return paginate(result, page, pageSize);
  }, [filteredItems, page, pageSize, sortStatus]);

  return (
    <Stack>
      {searchable && (
        <SearchField
          value={searchValue ?? ""}
          onChange={onSearchChange ?? (() => {})}
          onSearch={onSearch}
          rightSection={
            <Group gap={4}>
              {searchRightSection}
              {onAddAll && (
                <ActionWithTooltip
                  tooltip={useTranslateForm("add_all_tooltip")}
                  icon={faAdd}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={() => onAddAll(filteredItems)}
                />
              )}
            </Group>
          }
          rightSectionWidth={searchRightSectionWidth}
          filter={searchFilter}
          onFilterToggle={(open) => setIsFilterOpen(open)}
        />
      )}

      <Box className={classes.datatable} data-filter={is_filter_open}>
        <DataTable
          records={rows}
          totalRecords={filteredItems.length}
          withTableBorder
          withColumnBorders
          page={page}
          recordsPerPage={pageSize}
          idAccessor={idAccessor as any}
          onPageChange={setPage}
          recordsPerPageOptions={pageSizes}
          onRecordsPerPageChange={setPageSize}
          sortStatus={sortStatus}
          onSortStatusChange={setSortStatus}
          onRowClick={(row) => onAddItem?.(row.record)}
          columns={columns}
        />
      </Box>
    </Stack>
  );
}

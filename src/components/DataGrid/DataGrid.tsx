import { Box, Center, Checkbox, Divider, Group, Loader, Pagination, ScrollArea, Select, Table, TableProps, Text } from "@mantine/core";
import classes from './DataGrid.module.css';
import React, { useEffect, useState } from "react";
import { upperFirst } from "@mantine/hooks";
import { Query, searchProperties } from "@utils/search.helper";
import { paginate } from "@utils/helper";


export interface DataGridColumnProps<T> {
  title?: string;
  accessor: keyof T | string;
  width?: number | string;
  render?: (value: T) => string | JSX.Element;
}
export type DataGridProps<T> = {
  columns: DataGridColumnProps<T>[];
  height?: number | string;
  filters?: Query;
  records: T[];
  fetching?: boolean;
  customLoader?: React.ReactNode;
  selectedRecords?: T[];
  onSelectionChange?: (records: T[]) => void;
  onRowClick?: (record: T) => void;
  customRowAttributes?: (record: T, index: number) => Record<string, unknown>;
} & Omit<
  TableProps,
  | 'onScroll'
  | 'className'
  | 'classNames'
  | 'style'
  | 'styles'
  | 'p'
  | 'px'
  | 'py'
  | 'pt'
  | 'pb'
  | 'layout'
  | 'captionSide'
  | 'c'
  | 'color'
  | 'borderColor'
  | 'stripedColor'
  | 'highlightOnHoverColor'
  | 'stickyHeader'
  | 'stickyHeaderOffset'
  | 'onDragEnd'
>
export const DataGrid = <T,>({ m, my, mx, mt, mb, ml, mr, customRowAttributes, records, height, selectedRecords, onSelectionChange, fetching, filters, columns, customLoader, ...otherProps }: DataGridProps<T>) => {

  // States For DataGrid
  const [page, setPage] = useState(1);
  const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, setPageSize] = useState(pageSizes[4]);
  const [rows, setRows] = useState<T[]>([]);
  const [totalRecords, setTotalRecords] = useState(0);
  const [totalPages, setTotalPages] = useState(0);
  const [start, setStart] = useState(0);
  const [end, setEnd] = useState(0);

  useEffect(() => {
    let filteredRecords = [...records];
    if (filters)
      filteredRecords = searchProperties(records, filters);
    // Pagination Logic
    setTotalRecords(filteredRecords.length);
    setTotalPages(Math.ceil(filteredRecords.length / pageSize));
    setStart((page - 1) * pageSize + 1);
    setEnd(Math.min(page * pageSize, filteredRecords.length));
    filteredRecords = paginate(filteredRecords, page, pageSize);
    setRows(filteredRecords);
  }, [records, filters, page, pageSize]);


  const GetCellValues = (record: T, col: DataGridColumnProps<T>): React.ReactNode => {
    if (col.render)
      return col.render(record);
    return record[col.accessor as keyof T] as unknown as React.ReactNode;
  }
  const marginProperties = { m, my, mx, mt, mb, ml, mr };
  return (
    <Box pos="relative" {...marginProperties}>
      <Center className={classes.loader} data-fetching={fetching}>
        {customLoader || <Loader />}
      </Center>
      <ScrollArea.Autosize type="auto" h={height} >
        <Table verticalSpacing="5px" classNames={classes} {...otherProps}>
          <Table.Thead className={classes.thead}>
            <Table.Tr>
              {selectedRecords && (
                <Table.Th>
                  <Checkbox
                    aria-label="Select all rows"
                    checked={selectedRecords.length === rows.length}
                    onChange={(event) =>
                      onSelectionChange?.(
                        event.currentTarget.checked ? rows : []
                      )
                    }
                  />
                </Table.Th>
              )}
              {columns.map((col, index) => (
                <Table.Th key={index}>
                  {col.title || upperFirst(col.accessor.toString())}
                </Table.Th>
              ))}
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {rows.map((_record, index) => (
              <Table.Tr {...(customRowAttributes?.(_record, index) ?? {})} key={index} >
                {selectedRecords && (
                  <Table.Td w={30}>
                    <Checkbox
                      checked={selectedRecords.includes(_record)}
                      onChange={(event) =>
                        onSelectionChange?.(
                          event.currentTarget.checked
                            ? [...selectedRecords, _record]
                            : selectedRecords.filter((x) => x !== _record)
                        )
                      }
                    />
                  </Table.Td>
                )}
                {columns.map((col, index) => (
                  <Table.Td key={index} w={col.width}>
                    {GetCellValues(_record, col)}
                  </Table.Td>
                ))}
              </Table.Tr>
            ))}
          </Table.Tbody>
        </Table>
      </ScrollArea.Autosize>
      <Divider />
      <Group h={"35px"} justify="space-between" mt={"sm"}>
        <Text size="sm">{start} - {end} / {totalRecords}</Text>
        <Group>
          <Text size="sm">Rows: </Text>
          <Select
            allowDeselect={false}
            w={"60px"}
            size="xs"
            data={pageSizes.map((size) => ({ label: size.toString(), value: size.toString() }))}
            value={pageSize.toString()}
            onChange={(value) => setPageSize(parseInt(value || "0"))}
          />
          <Pagination size={"sm"} value={page} onChange={setPage} total={totalPages} />
        </Group>
      </Group>

    </Box>
  )
}

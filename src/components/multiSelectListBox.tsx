
import { ActionIcon, Box, Divider, Flex, Group, Stack, TextInput, Tooltip } from '@mantine/core';
import { useState, useEffect } from 'react';
import { paginate } from '../utils';
import { DataTable } from 'mantine-datatable';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPlusCircle, faRemove } from '@fortawesome/free-solid-svg-icons';
import { useTranslateComponent } from '../hooks';

interface Row {
  label: string;
  value: string;
}

interface SearchItemFieldProps {
  availableItems: Array<Row>;
  selectedItems: string[];
  onChange: (items: string[]) => void;
}

enum ListBoxType {
  Available,
  Selected
}

const ListBox = ({ items, onClick, type, onChange }: { type: ListBoxType, items: Array<Row>, onChange: (items: string[]) => void, onClick: (row: Row) => void, actions?: React.ReactNode }) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`multiSelectListBox.${key}`, { ...context })
  const [page, setPage] = useState(1);
  const [query, setQuery] = useState<string>('');
  const [queryItems, setQueryItems] = useState<Array<Row>>([]);
  const [filteredItems, setFilteredItems] = useState<Array<Row>>([]);
  const pageSizes = [5, 10, 15, 20, 25];
  const [pageSize, setPageSize] = useState(pageSizes[4]);


  useEffect(() => {
    const filteredItemstemp = items.filter((item) => item.label.toLowerCase().includes(query.toLowerCase()));
    setFilteredItems(filteredItemstemp);
    setQueryItems(paginate(filteredItemstemp, page == 0 ? 1 : page, pageSize));
    // log
    // Check if page is not out of bounds
    if (page > Math.ceil(filteredItemstemp.length / pageSize) || page < 1) {
      setPage(Math.ceil(filteredItemstemp.length / pageSize));
    }
  }, [query, items, page, pageSize]);
  return (
    <Box>
      <Flex
        gap="sm"
        justify="flex-start"
        align="flex-start"
        direction="row"
      >
        <DataTable
          striped
          sx={{
            width: "100%",
          }}
          height={"300px"}
          records={queryItems}
          page={page}
          onPageChange={setPage}
          totalRecords={filteredItems.length}
          recordsPerPage={pageSize}
          recordsPerPageOptions={pageSizes}
          onRecordsPerPageChange={setPageSize}
          // define columns
          columns={[
            {
              accessor: 'label',
              title: useTranslateSearch('name'),
              width: 120,
            }
          ]}
          onRowClick={(row) => {
            onClick(row);
          }}
        />
        <Stack spacing="xs" w={20}>
          {type === ListBoxType.Available &&
            <Tooltip label={useTranslateSearch('add_all')}>
              <ActionIcon variant="filled" color="blue" onClick={() => { onChange(filteredItems.map(x => x.value)) }}>
                <FontAwesomeIcon icon={faPlusCircle} />
              </ActionIcon>
            </Tooltip>
          }
          {type === ListBoxType.Selected &&
            <Tooltip label={useTranslateSearch('remove_all')}>
              <ActionIcon variant="filled" color="red.7" onClick={() => { onChange([]) }}>
                <FontAwesomeIcon icon={faRemove} />
              </ActionIcon>
            </Tooltip>
          }
        </Stack>
      </Flex>
      <Group grow>
        <TextInput
          label="Search"
          placeholder="Search"
          value={query}
          onChange={(event) => setQuery(event.currentTarget.value)}
        />
      </Group>
    </Box >
  );
}

export const MultiSelectListBox = ({ availableItems, selectedItems, onChange }: SearchItemFieldProps) => {
  const [selectedItemsState, setSelectedItemsState] = useState<Array<Row>>([]);
  const handleAvailableItemClick = (row: Row) => {
    onChange([...selectedItems, row.value]);
  };

  const handleSelectedItemClick = (row: Row) => {
    onChange(selectedItems.filter((item) => item !== row.value));
  };

  useEffect(() => {
    const items = selectedItems.map((item) => {
      const aItem = availableItems.find((x) => x.value === item);
      if (!item) return;
      return aItem;
    }).filter((item) => item !== undefined) as Array<Row>;
    setSelectedItemsState(items);
  }, [selectedItems]);
  return (
    <Group position="left" spacing="xl">
      <Group grow>
        <ListBox onClick={(row: Row) => handleAvailableItemClick(row)} type={ListBoxType.Available} items={availableItems.filter((item) => !selectedItems.includes(item.value))}
          onChange={(items: string[]) => { onChange(items); }}
        />
      </Group>
      <Divider orientation="vertical" />
      <Group grow>
        <ListBox onClick={(row: Row) => handleSelectedItemClick(row)} type={ListBoxType.Selected} items={selectedItemsState}
          onChange={(items: string[]) => { onChange(items); }}
        />
      </Group>
    </Group>

  );
}
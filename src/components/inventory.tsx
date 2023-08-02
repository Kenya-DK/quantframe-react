
import { ActionIcon, Box, Button, Divider, Group, NumberInput, Tooltip, Text, Stack } from '@mantine/core';
import { useTranslateComponent } from '@hooks/index';
import { useForm } from '@mantine/form';
import { SearchItemField } from './searchItemField';
import { DataTable } from 'mantine-datatable';
import { useDatabaseContext } from '../contexts';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faHammer, faTrashCan } from '@fortawesome/free-solid-svg-icons';
import { modals } from '@mantine/modals';
import { Wfm } from '../types';
import { useState } from 'react';

interface PurchaseNewItemProps {
  onSumit: (type: string, id: string, quantity: number, price: number, mod_rank: number) => void;
}
const PurchaseNewItem = (props: PurchaseNewItemProps) => {
  const { onSumit } = props;
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`inventory.${key}`, { ...context })
  const [selectedItem, setSelectedItem] = useState<Wfm.ItemDto | null>(null);
  const roleForm = useForm({
    initialValues: {
      price: 0,
      item: "",
      quantity: 1,
      rank: 0,
      type: "buy"
    },
    validate: {
      item: (val) => (val.length <= 3 ? ('name_min') : null),
    },
  });
  return (
    <Group grow position="center" >
      <form method="post" onSubmit={roleForm.onSubmit(async (d) => {
        onSumit(d.type, d.item, d.quantity, d.price, d.rank);
      })}>
        <Stack justify='center' spacing="md">
          <Group grow >
            <SearchItemField value={roleForm.values.item} onChange={(value) => {
              setSelectedItem(value);
              roleForm.setFieldValue('item', value.id)
            }} />
            <NumberInput
              required
              label={useTranslateSearch('price')}
              description={useTranslateSearch('price_description')}
              value={roleForm.values.price}
              min={0}
              onChange={(value) => roleForm.setFieldValue('price', Number(value))}
              error={roleForm.errors.price && 'Invalid identifier'}
            />
            {/* <NumberInput
              required
              label={useTranslateSearch('quantity')}
              description={useTranslateSearch('quantity_description')}
              value={roleForm.values.quantity}
              min={1}
              onChange={(value) => roleForm.setFieldValue('quantity', Number(value))}
              error={roleForm.errors.quantity && 'Invalid identifier'}
            /> */}
            {(selectedItem?.category == "Mods" || selectedItem?.category == "Arcanes") &&
              <NumberInput
                required
                label={useTranslateSearch('rank')}
                description={useTranslateSearch('rank_description')}
                value={roleForm.values.rank}
                min={0}
                onChange={(value) => roleForm.setFieldValue('rank', Number(value))}
                error={roleForm.errors.rank && 'Invalid identifier'}
              />
            }
          </Group>
          <Group mt={5} position="center">
            <Button type="submit" onClick={() => roleForm.setFieldValue('type', "buy")} disabled={roleForm.values.item.length <= 0} radius="xl">
              {useTranslateSearch('buttons.buy')}
            </Button>
            <Button type="submit" onClick={() => roleForm.setFieldValue('type', "sell")} disabled={roleForm.values.item.length <= 0} radius="xl">
              {useTranslateSearch('buttons.sell')}
            </Button>
          </Group>
        </Stack>
      </form>
    </Group>
  );
}
const Items = () => {
  const useTranslateDataGrid = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`inventory.datagrid.${key}`, { ...context })
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }) => useTranslateDataGrid(`columns.${key}`, { ...context })
  const { invantory, deleteInvantoryEntryById } = useDatabaseContext();
  return (
    <DataTable
      sx={{ marginTop: "20px" }}
      height={"75vh"}
      withBorder
      striped

      records={invantory}
      // define columns
      columns={[
        {
          accessor: 'item_name',
          title: useTranslateDataGridColumns('name'),
          width: 120,
        },
        {
          accessor: 'price',
          title: useTranslateDataGridColumns('price'),
          width: 64,
        },
        {
          accessor: 'listed_price',
          title: useTranslateDataGridColumns('listed_price'),
          width: 64,
        },
        {
          accessor: 'owned',
          title: useTranslateDataGridColumns('owned'),
          width: 64,
        },
        {
          accessor: 'actions',
          width: 100,
          title: useTranslateDataGridColumns('actions.title'),
          render: ({ id }) =>
            <Group grow position="center" >
              <NumberInput
                required
                size='sm'
                min={0}
                rightSectionWidth={75}
                rightSection={
                  <Group spacing={"5px"} mr={0}>
                    <Divider orientation="vertical" />
                    <Tooltip label={useTranslateDataGridColumns('actions.sell')}>
                      <ActionIcon color="green.7" variant="filled" onClick={async () => {

                      }} >
                        <FontAwesomeIcon icon={faHammer} />
                      </ActionIcon>
                    </Tooltip>
                    <Tooltip label={useTranslateDataGridColumns('actions.delete.title')}>
                      <ActionIcon color="red.7" variant="filled" onClick={async () => {
                        modals.openConfirmModal({
                          title: useTranslateDataGridColumns('actions.delete.title'),
                          children: (<Text>
                            {useTranslateDataGridColumns('actions.delete.message', { name: id })}
                          </Text>),
                          labels: {
                            confirm: useTranslateDataGridColumns('actions.delete.buttons.confirm'),
                            cancel: useTranslateDataGridColumns('actions.delete.buttons.cancel')
                          },
                          confirmProps: { color: 'red' },
                          onConfirm: async () => {
                            if (!id) return;
                            await deleteInvantoryEntryById(id);
                          }
                        })
                      }} >
                        <FontAwesomeIcon icon={faTrashCan} />
                      </ActionIcon>
                    </Tooltip>
                  </Group>
                }
              />
            </Group>
        },
      ]}
    />
  );
}

export const Inventory = () => {
  const { createInvantoryEntry } = useDatabaseContext();
  return (
    <Box >
      <PurchaseNewItem onSumit={(__type: string, id: string, quantity: number, price: number, mod_rank: number) => {
        createInvantoryEntry(id, quantity, price, mod_rank);
      }} />
      <Items />
    </Box>
  );
}
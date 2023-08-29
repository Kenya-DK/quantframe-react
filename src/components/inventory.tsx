
import { ActionIcon, Box, Button, Divider, Group, NumberInput, Tooltip, Text, Stack, Paper, Checkbox } from '@mantine/core';
import { useTranslateComponent, useTranslateSuccess } from '@hooks/index';
import { useForm } from '@mantine/form';
import { SearchItemField } from './searchItemField';
import { DataTable } from 'mantine-datatable';
import { useTauriContext } from '../contexts';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCheck, faHammer, faTrashCan } from '@fortawesome/free-solid-svg-icons';
import { modals } from '@mantine/modals';
import { useEffect, useState } from 'react';
import { useMutation } from '@tanstack/react-query';
import api from '@api/index';
import { notifications } from '@mantine/notifications';
import { Trans } from 'react-i18next';

interface PurchaseNewItemProps {
  onSumit: (type: string, id: string, report: boolean, quantity: number, price: number, mod_rank: number) => void;
}
const PurchaseNewItem = (props: PurchaseNewItemProps) => {
  const { onSumit } = props;
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`inventory.${key}`, { ...context })
  const roleForm = useForm({
    initialValues: {
      price: 0,
      item: "",
      quantity: 1,
      rank: 0,
      report: true,
      type: "buy"
    },
    validate: {
      item: (val) => (val.length <= 3 ? ('name_min') : null),
    },
  });
  return (
    <Group grow position="center" >
      <form method="post" onSubmit={roleForm.onSubmit(async (d) => {
        onSumit(d.type, d.item, d.report, d.quantity, d.price, d.rank);
      })}>
        <Stack justify='center' spacing="md">
          <Group grow >
            <SearchItemField value={roleForm.values.item} onChange={(value) => {
              roleForm.setFieldValue('item', value.url_name)
            }} />
            <NumberInput
              required
              label={useTranslateSearch('quantity')}
              description={useTranslateSearch('quantity_description')}
              value={roleForm.values.quantity}
              min={1}
              onChange={(value) => roleForm.setFieldValue('quantity', Number(value))}
              error={roleForm.errors.quantity && 'Invalid identifier'}
            />
            <NumberInput
              required
              label={useTranslateSearch('price')}
              description={useTranslateSearch('price_description')}
              value={roleForm.values.price}
              min={0}
              onChange={(value) => roleForm.setFieldValue('price', Number(value))}
              error={roleForm.errors.price && 'Invalid identifier'}
            />
            <Checkbox checked={roleForm.values.report} description={useTranslateSearch('report_description')} onChange={(event) => roleForm.setFieldValue('report', event.currentTarget.checked)} label={useTranslateSearch('report')} />
            {/* {(selectedItem?.category == "Mods" || selectedItem?.category == "Arcanes") &&
              <NumberInput
                required
                label={useTranslateSearch('rank')}
                description={useTranslateSearch('rank_description')}
                value={roleForm.values.rank}
                min={0}
                max={selectedItem?.max_rank}
                onChange={(value) => roleForm.setFieldValue('rank', Number(value))}
                error={roleForm.errors.rank && 'Invalid identifier'}
              />
            } */}
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
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }) => useTranslateDataGrid(`columns.${key}`, { ...context });
  const useTranslateInvSuccess = (key: string, context?: { [key: string]: any }) => useTranslateSuccess(`invantory.${key}`, { ...context })
  const [total_purchase_price, setTotal_purchase_price] = useState(0);
  const [total_listed_price, setTotal_listed_price] = useState(0);

  const { inventorys } = useTauriContext();
  useEffect(() => {
    if (!inventorys) return;
    setTotal_purchase_price(inventorys.reduce((a, b) => a + (b.price * b.owned), 0));
    setTotal_listed_price(inventorys.reduce((a, b) => a + (b.listed_price || 0), 0));
  }, [inventorys]);


  const [itemPrices, setItemPrices] = useState<Record<string, number>>({});

  const sellInvantoryEntryMutation = useMutation((data: { id: number, report: boolean, price: number }) => api.inventory.sellInvantoryEntry(data.id, data.report, data.price), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateInvSuccess("sell_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateInvSuccess("sell_message", { name: data.item_name, price: data.price }),
        color: "green"
      });
    },
    onError: () => {

    },
  })
  const deleteInvantoryEntryMutation = useMutation((id: number) => api.inventory.deleteInvantoryEntry(id), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateInvSuccess("delete_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateInvSuccess("delete_message", { name: data.item_name }),
        color: "green"
      });
    },
    onError: () => {

    },
  })
  return (
    <Stack>
      <DataTable
        sx={{ marginTop: "20px" }}
        striped
        height={"55vh"}
        records={inventorys}
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
            width: 40,
          },
          {
            accessor: 'listed_price',
            title: useTranslateDataGridColumns('listed_price'),
            width: 60,
          },
          {
            accessor: 'owned',
            title: useTranslateDataGridColumns('owned'),
            width: 40,
          },
          {
            accessor: 'actions',
            width: 100,
            title: useTranslateDataGridColumns('actions.title'),
            render: ({ id, item_url }) =>
              <Group grow position="center" >
                <NumberInput
                  required
                  size='sm'
                  min={0}
                  max={999999}
                  value={itemPrices[item_url] || ""}
                  onChange={(value) => setItemPrices({ ...itemPrices, [item_url]: Number(value) })}
                  rightSectionWidth={100}
                  rightSection={
                    <Group spacing={"5px"} mr={0}>
                      <Divider orientation="vertical" />
                      <Tooltip label={useTranslateDataGridColumns('actions.sell')}>
                        <ActionIcon color="green.7" variant="filled" onClick={async () => {
                          const price = itemPrices[item_url];
                          if (!price || price <= 0 || !id) return;
                          await sellInvantoryEntryMutation.mutateAsync({ id, price, report: false });
                        }} >
                          <FontAwesomeIcon icon={faHammer} />
                        </ActionIcon>
                      </Tooltip>
                      <Tooltip label={useTranslateDataGridColumns('actions.sell_report')}>
                        <ActionIcon color="blue.7" variant="filled" onClick={async () => {
                          const price = itemPrices[item_url];
                          if (!price || price <= 0 || !id) return;
                          await sellInvantoryEntryMutation.mutateAsync({ id, price, report: true });
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
                              await deleteInvantoryEntryMutation.mutateAsync(id);
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
      <Paper mt={5}
        style={{
          display: "flex",
          justifyContent: "center",
          flexDirection: "column",
          alignItems: "center",
        }}>
        <Text size="lg">
          <Trans
            i18nKey={"components.inventory.total_listed_price"}
            values={{ price: total_listed_price }}
            components={{ italic: <Text component="span" size="md" color="blue.3" /> }}
          />
        </Text>
        <Text size="lg">
          <Trans
            i18nKey={"components.inventory.total_purchase_price"}
            values={{ price: total_purchase_price }}
            components={{ italic: <Text component="span" size="md" color="blue.3" /> }}
          />
        </Text>
      </Paper>
    </Stack>

  );
}

export const Inventory = () => {
  const useTranslateInvSuccess = (key: string, context?: { [key: string]: any }) => useTranslateSuccess(`invantory.${key}`, { ...context })
  const createInvantoryEntryMutation = useMutation((data: { id: string, report: boolean, quantity: number, price: number, mod_rank: number }) => api.inventory.createInvantoryEntry(data.id, data.report, data.quantity, data.price, data.mod_rank), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateInvSuccess("create_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateInvSuccess("create_message", { name: data.item_name }),
        color: "green"
      });
    },
    onError: () => {

    },
  })
  return (
    <Box >
      <PurchaseNewItem onSumit={async (__type: string, id: string, report: boolean, quantity: number, price: number, mod_rank: number) => {
        createInvantoryEntryMutation.mutate({ id, report, price, quantity, mod_rank });
      }} />
      <Items />
    </Box>
  );
}
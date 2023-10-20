
import { ActionIcon, Box, Button, Divider, Group, NumberInput, Tooltip, Text, Stack, Checkbox } from '@mantine/core';
import { useTranslateComponent, useTranslateSuccess } from '@hooks/index';
import { useForm } from '@mantine/form';
import { SearchItemField } from './searchItemField';
import { DataTable } from 'mantine-datatable';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCheck, faHammer, faTrashCan } from '@fortawesome/free-solid-svg-icons';
import { modals } from '@mantine/modals';
import { useEffect, useState } from 'react';
import { useMutation } from '@tanstack/react-query';
import api from '@api/index';
import { notifications } from '@mantine/notifications';
import { Trans } from 'react-i18next';
import { useStockContextContext } from '../contexts';
import { CreateStockItemEntryDto, CreateTransactionEntryDto } from '../types';

interface PurchaseNewItemProps {
  loading: boolean;
  onSumit: (data: CreateTransactionEntryDto) => void;
}

const PurchaseNewItem = (props: PurchaseNewItemProps) => {
  const { onSumit, loading } = props;
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
        onSumit({
          transaction_type: d.type,
          item_id: d.item,
          report: d.report,
          price: d.price,
          quantity: d.quantity,
          item_type: "item",
          rank: d.rank
        });
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
            <Tooltip label={useTranslateSearch('report_tooltip')} position="top" withArrow>
              <Checkbox mt={55} checked={roleForm.values.report} onChange={(event) => roleForm.setFieldValue('report', event.currentTarget.checked)} label={useTranslateSearch('report')} />
            </Tooltip>
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
            <Group mt={5} position="center">
              <Tooltip label={useTranslateSearch('buttons.resell_tooltip')} position="top" withArrow>
                <Button loading={loading} type="submit" onClick={() => roleForm.setFieldValue('type', "resell")} disabled={roleForm.values.item.length <= 0} radius="xl">
                  {useTranslateSearch('buttons.resell')}
                </Button>
              </Tooltip>
              <Tooltip label={useTranslateSearch('buttons.buy_tooltip')} position="top" withArrow>
                <Button loading={loading} type="submit" onClick={() => roleForm.setFieldValue('type', "buy")} disabled={roleForm.values.item.length <= 0} radius="xl">
                  {useTranslateSearch('buttons.buy')}
                </Button>
              </Tooltip>
              <Tooltip label={useTranslateSearch('buttons.sell_tooltip')} position="top" withArrow>
                <Button loading={loading} type="submit" onClick={() => roleForm.setFieldValue('type', "sell")} disabled={roleForm.values.item.length <= 0} radius="xl">
                  {useTranslateSearch('buttons.sell')}
                </Button>
              </Tooltip>
            </Group>
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

  const { items: stocks } = useStockContextContext();
  useEffect(() => {
    if (!stocks) return;
    setTotal_purchase_price(stocks.reduce((a, b) => a + (b.price * b.owned), 0));
    setTotal_listed_price(stocks.reduce((a, b) => a + ((b.listed_price || 0) * b.owned), 0));
  }, [stocks]);


  const [itemPrices, setItemPrices] = useState<Record<string, number>>({});

  const sellInvantoryEntryMutation = useMutation((data: { id: number, report: boolean, price: number }) => api.stock.item.sell(data.id, data.report, data.price, 1), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateInvSuccess("sell_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateInvSuccess("sell_message", { name: data.name, price: data.price }),
        color: "green"
      });
    },
    onError: () => {

    },
  })
  const deleteInvantoryEntryMutation = useMutation((id: number) => api.stock.item.delete(id), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateInvSuccess("delete_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateInvSuccess("delete_message", { name: data.name }),
        color: "green"
      });
    },
    onError: () => {

    },
  })
  return (
    <DataTable
      sx={{ marginTop: "20px" }}
      striped
      mah={5}
      height={"65vh"}
      records={stocks}
      // define columns
      columns={[
        {
          accessor: 'name',
          title: useTranslateDataGridColumns('name'),
          width: 120,
        },
        {
          accessor: 'price',
          title: useTranslateDataGridColumns('price'),
          width: 100,
          footer: (
            <Group spacing="xs">
              <Text size="lg">
                <Trans
                  i18nKey={"components.inventory.total_purchase_price"}
                  values={{ price: total_purchase_price }}
                  components={{ italic: <Text component="span" size="md" color="blue.3" /> }}
                />
              </Text>
            </Group>
          ),
        },
        {
          accessor: 'listed_price',
          title: useTranslateDataGridColumns('listed_price'),
          width: 100,
          footer: (
            <Group spacing="xs">
              <Text size="lg">
                <Trans
                  i18nKey={"components.inventory.total_listed_price"}
                  values={{ price: total_listed_price }}
                  components={{ italic: <Text component="span" size="md" color="blue.3" /> }}
                />
              </Text>
            </Group>
          ),
        },
        {
          accessor: 'owned',
          title: useTranslateDataGridColumns('owned'),
          width: 40,
        },
        {
          accessor: 'actions',
          width: 150,
          title: useTranslateDataGridColumns('actions.title'),
          render: ({ id, url }) =>
            <Group grow position="center" >
              <NumberInput
                required
                size='sm'
                min={0}
                max={999}
                value={itemPrices[url] || ""}
                onChange={(value) => setItemPrices({ ...itemPrices, [url]: Number(value) })}
                rightSectionWidth={100}
                rightSection={
                  <Group spacing={"5px"} mr={0}>
                    <Divider orientation="vertical" />
                    <Tooltip label={useTranslateDataGridColumns('actions.sell')}>
                      <ActionIcon disabled={!itemPrices[url]} loading={sellInvantoryEntryMutation.isLoading} color="green.7" variant="filled" onClick={async () => {
                        const price = itemPrices[url];
                        if (!price || price <= 0 || !id) return;
                        await sellInvantoryEntryMutation.mutateAsync({ id, price, report: false });
                      }} >
                        <FontAwesomeIcon icon={faHammer} />
                      </ActionIcon>
                    </Tooltip>
                    <Tooltip label={useTranslateDataGridColumns('actions.sell_report')}>
                      <ActionIcon disabled={!itemPrices[url]} loading={sellInvantoryEntryMutation.isLoading} color="blue.7" variant="filled" onClick={async () => {
                        const price = itemPrices[url];
                        if (!price || price <= 0 || !id) return;
                        await sellInvantoryEntryMutation.mutateAsync({ id, price, report: true });
                      }} >
                        <FontAwesomeIcon icon={faHammer} />
                      </ActionIcon>
                    </Tooltip>
                    <Tooltip label={useTranslateDataGridColumns('actions.delete.title')}>
                      <ActionIcon loading={sellInvantoryEntryMutation.isLoading} color="red.7" variant="filled" onClick={async () => {
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
  );
}

export const Inventory = () => {
  const useTranslateInvSuccess = (key: string, context?: { [key: string]: any }) => useTranslateSuccess(`invantory.${key}`, { ...context })
  const createInvantoryEntryMutation = useMutation((data: CreateStockItemEntryDto) => api.stock.item.create(data), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateInvSuccess("create_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateInvSuccess("create_message", { name: data.name }),
        color: "green"
      });
    },
    onError: () => {

    },
  })
  const createTransactionsEntryMutation = useMutation((data: CreateTransactionEntryDto) => api.transactions.create(data), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateInvSuccess("create_title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateInvSuccess("create_message", { name: data.name }),
        color: "green"
      });
    },
    onError: () => {

    },
  })
  return (
    <Box sx={{}} >
      <PurchaseNewItem loading={createInvantoryEntryMutation.isLoading} onSumit={async (data: CreateTransactionEntryDto) => {
        switch (data.transaction_type) {
          case "buy":
            createTransactionsEntryMutation.mutate(data);
            break;
          case "sell":
            createTransactionsEntryMutation.mutate(data);
            break;
          case "resell":
            createInvantoryEntryMutation.mutate({
              item_id: data.item_id,
              report: data.report || true,
              price: data.price,
              quantity: data.quantity,
              rank: data.rank
            });
            break;
        }
      }} />
      <Items />
    </Box>
  );
}
import { ActionIcon, Box, Grid, Group, Stack, Tooltip, Text, Button } from "@mantine/core";
import { useTranslatePage, useTranslateRustError } from "@hooks/index";
import { TextColor } from "@components/textColor";
import { useLiveScraperContext, useStockContextContext } from "@contexts/index";
import { PurchaseNewItem } from "./purchase";
import { notifications } from "@mantine/notifications";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCheck, faEdit, faEye, faEyeSlash, faHammer, faPen, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useMutation } from "@tanstack/react-query";
import { RustError, CreateStockItemEntryDto, StockItemDto, Wfm } from "$types/index";
import api from '@api/index';
import { useEffect, useState } from "react";
import { DataTable, DataTableSortStatus } from "mantine-datatable";
import { modals } from "@mantine/modals";
import { SendNotificationToWindow, getOrderStatusColorClass, getOrderStatusColorCode, paginate, sortArray } from "@utils/index";
import { SearchField } from "@components/searchfield";
import { InfoBox } from "@components/InfoBox";


interface StockItemsPanelProps {
}
export const StockItemsPanel = ({ }: StockItemsPanelProps) => {
  const useTranslateItemPanel = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`live_trading.tabs.item.${key}`, { ...context }, i18Key)
  const useTranslateNotifaications = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateItemPanel(`notifaications.${key}`, { ...context }, i18Key)
  const useTranslateDataGrid = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateItemPanel(`datagrid.${key}`, { ...context }, i18Key)
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateDataGrid(`columns.${key}`, { ...context }, i18Key);

  const priceIntervals = [5, 10];

  const { items } = useStockContextContext();
  const { message } = useLiveScraperContext();

  // States For DataGrid
  const [page, setPage] = useState(1);
  const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, setPageSize] = useState(pageSizes[4]);
  const [rows, setRows] = useState<StockItemDto[]>([]);
  const [totalRecords, setTotalRecords] = useState<number>(0);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus>({ columnAccessor: 'listed_price', direction: 'desc' });
  const [query, setQuery] = useState<string>("");

  // States For Total Price
  const [totalPurchasePrice, setTotalPurchasePrice] = useState<number>(0);
  const [totalListedPrice, setTotalListedPrice] = useState<number>(0);
  const [totalProfit, setTotalProfit] = useState<number>(0);


  useEffect(() => {
    if (!items) return;
    const totalPurchasePrice = items.reduce((a, b) => a + (b.price || 0) * b.owned, 0);
    const totalListedPrice = items.reduce((a, b) => a + (b.listed_price || 0) * b.owned, 0);
    setTotalPurchasePrice(totalPurchasePrice);
    setTotalListedPrice(totalListedPrice);
    setTotalProfit(totalListedPrice - totalPurchasePrice);
  }, [items])

  // Update DataGrid Rows
  useEffect(() => {
    if (!items)
      return;
    let rivensFilter = items.map((r) => {
      return {
        ...r,
        listed_price: r.listed_price || 0,
      }
    });
    if (query !== "") {
      rivensFilter = rivensFilter.filter((item) => item.name.toLowerCase().includes(query.toLowerCase()));
    }

    setTotalRecords(rivensFilter.length);
    rivensFilter = sortArray([{
      field: sortStatus.columnAccessor,
      direction: sortStatus.direction
    }], rivensFilter);
    rivensFilter = paginate(rivensFilter, page, pageSize);
    setRows(rivensFilter);
  }, [items, query, pageSize, page, sortStatus])


  // Mutations
  const createStockItemEntryMutation = useMutation((data: CreateStockItemEntryDto) => api.stock.item.create(data), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("createStockItem.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("createStockItem.message", { name: data.name }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })

  const sellStockItemEntryMutation = useMutation((data: { id: number, price: number }) => api.stock.item.sell(data.id, data.price, 1), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("sellStockItem.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("sellStockItem.message", { name: data.name, price: data.listed_price }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })
  const deleteStockItemEntryMutation = useMutation((id: number) => api.stock.item.delete(id), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("deleteStockItem.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("deleteStockItem.message", { name: data.name }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })
  const updateItemEntryMutation = useMutation((data: { id: number, item: Partial<StockItemDto> }) => api.stock.item.update(data.id, data.item), {
    onSuccess: async (data) => {
      notifications.show({
        title: useTranslateNotifaications("updateStockItem.title"),
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: useTranslateNotifaications("updateStockItem.message", { name: `${data.name}` }),
        color: "green"
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })
  return (
    <Stack >
      <Grid>
        <Grid.Col span={10}>
          <PurchaseNewItem loading={createStockItemEntryMutation.isLoading} onSumit={async (data: CreateStockItemEntryDto) => {
            createStockItemEntryMutation.mutate({
              item_id: data.item_id,
              price: data.price,
              quantity: data.quantity,
              rank: data.rank
            });
          }} />
          <Group mt={15} >
            <InfoBox text={useTranslateItemPanel("info_boxs.to_low_profit_description")} color={getOrderStatusColorCode(Wfm.OrderStatus.ToLowProfile)} />
            <InfoBox text={useTranslateItemPanel("info_boxs.pending_description")} color={getOrderStatusColorCode(Wfm.OrderStatus.Pending)} />
            <InfoBox text={useTranslateItemPanel("info_boxs.live_description")} color={getOrderStatusColorCode(Wfm.OrderStatus.Live)} />
            <InfoBox text={useTranslateItemPanel("info_boxs.inactive_description")} color={getOrderStatusColorCode(Wfm.OrderStatus.Inactive)} />
            <InfoBox text={useTranslateItemPanel("info_boxs.no_offers_description")} color={getOrderStatusColorCode(Wfm.OrderStatus.NoOffers)} />
            <InfoBox text={useTranslateItemPanel("info_boxs.no_buyers_description")} color={getOrderStatusColorCode(Wfm.OrderStatus.NoBuyers)} />
          </Group>
        </Grid.Col>
        <Grid.Col span={2} >
          <Stack spacing={2} h={"100%"} w={"100%"}
            sx={{
              display: "flex",
              flexDirection: "column",
              justifyContent: "center",
              alignItems: "center",
            }}
          >
            <TextColor size={"lg"} i18nKey={useTranslateItemPanel("total_purchase_price", undefined, true)} values={{ price: totalPurchasePrice }} />
            <TextColor size={"lg"} i18nKey={useTranslateItemPanel("total_listed_price", undefined, true)} values={{ price: totalListedPrice }} />
            <TextColor size={"lg"} i18nKey={useTranslateItemPanel("total_profit", undefined, true)} values={{ price: totalProfit }} />
          </Stack>
        </Grid.Col>
      </Grid>
      <SearchField value={query} onChange={(text) => setQuery(text)} />
      <DataTable
        sx={{ marginTop: "3px" }}
        striped
        mah={5}
        height={`calc(100vh - ${message ? "411px" : "389px"})`}
        withColumnBorders
        records={rows}
        page={page}
        onPageChange={setPage}
        totalRecords={totalRecords}
        rowClassName={(row) => getOrderStatusColorClass(row.status)}
        recordsPerPage={pageSize}
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
            accessor: 'price',
            title: useTranslateDataGridColumns('price'),
            sortable: true,
          },
          {
            accessor: 'minium_price',
            title: useTranslateDataGridColumns('minium_price.title'),
            width: 300,
            sortable: true,
            render: ({ id, minium_price }) => <Group grow position="apart" >
              <Text>{minium_price || "N/A"}</Text>
              <Box w={25} display="flex" sx={{ justifyContent: "flex-end" }}>
                {priceIntervals.map((price, i) =>
                  <Button key={i} mr={10} size="xs" h={22} sx={{ padding: "0px 11px 0px" }} color={"red.7"} onClick={async () => {
                    if (!id) return;
                    const new_price = (minium_price || 0) - price;
                    updateItemEntryMutation.mutateAsync({ id, item: { minium_price: new_price <= 0 ? -1 : new_price } })
                  }} >
                    {`-${price}`}
                  </Button>
                )}
                {priceIntervals.map((price, i) =>
                  <Button key={i} mr={10} size="xs" h={22} sx={{ padding: "0px 11px 0px" }} color={"green.7"} onClick={async () => {
                    if (!id) return;
                    const new_price = (minium_price || 0) + price;
                    updateItemEntryMutation.mutateAsync({ id, item: { minium_price: new_price <= 0 ? -1 : new_price } })
                  }} >
                    {`+${price}`}
                  </Button>
                )}
                <Tooltip label={useTranslateDataGridColumns('minium_price.description')}>
                  <ActionIcon size={"sm"} color={"blue.7"} variant="filled" onClick={async (e) => {
                    e.stopPropagation();
                    modals.openContextModal({
                      modal: 'prompt',
                      title: useTranslateDataGridColumns('minium_price.prompt.title'),
                      innerProps: {
                        fields: [
                          {
                            name: 'minium_price',
                            label: useTranslateDataGridColumns('minium_price.prompt.minium_price_label'),
                            value: minium_price || 0,
                            type: 'number',
                          },
                        ],
                        onConfirm: async (data: { minium_price: number }) => {
                          if (!id) return;
                          const { minium_price } = data;
                          updateItemEntryMutation.mutateAsync({ id, item: { minium_price: minium_price == 0 ? -1 : minium_price } })
                        },
                        onCancel: (id: string) => modals.close(id),
                      },
                    })
                  }} >
                    <FontAwesomeIcon size="xs" icon={faEdit} />
                  </ActionIcon>
                </Tooltip>
              </Box>
            </Group>
          },
          {
            accessor: 'listed_price',
            title: useTranslateDataGridColumns('listed_price'),
            render: ({ listed_price }) => <Text>{listed_price || ""}</Text>
          },
          {
            accessor: 'owned',
            title: useTranslateDataGridColumns('owned'),
            sortable: true,
          },
          {
            accessor: 'actions',
            width: 200,
            title: useTranslateDataGridColumns('actions.title'),
            render: ({ id, listed_price, hidden: hide }) =>
              <Group position="center" >
                <Tooltip label={useTranslateDataGridColumns('actions.sell.title')}>
                  <ActionIcon loading={sellStockItemEntryMutation.isLoading} color="green.7" variant="filled" onClick={async (e) => {
                    e.stopPropagation();
                    modals.openContextModal({
                      modal: 'prompt',
                      title: useTranslateDataGridColumns("actions.sell.prompt.title"),
                      innerProps: {
                        fields: [{ name: 'price', description: useTranslateDataGridColumns("actions.sell.prompt.description"), label: useTranslateDataGridColumns("actions.sell.prompt.label"), type: 'number', value: 0, }],
                        onConfirm: async (data: { price: number }) => {
                          const { price } = data;
                          if (!price || price <= 0 || !id) return;
                          await sellStockItemEntryMutation.mutateAsync({ id: id, price });
                        },
                        onCancel: (id: string) => modals.close(id),
                      },
                    })
                  }} >
                    <FontAwesomeIcon icon={faPen} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label={useTranslateDataGridColumns('actions.sell_for_listed_price')}>
                  <ActionIcon disabled={!listed_price} loading={sellStockItemEntryMutation.isLoading} color="blue.7" variant="filled" onClick={async () => {
                    if (!listed_price || !id) return;
                    await sellStockItemEntryMutation.mutateAsync({ id, price: listed_price });
                  }} >
                    <FontAwesomeIcon icon={faHammer} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label={useTranslateDataGridColumns(`actions.is_hiding.${hide ? "enable" : "disable"}`)}>
                  <ActionIcon loading={sellStockItemEntryMutation.isLoading} color={`${hide ? "red.7" : "green.7"}`} variant="filled" onClick={async () => {
                    if (!id) return;
                    await updateItemEntryMutation.mutateAsync({ id: id, item: { hidden: !hide } });
                  }} >
                    <FontAwesomeIcon icon={hide ? faEye : faEyeSlash} />
                  </ActionIcon>
                </Tooltip>
                <Tooltip label={useTranslateDataGridColumns('actions.delete.title')}>
                  <ActionIcon loading={sellStockItemEntryMutation.isLoading} color="red.7" variant="filled" onClick={async () => {
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
                        await deleteStockItemEntryMutation.mutateAsync(id);
                      }
                    })
                  }} >
                    <FontAwesomeIcon icon={faTrashCan} />
                  </ActionIcon>
                </Tooltip>
              </Group>
          },
        ]}
      />
    </Stack>)
}
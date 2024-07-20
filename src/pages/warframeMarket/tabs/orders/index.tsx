import { Box, Divider, Group, Pagination, ScrollArea, Text, SimpleGrid } from "@mantine/core";
import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useEffect, useState } from "react";
import { Wfm } from "$types/index";
import { useWarframeMarketContextContext } from "@contexts/warframeMarket.context";
import { paginate } from "@utils/helper";
import { faCartShopping, faPen, faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { CreateStockItem, SellByWfmOrder } from "@api/types";
import api from "@api/index";
import { useMutation } from "@tanstack/react-query";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { ColorInfo } from "@components/ColorInfo";
import { Loading } from "@components/Loading";
import { OrderItem } from "@components/OrderItem";
import { SearchField } from "@components/SearchField";

interface OrderPanelProps {
}
export const OrderPanel = ({ }: OrderPanelProps) => {
    // State's
    const [query, setQuery] = useState<string>("");
    const [statusCount, setStatusCount] = useState<{ [key: string]: string }>({}); // Count of each status
    const [page, setPage] = useState(1);
    const pageSizes = [1, 5, 10, 15, 20, 25, 30, 50, 100];
    const [pageSize, _setPageSize] = useState(pageSizes[4]);
    const [totalPages, setTotalPages] = useState(0);
    const [rows, setRows] = useState<Wfm.OrderDto[]>([]);
    const { orders } = useWarframeMarketContextContext();
    const [filterOrderType, setFilterOrderType] = useState<Wfm.OrderType | undefined>(undefined);

    // Translate general
    const useTranslateTabOrder = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`warframe_market.tabs.orders.${key}`, { ...context }, i18Key)
    const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateTabOrder(`buttons.${key}`, { ...context }, i18Key)
    const useTranslateOrderType = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateEnums(`order_type.${key}`, { ...context }, i18Key)
    const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateTabOrder(`prompts.${key}`, { ...context }, i18Key)
    const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateTabOrder(`errors.${key}`, { ...context }, i18Key)
    const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateTabOrder(`success.${key}`, { ...context }, i18Key)

    // Mutations
    const createStockMutation = useMutation({
        mutationFn: (data: CreateStockItem) => api.stock.item.create(data),
        onSuccess: async (u) => {
            notifications.show({ title: useTranslateSuccess("create_stock.title"), message: useTranslateSuccess("create_stock.message", { name: u.item_name }), color: "green.7" });
        },
        onError: (e) => {
            console.error(e);
            notifications.show({ title: useTranslateErrors("create_stock.title"), message: useTranslateErrors("create_stock.message"), color: "red.7" })
        }
    })
    const sellStockMutation = useMutation({
        mutationFn: (data: SellByWfmOrder) => api.stock.item.sellByWfmOrder(data),
        onSuccess: async (u) => {
            notifications.show({ title: useTranslateSuccess("sell_stock.title"), message: useTranslateSuccess("sell_stock.message", { name: u.item_name }), color: "green.7" });
        },
        onError: (e) => {
            console.error(e);
            notifications.show({ title: useTranslateErrors("sell_stock.title"), message: useTranslateErrors("sell_stock.message"), color: "red.7" })
        }
    })
    const refreshOrdersMutation = useMutation({
        mutationFn: () => api.order.refresh(),
        onSuccess: async (u) => {
            notifications.show({ title: useTranslateSuccess("refresh.title"), message: useTranslateSuccess("refresh.message", { count: u }), color: "green.7" });
        },
        onError: (e) => {
            console.error(e);
            notifications.show({ title: useTranslateErrors("refresh.title"), message: useTranslateErrors("refresh.message"), color: "red.7" })
        }
    })
    const deleteOrderMutation = useMutation({
        mutationFn: (id: string) => api.order.delete(id),
        onSuccess: async (name) => {
            notifications.show({ title: useTranslateSuccess("delete.title"), message: useTranslateSuccess("delete.message", { name }), color: "green.7" });
        },
        onError: (e) => {
            console.error(e);
            notifications.show({ title: useTranslateErrors("delete.title"), message: useTranslateErrors("delete.message"), color: "red.7" })
        }
    })
    const deleteAllOrdersMutation = useMutation({
        mutationFn: () => api.order.deleteAll(),
        onSuccess: async (u) => {
            notifications.show({ title: useTranslateSuccess("delete_all.title"), message: useTranslateSuccess("delete_all.message", { count: u }), color: "green.7" });
        },
        onError: (e) => {
            console.error(e);
            notifications.show({ title: useTranslateErrors("delete_all.title"), message: useTranslateErrors("delete_all.message"), color: "red.7" })
        }
    })

    // Update Database Rows
    useEffect(() => {
        let ordersF = orders;
        setStatusCount(() => {
            let items: { [key: string]: string } = {};
            // Create a transaction type count
            Object.values(Wfm.OrderType).forEach((type) => {
                let fOrders = orders.filter((item) => item.order_type === type);
                let total_platinum = fOrders.reduce((acc, item) => acc + (item.platinum * item.quantity), 0);
                items[type] = `${fOrders.length} (${total_platinum})`
            });
            return items
        });

        // Filter by type
        if (filterOrderType)
            ordersF = ordersF.filter((order) => order.order_type === filterOrderType);

        // Filter by query
        if (query)
            ordersF = ordersF.filter((order) => order.item?.en?.item_name.toLowerCase().includes(query.toLowerCase()));

        // Update total pages
        setTotalPages(Math.ceil(ordersF.length / pageSize));

        // Sort by order_type
        ordersF = ordersF.sort((a, b) => {
            if (a.order_type == Wfm.OrderType.Buy && b.order_type == Wfm.OrderType.Sell)
                return 1;
            if (a.order_type == Wfm.OrderType.Sell && b.order_type == Wfm.OrderType.Buy)
                return -1;
            return 0;
        });

        setRows(paginate(ordersF, page, pageSize));
    }, [orders, filterOrderType, query, pageSize, page]);

    // Functions
    const HandleSellOrBuy = async (order: Wfm.OrderDto, price: number) => {
        if (!price) return;
        if (!order || !order.item) return;

        let sub_type = undefined;
        if (order.amber_stars || order.cyan_stars) {
            sub_type = {
                amber_stars: order.amber_stars,
                cyan_stars: order.cyan_stars,
            }
        } else if (order.subtype) {
            sub_type = { variant: order.subtype }
        } else if (order.mod_rank) {
            sub_type = { rank: order.mod_rank }
        }



        switch (order.order_type) {
            case Wfm.OrderType.Buy:
                await createStockMutation.mutateAsync({
                    wfm_url: order.item?.url_name || "",
                    bought: price,
                    quantity: order.quantity,
                    minimum_price: 0,
                    sub_type: sub_type,
                });
                break;
            case Wfm.OrderType.Sell:
                await sellStockMutation.mutateAsync({
                    url: order.item?.url_name || "",
                    sub_type: sub_type,
                    quantity: 1,
                    price: price,
                });
                break;
        }
    }

    const OpenSellOrBuyModal = (order: Wfm.OrderDto) => {
        modals.openContextModal({
            modal: 'prompt',
            title: useTranslatePrompt(`${order.order_type}.title`),
            innerProps: {
                fields: [
                    {
                        name: 'price',
                        label: useTranslatePrompt(`${order.order_type}.field.label`),
                        attributes: {
                            min: 0,
                        },
                        value: 0,
                        type: 'number',
                    },
                ],
                onConfirm: async (data: { price: number }) => {
                    if (!order) return;
                    HandleSellOrBuy(order, data.price);
                },
                onCancel: (id: string) => modals.close(id),
            },
        })
    }

    return (
        <Box>
            <SearchField value={query} onChange={(text) => setQuery(text)}
                rightSectionWidth={63}
                rightSection={
                    <Group gap={5}>
                        <ActionWithTooltip
                            tooltip={useTranslateButtons('refresh.tooltip')}
                            icon={faRefresh}
                            color={"green.7"}
                            actionProps={{ size: "sm" }}
                            iconProps={{ size: "xs" }}
                            loading={createStockMutation.isPending || sellStockMutation.isPending || refreshOrdersMutation.isPending || deleteOrderMutation.isPending || deleteAllOrdersMutation.isPending}
                            onClick={(e) => {
                                e.stopPropagation();
                                refreshOrdersMutation.mutateAsync();
                            }}
                        />
                        <ActionWithTooltip
                            tooltip={useTranslateButtons('delete_all.tooltip')}
                            icon={faTrashCan}
                            loading={createStockMutation.isPending || sellStockMutation.isPending || refreshOrdersMutation.isPending || deleteOrderMutation.isPending || deleteAllOrdersMutation.isPending}
                            color={"red.7"}
                            actionProps={{ size: "sm" }}
                            iconProps={{ size: "xs" }}
                            onClick={(e) => {
                                e.stopPropagation();
                                modals.openConfirmModal({
                                    title: useTranslatePrompt('delete_all.title'),
                                    children: (
                                        <Text size="sm">
                                            {useTranslatePrompt('delete_all.message')}
                                        </Text>
                                    ),
                                    labels: { confirm: useTranslatePrompt('delete_all.confirm'), cancel: useTranslatePrompt('delete.cancel') },
                                    onConfirm: async () => await deleteAllOrdersMutation.mutateAsync(),
                                });
                            }}
                        />
                    </Group>
                }
            />
            <Group gap={"sm"} mt={"md"}>
                {Object.values(Wfm.OrderType).map((type) => (
                    <ColorInfo active={type == filterOrderType} key={type} onClick={() => {
                        setFilterOrderType(s => s === type ? undefined : type);
                    }} infoProps={{
                        "data-color-mode": "bg",
                        "data-order-type": type,
                    }} text={useTranslateOrderType(`${type}`) + `${!statusCount[type] ? "" : ` ${statusCount[type]}`}`} tooltip={useTranslateOrderType(`details.${type}`)} />
                ))}
            </Group>
            <ScrollArea mt={"md"} h={"calc(100vh - 340px)"} >
                {createStockMutation.isPending || sellStockMutation.isPending || refreshOrdersMutation.isPending || deleteOrderMutation.isPending || deleteAllOrdersMutation.isPending ? <Loading /> : null}
                <SimpleGrid
                    cols={4}
                    spacing="lg"
                >
                    {rows.map((order, i) => (
                        <OrderItem
                            key={i}
                            order={order}
                            footer={<>
                                <ActionWithTooltip
                                    tooltip={useTranslateButtons('sell_manual.' + (order.order_type == Wfm.OrderType.Buy ? 'buy.tooltip' : 'sell.tooltip'))}
                                    icon={faPen}
                                    color={"blue.7"}
                                    actionProps={{ size: "sm" }}
                                    iconProps={{ size: "xs" }}
                                    onClick={(e) => {
                                        e.stopPropagation();
                                        OpenSellOrBuyModal(order);
                                    }}
                                />
                                <ActionWithTooltip
                                    tooltip={useTranslateButtons('sell_auto.' + (order.order_type == Wfm.OrderType.Buy ? 'buy.tooltip' : 'sell.tooltip'))}
                                    icon={faCartShopping}
                                    color={"green.7"}
                                    actionProps={{ size: "sm" }}
                                    iconProps={{ size: "xs" }}
                                    onClick={(e) => {
                                        e.stopPropagation();
                                        HandleSellOrBuy(order, order.platinum);
                                    }}
                                />
                                <ActionWithTooltip
                                    tooltip={useTranslateButtons('delete.tooltip')}
                                    icon={faTrashCan}
                                    color={"red.7"}
                                    actionProps={{ size: "sm" }}
                                    iconProps={{ size: "xs" }}
                                    onClick={(e) => {
                                        e.stopPropagation();
                                        modals.openConfirmModal({
                                            title: useTranslatePrompt('delete.title'),
                                            children: (
                                                <Text size="sm">
                                                    {useTranslatePrompt('delete.message', { name: order.item?.en?.item_name })}
                                                </Text>
                                            ),
                                            labels: { confirm: useTranslatePrompt('delete.confirm'), cancel: useTranslatePrompt('delete.cancel') },
                                            onConfirm: async () => await deleteOrderMutation.mutateAsync(order.id),
                                        });
                                    }}
                                />
                            </>}

                        />
                    ))}
                </SimpleGrid>
            </ScrollArea>
            <Divider mt={"md"} />
            <Group grow mt={"md"}>
                <Group>
                </Group>
                <Group justify="flex-end">
                    <Pagination value={page} onChange={setPage} total={totalPages} />
                </Group>
            </Group>
        </Box>
    );
};
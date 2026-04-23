import { WFMarketTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useStockMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };

  const refreshOrdersMutation = createGenericMutation(
    {
      mutationFn: () => api.order.refreshOrders(),
      successKey: "refresh_orders",
      errorKey: "refresh_orders",
    },
    hooks,
  );

  const deleteAllOrdersMutation = createGenericMutation(
    {
      mutationFn: (order_type?: WFMarketTypes.OrderType) => api.order.deleteAllOrders(order_type),
      successKey: "delete_all_orders",
      errorKey: "delete_all_orders",
    },
    hooks,
  );

  const createStockMutation = createGenericMutation(
    {
      mutationFn: (data: WFMarketTypes.Order) =>
        api.stock_item.create(
          {
            raw: data.itemId,
            quantity: data.quantity,
            sub_type: data,
            bought: data.platinum,
          },
          "id",
        ),
      successKey: "create_stock_item",
      errorKey: "create_stock_item",
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks,
  );

  const sellStockMutation = createGenericMutation(
    {
      mutationFn: (data: WFMarketTypes.Order) =>
        api.stock_item.sell(
          {
            id: -1,
            wfm_url: data.itemId,
            sub_type: data,
            price: data.platinum,
            quantity: data.quantity,
          },
          "id",
        ),
      successKey: "sell_stock_item",
      errorKey: "sell_stock_item",
      getLoadingId: (variables: WFMarketTypes.Order) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks,
  );

  const deleteStockMutation = createGenericMutation(
    {
      mutationFn: (id: string) => api.order.deleteById(id),
      successKey: "delete_order",
      errorKey: "delete_order",
      getLoadingId: (variables: string) => `${variables}`,
    },
    hooks,
  );

  return {
    refreshOrdersMutation,
    deleteAllOrdersMutation,
    createStockMutation,
    sellStockMutation,
    deleteStockMutation,
  };
};

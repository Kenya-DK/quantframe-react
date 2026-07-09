import { TauriTypes, WFMarketTypes } from "$types";
import api from "@api/index";
import { useAppContext } from "@contexts/app.context";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useStockMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };
  const { settings } = useAppContext();

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

  const blacklistOrderMutation = createGenericMutation(
    {
      mutationFn: async (order: WFMarketTypes.Order) => {
        if (!settings) throw new Error("Settings are not loaded");
        const wfm_id = order.properties?.wfm_id || order.itemId;
        const allModes = [TauriTypes.TradeMode.Buy, TauriTypes.TradeMode.Sell, TauriTypes.TradeMode.Wishlist];
        const blacklist = settings.live_scraper.items.general.blacklist || [];
        const updatedBlacklist = [...blacklist.filter((b) => b.wfmId !== wfm_id), { wfmId: wfm_id, disabled_for: allModes }];
        await api.app.updateSettings({
          ...settings,
          live_scraper: {
            ...settings.live_scraper,
            items: {
              ...settings.live_scraper.items,
              general: { ...settings.live_scraper.items.general, blacklist: updatedBlacklist },
            },
          },
        });
        return api.order.deleteById(order.id);
      },
      successKey: "blacklist_order",
      errorKey: "blacklist_order",
      getLoadingId: (order: WFMarketTypes.Order) => `${order.id}`,
    },
    hooks,
  );

  return {
    refreshOrdersMutation,
    deleteAllOrdersMutation,
    createStockMutation,
    sellStockMutation,
    deleteStockMutation,
    blacklistOrderMutation,
  };
};

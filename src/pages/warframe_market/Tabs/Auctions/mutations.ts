import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { WFMarketTypes } from "$types";
import api from "@api/index";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

interface MutationHooks {
  useTranslateSuccess: (key: string, context?: { [key: string]: any }) => string;
  useTranslateErrors: (key: string, context?: { [key: string]: any }) => string;
  refetchQueries: (refetchStatus?: boolean) => void;
  setLoadingRows: (callback: (prev: string[]) => string[]) => void;
}

// Generic mutation creator function
const createGenericMutation = <TData, TVariables>(
  config: {
    mutationFn: (data: TVariables) => Promise<TData>;
    successKey: string;
    errorKey: string;
    translateCommon?: boolean;
    getLoadingId?: (variables: TVariables) => string | string[];
    getSuccessMessage?: (data: TData, variables: TVariables) => { [key: string]: any };
  },
  hooks: MutationHooks
) => {
  return useMutation({
    mutationFn: config.mutationFn,
    onMutate: config.getLoadingId
      ? (variables: TVariables) => {
          const loadingIds = config.getLoadingId!(variables);
          const ids = Array.isArray(loadingIds) ? loadingIds : [loadingIds];
          hooks.setLoadingRows((prev: string[]) => [...prev, ...ids]);
        }
      : undefined,
    onSettled: config.getLoadingId
      ? (_data: TData | undefined, _error: any, variables: TVariables) => {
          const loadingIds = config.getLoadingId!(variables);
          const ids = Array.isArray(loadingIds) ? loadingIds : [loadingIds];
          hooks.setLoadingRows((prev: string[]) => prev.filter((id: string) => !ids.includes(id)));
        }
      : undefined,
    onSuccess: (data: TData, variables: TVariables) => {
      let refetchStatusString = ["create_stock_riven", "refresh_auctions", "delete_all_auctions"];
      hooks.refetchQueries(refetchStatusString.includes(config.successKey));
      notifications.show({
        title: config.translateCommon
          ? useTranslateCommon(`notifications.${config.successKey}.success.title`)
          : hooks.useTranslateSuccess(`${config.successKey}.title`),
        message: config.translateCommon
          ? useTranslateCommon(
              `notifications.${config.successKey}.success.message`,
              config.getSuccessMessage ? config.getSuccessMessage(data, variables) : {}
            )
          : hooks.useTranslateSuccess(`${config.successKey}.message`, config.getSuccessMessage ? config.getSuccessMessage(data, variables) : {}),
        color: "green.7",
      });
    },
    onError: (e: any) => {
      console.error(e);
      notifications.show({
        title: config.translateCommon
          ? useTranslateCommon(`notifications.${config.errorKey}.error.title`)
          : hooks.useTranslateErrors(`${config.errorKey}.title`),
        message: config.translateCommon
          ? useTranslateCommon(`notifications.${config.errorKey}.error.message`, { error: e })
          : hooks.useTranslateErrors(`${config.errorKey}.message`, { error: e }),
        color: "red.7",
      });
    },
  });
};

export const useStockMutations = ({ useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows };

  const refreshAuctionsMutation = createGenericMutation(
    {
      mutationFn: () => api.auction.refreshAuctions(),
      successKey: "refresh_auctions",
      errorKey: "refresh_auctions",
    },
    hooks
  );

  const deleteAllAuctionsMutation = createGenericMutation(
    {
      mutationFn: () => api.auction.deleteAllAuctions(),
      successKey: "delete_all_auctions",
      errorKey: "delete_all_auctions",
    },
    hooks
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
          "id"
        ),
      successKey: "create_stock_riven",
      errorKey: "create_stock_riven",
      translateCommon: true,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
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
            quantity: 1,
          },
          "id"
        ),
      successKey: "sell_stock_riven",
      errorKey: "sell_stock_riven",
      translateCommon: true,
      getLoadingId: (variables: WFMarketTypes.Order) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const deleteStockMutation = createGenericMutation(
    {
      mutationFn: (id: string) => api.order.deleteById(id),
      successKey: "delete_auction",
      errorKey: "delete_auction",
      getLoadingId: (variables: string) => `${variables}`,
    },
    hooks
  );

  return {
    refreshAuctionsMutation,
    deleteAllAuctionsMutation,
    createStockMutation,
    sellStockMutation,
    deleteStockMutation,
  };
};

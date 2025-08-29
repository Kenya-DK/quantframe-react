import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { TauriTypes } from "$types";
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
      let refetchStatusString = ["create_stock_item"];
      hooks.refetchQueries(refetchStatusString.includes(config.successKey));
      notifications.show({
        title: useTranslateCommon(`notifications.${config.successKey}.success.title`),
        message: useTranslateCommon(
          `notifications.${config.successKey}.success.message`,
          config.getSuccessMessage ? config.getSuccessMessage(data, variables) : {}
        ),
        color: "green.7",
      });
    },
    onError: (e: any) => {
      console.error(e);
      notifications.show({
        title: useTranslateCommon(`notifications.${config.errorKey}.error.title`),
        message: useTranslateCommon(`notifications.${config.errorKey}.error.message`),
        color: "red.7",
      });
    },
  });
};

export const useStockMutations = ({ useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows };

  const createStockMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.CreateStockItem) => api.stock_item.create(data),
      successKey: "create_stock_item",
      errorKey: "create_stock_item",
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const updateStockMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.UpdateStockItem) => api.stock_item.update(data),
      successKey: "update_stock_item",
      errorKey: "update_stock_item",
      getLoadingId: (variables: TauriTypes.UpdateStockItem) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const sellStockMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.SellStockItem) => api.stock_item.sell(data),
      successKey: "sell_stock_item",
      errorKey: "sell_stock_item",
      getLoadingId: (variables: TauriTypes.SellStockItem) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const deleteStockMutation = createGenericMutation(
    {
      mutationFn: (id: number) => api.stock_item.delete(id),
      successKey: "delete_stock_item",
      errorKey: "delete_stock_item",
      getLoadingId: (variables: number) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  return {
    createStockMutation,
    updateStockMutation,
    sellStockMutation,
    deleteStockMutation,
  };
};

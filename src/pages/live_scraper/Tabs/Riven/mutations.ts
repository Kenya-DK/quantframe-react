import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { TauriTypes } from "$types";
import api from "@api/index";
import { useTranslateCommon } from "../../../../hooks/useTranslate.hook";

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
      let refetchStatusString = ["create_stock"];
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
        title: useTranslateCommon(`notifications.${config.errorKey}.title`),
        message: useTranslateCommon(`notifications.${config.errorKey}.message`),
        color: "red.7",
      });
    },
  });
};

export const useStockMutations = ({ useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows };
  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.StockItemControllerGetListParams) => api.stock_riven.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks
  );
  const createStockMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.CreateStockRiven) => api.stock_riven.create(data),
      successKey: "create_stock_riven",
      errorKey: "create_stock_riven",
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks
  );

  const updateStockMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.UpdateStockRiven) => api.stock_riven.update(data),
      successKey: "update_stock_riven",
      errorKey: "update_stock_riven",
      getLoadingId: (variables: TauriTypes.UpdateStockRiven) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks
  );

  const sellStockMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.SellStockRiven) => api.stock_riven.sell(data),
      successKey: "sell_stock_riven",
      errorKey: "sell_stock_riven",
      getLoadingId: (variables: TauriTypes.SellStockRiven) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks
  );
  44;
  const deleteStockMutation = createGenericMutation(
    {
      mutationFn: (id: number) => api.stock_riven.delete(id),
      successKey: "delete_stock_riven",
      errorKey: "delete_stock_riven",
      getLoadingId: (variables: number) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks
  );

  return {
    exportMutation,
    createStockMutation,
    updateStockMutation,
    sellStockMutation,
    deleteStockMutation,
  };
};

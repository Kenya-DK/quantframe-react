import { TauriTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };
  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.TradeEntryControllerGetListParams) => api.trade_entry.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks
  );
  const createMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.CreateTradeEntry) => api.trade_entry.create(data),
      successKey: "create_trade_entry",
      errorKey: "create_trade_entry",
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const createMultipleMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.CreateTradeEntry[]) => api.trade_entry.createMultiple(data),
      successKey: "create_trade_entry",
      errorKey: "create_trade_entry",
      isMultiple: (variables: TauriTypes.CreateTradeEntry[]) => variables.length > 1,
      getSuccessMessage: (data: any) => ({ count: data }),
    },
    hooks
  );

  const updateMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.UpdateTradeEntry) => api.trade_entry.update(data),
      successKey: "update_trade_entry",
      errorKey: "update_trade_entry",
      getLoadingId: (variables: TauriTypes.UpdateTradeEntry) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const updateMultipleMutation = createGenericMutation(
    {
      mutationFn: (data: { ids: number[]; input: TauriTypes.UpdateTradeEntry }) => api.trade_entry.updateMultiple(data.ids, data.input),
      successKey: "update_trade_entry",
      errorKey: "update_trade_entry",
      isMultiple: (variables: { ids: number[]; input: TauriTypes.UpdateTradeEntry }) => variables.ids.length > 1,
      getLoadingId: (variables: { ids: number[]; input: TauriTypes.UpdateTradeEntry }) => variables.ids.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data.length }),
    },
    hooks
  );

  const deleteMutation = createGenericMutation(
    {
      mutationFn: (id: number) => api.trade_entry.delete(id),
      successKey: "delete_trade_entry",
      errorKey: "delete_trade_entry",
      getLoadingId: (variables: number) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const deleteMultipleMutation = createGenericMutation(
    {
      mutationFn: (ids: number[]) => api.trade_entry.deleteMultiple(ids),
      successKey: "delete_trade_entry",
      errorKey: "delete_trade_entry",
      isMultiple: (variables: number[]) => variables.length > 1,
      getLoadingId: (variables: number[]) => variables.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data }),
    },
    hooks
  );

  return {
    exportMutation,
    createMutation,
    createMultipleMutation,
    updateMutation,
    updateMultipleMutation,
    deleteMutation,
    deleteMultipleMutation,
  };
};

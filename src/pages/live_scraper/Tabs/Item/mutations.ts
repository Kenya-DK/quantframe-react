import { TauriTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };
  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.StockItemControllerGetListParams) => api.stock_item.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks
  );
  const createMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.CreateStockItem) => api.stock_item.create(data),
      successKey: "create_stock_item",
      errorKey: "create_stock_item",
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const updateMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.UpdateStockItem) => api.stock_item.update(data),
      successKey: "update_stock_item",
      errorKey: "update_stock_item",
      getLoadingId: (variables: TauriTypes.UpdateStockItem) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const updateMultipleMutation = createGenericMutation(
    {
      mutationFn: (data: { ids: number[]; input: TauriTypes.UpdateStockItem }) => api.stock_item.updateMultiple(data.ids, data.input),
      successKey: "update_stock_item",
      errorKey: "update_stock_item",
      isMultiple: (variables: { ids: number[]; input: TauriTypes.UpdateStockItem }) => variables.ids.length > 1,
      getLoadingId: (variables: { ids: number[]; input: TauriTypes.UpdateStockItem }) => variables.ids.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data.length }),
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

  const deleteMutation = createGenericMutation(
    {
      mutationFn: (id: number) => api.stock_item.delete(id),
      successKey: "delete_stock_item",
      errorKey: "delete_stock_item",
      getLoadingId: (variables: number) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const deleteMultipleMutation = createGenericMutation(
    {
      mutationFn: (ids: number[]) => api.stock_item.deleteMultiple(ids),
      successKey: "delete_stock_item",
      errorKey: "delete_stock_item",
      isMultiple: (variables: number[]) => variables.length > 1,
      getLoadingId: (variables: number[]) => variables.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data }),
    },
    hooks
  );

  return {
    exportMutation,
    createMutation,
    updateMutation,
    updateMultipleMutation,
    sellStockMutation,
    deleteMutation,
    deleteMultipleMutation,
  };
};

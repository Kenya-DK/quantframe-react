import { TauriTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useStockMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };
  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.StockItemControllerGetListParams) => api.stock_riven.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks
  );
  const createMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.CreateStockRiven) => api.stock_riven.create(data),
      successKey: "create_stock_riven",
      errorKey: "create_stock_riven",
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks
  );

  const updateMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.UpdateStockRiven) => api.stock_riven.update(data),
      successKey: "update_stock_riven",
      errorKey: "update_stock_riven",
      getLoadingId: (variables: TauriTypes.UpdateStockRiven) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks
  );
  const updateMultipleMutation = createGenericMutation(
    {
      mutationFn: (data: { ids: number[]; input: TauriTypes.UpdateStockRiven }) => api.stock_riven.updateMultiple(data.ids, data.input),
      successKey: "update_stock_riven",
      errorKey: "update_stock_riven",
      isMultiple: (variables: { ids: number[]; input: TauriTypes.UpdateStockItem }) => variables.ids.length > 1,
      getLoadingId: (variables: { ids: number[]; input: TauriTypes.UpdateStockItem }) => variables.ids.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data.length }),
    },
    hooks
  );
  const sellMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.SellStockRiven) => api.stock_riven.sell(data),
      successKey: "sell_stock_riven",
      errorKey: "sell_stock_riven",
      getLoadingId: (variables: TauriTypes.SellStockRiven) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks
  );

  const deleteMutation = createGenericMutation(
    {
      mutationFn: (id: number) => api.stock_riven.delete(id),
      successKey: "delete_stock_riven",
      errorKey: "delete_stock_riven",
      getLoadingId: (variables: number) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks
  );
  const deleteMultipleMutation = createGenericMutation(
    {
      mutationFn: (ids: number[]) => api.stock_riven.deleteMultiple(ids),
      successKey: "delete_stock_riven",
      errorKey: "delete_stock_riven",
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
    sellMutation,
    deleteMutation,
    deleteMultipleMutation,
  };
};

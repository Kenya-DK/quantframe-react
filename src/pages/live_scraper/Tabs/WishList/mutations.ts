import { TauriTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useWishListMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };

  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.WishListControllerGetListParams) => api.wish_list.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks
  );
  const createMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.CreateWishListItem) => api.wish_list.create(data),
      successKey: "create_wish_list",
      errorKey: "create_wish_list",
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const updateMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.UpdateWishListItem) => api.wish_list.update(data),
      successKey: "update_wish_list",
      errorKey: "update_wish_list",
      getLoadingId: (variables: TauriTypes.UpdateWishListItem) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );
  const updateMultipleMutation = createGenericMutation(
    {
      mutationFn: (data: { ids: number[]; input: TauriTypes.UpdateWishListItem }) => api.wish_list.updateMultiple(data.ids, data.input),
      successKey: "update_wish_list",
      errorKey: "update_wish_list",
      isMultiple: (variables: { ids: number[]; input: TauriTypes.UpdateWishListItem }) => variables.ids.length > 1,
      getLoadingId: (variables: { ids: number[]; input: TauriTypes.UpdateWishListItem }) => variables.ids.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data.length }),
    },
    hooks
  );
  const deleteMutation = createGenericMutation(
    {
      mutationFn: (id: number) => api.wish_list.delete(id),
      successKey: "delete_wish_list",
      errorKey: "delete_wish_list",
      getLoadingId: (variables: number) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );
  const deleteMultipleMutation = createGenericMutation(
    {
      mutationFn: (ids: number[]) => api.wish_list.deleteMultiple(ids),
      successKey: "delete_wish_list",
      errorKey: "delete_wish_list",
      isMultiple: (variables: number[]) => variables.length > 1,
      getLoadingId: (variables: number[]) => variables.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data }),
    },
    hooks
  );
  const boughtMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.BoughtWishListItem) => api.wish_list.bought(data),
      successKey: "bought_wish_list",
      errorKey: "bought_wish_list",
      getLoadingId: (variables: TauriTypes.BoughtWishListItem) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );
  return {
    exportMutation,
    createMutation,
    updateMutation,
    updateMultipleMutation,
    boughtMutation,
    deleteMutation,
    deleteMultipleMutation,
  };
};

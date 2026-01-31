import { TauriTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };

  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.TransactionControllerGetListParams) => api.transaction.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks,
  );

  const updateMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.UpdateTransaction) => api.transaction.update(data),
      successKey: "update_transaction",
      errorKey: "update_transaction",
      getLoadingId: (variables: TauriTypes.UpdateTransaction) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks,
  );

  const deleteMutation = createGenericMutation(
    {
      mutationFn: (id: number) => api.transaction.delete(id),
      successKey: "delete_transaction",
      errorKey: "delete_transaction",
      getLoadingId: (variables: number) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks,
  );

  const deleteMultipleMutation = createGenericMutation(
    {
      mutationFn: (ids: number[]) => api.transaction.deleteBulk(ids),
      successKey: "delete_transaction",
      errorKey: "delete_transaction",
      isMultiple: (variables: number[]) => variables.length > 1,
      getLoadingId: (variables: number[]) => variables.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data }),
    },
    hooks,
  );

  const calculateTaxMutation = createGenericMutation(
    {
      mutationFn: () => api.transaction.calculateTax(),
      successKey: "calculate_tax_transaction",
      errorKey: "calculate_tax_transaction",
      getSuccessMessage: () => ({}),
    },
    hooks,
  );

  return {
    exportMutation,
    updateMutation,
    deleteMutation,
    deleteMultipleMutation,
    calculateTaxMutation,
  };
};

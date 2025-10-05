import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { TauriTypes } from "$types";
import api from "@api/index";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

interface MutationHooks {
  refetchQueries: (refetchStatus?: boolean) => void;
  setLoadingRows: (callback: (prev: string[]) => string[]) => void;
  setSelectedRecords?: (callback: (prev: TauriTypes.TransactionDto[]) => TauriTypes.TransactionDto[]) => void;
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
      let refetchStatusString = ["update_transaction", "delete_bulk_transaction"];
      hooks.refetchQueries(refetchStatusString.includes(config.successKey));
      if (hooks.setSelectedRecords && refetchStatusString.includes(config.successKey)) {
        console.log("Resetting selected records");
        hooks.setSelectedRecords(() => []);
      }
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

export const useMutations = ({ refetchQueries, setLoadingRows, setSelectedRecords }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows, setSelectedRecords };

  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.TransactionControllerGetListParams) => api.transaction.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks
  );

  const updateMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.UpdateTransaction) => api.transaction.update(data),
      successKey: "update_transaction",
      errorKey: "update_transaction",
      getLoadingId: (variables: TauriTypes.UpdateTransaction) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const deleteMutation = createGenericMutation(
    {
      mutationFn: (id: number) => api.transaction.delete(id),
      successKey: "delete_transaction",
      errorKey: "delete_transaction",
      getLoadingId: (variables: number) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  const deleteBulkMutation = createGenericMutation(
    {
      mutationFn: (ids: number[]) => api.transaction.deleteBulk(ids),
      successKey: "delete_bulk_transaction",
      errorKey: "delete_bulk_transaction",
      getLoadingId: (variables: number[]) => variables.map((id) => `${id}`),
      getSuccessMessage: (data: any) => ({ count: data.deleted_count }),
    },
    hooks
  );

  return {
    exportMutation,
    updateMutation,
    deleteMutation,
    deleteBulkMutation,
  };
};

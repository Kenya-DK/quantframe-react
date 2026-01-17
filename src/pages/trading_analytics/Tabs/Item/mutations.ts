import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { QuantframeApiTypes } from "$types";
import api from "@api/index";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

interface MutationHooks {
  refetchQueries: (refetchStatus?: boolean) => void;
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
    onMutate: config.getLoadingId ? () => {} : undefined,
    onSettled: config.getLoadingId ? (_data: TData | undefined, _error: any) => {} : undefined,
    onSuccess: (data: TData, variables: TVariables) => {
      let refetchStatusString = ["update_transaction", "delete_bulk_transaction"];
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

export const useMutations = ({ refetchQueries }: MutationHooks) => {
  const hooks = { refetchQueries };

  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: QuantframeApiTypes.ItemPriceControllerGetListParams) => api.item.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks
  );

  return {
    exportMutation,
  };
};

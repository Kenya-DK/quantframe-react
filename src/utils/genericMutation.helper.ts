import { notifications } from "@mantine/notifications";
import { useMutation } from "@tanstack/react-query";
import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { Dispatch, SetStateAction } from "react";

export interface MutationHooks {
  refetchQueries: (refetchStatus?: boolean) => void;
  setLoadingRows: (callback: (prev: string[]) => string[]) => void;
}
// Generic mutation creator function
export const createGenericMutation = <TData, TVariables>(
  config: {
    mutationFn: (data: TVariables) => Promise<TData>;
    successKey: string;
    errorKey: string;
    isMultiple?: (variables: TVariables) => boolean;
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
      const isMultiple = config.isMultiple ? config.isMultiple(variables) : false;
      notifications.show({
        title: useTranslateCommon(`notifications.${config.successKey}.success.title${isMultiple ? "_multiple" : ""}`),
        message: useTranslateCommon(
          `notifications.${config.successKey}.success.message${isMultiple ? "_multiple" : ""}`,
          config.getSuccessMessage ? config.getSuccessMessage(data, variables) : {}
        ),
        color: "green.7",
      });
    },
    onError: (e: any, variables: TVariables) => {
      console.error(e);
      const isMultiple = config.isMultiple ? config.isMultiple(variables) : false;
      notifications.show({
        title: useTranslateCommon(`notifications.${config.errorKey}.error.title${isMultiple ? "_multiple" : ""}`),
        message: useTranslateCommon(`notifications.${config.errorKey}.error.message${isMultiple ? "_multiple" : ""}`),
        color: "red.7",
      });
    },
  });
};

import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
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
      let refetchStatusString = ["delete_chat"];
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

export const useMutations = ({ useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows };

  const deleteMutation = createGenericMutation(
    {
      mutationFn: (id: string) => api.chat.delete(id),
      successKey: "delete_chat",
      errorKey: "delete_chat",
      getLoadingId: (variables: string) => `${variables}`,
      getSuccessMessage: (data: any) => ({ name: data.item_name }),
    },
    hooks
  );

  return {
    deleteMutation,
  };
};

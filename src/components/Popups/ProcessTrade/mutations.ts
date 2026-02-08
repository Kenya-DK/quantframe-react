import { TauriTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };

  const createMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.HandleItem[]) => api.handlers.handle_items(data),
      successKey: "processed_multiple_items",
      errorKey: "processed_multiple_items",
      getSuccessMessage: (data: any) => ({ count: data }),
    },
    hooks,
  );

  return {
    createMutation,
  };
};

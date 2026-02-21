import { QuantframeApiTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useMutations = ({ refetchQueries }: Omit<MutationHooks, "refreshSettings" | "setLoadingRows">) => {
  const hooks = { refetchQueries, refreshSettings: () => refetchQueries(true), setLoadingRows: () => {} };

  const exportMutation = createGenericMutation(
    {
      mutationFn: (data: QuantframeApiTypes.ItemPriceControllerGetListParams) => api.item.exportJson(data),
      successKey: "export_data",
      errorKey: "export_data",
      getSuccessMessage: (data: any) => ({ path: data }),
    },
    hooks,
  );

  return {
    exportMutation,
  };
};

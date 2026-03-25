import { TauriTypes } from "$types";
import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };

  const createMutation = createGenericMutation(
    {
      mutationFn: (data: TauriTypes.CreateStockRiven) => api.stock_riven.create(data),
      successKey: "create_stock_riven",
      errorKey: "create_stock_riven",
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks,
  );
  return { createMutation };
};

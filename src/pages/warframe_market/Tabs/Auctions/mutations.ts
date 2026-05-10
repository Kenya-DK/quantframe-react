import api from "@api/index";
import { createGenericMutation, MutationHooks } from "@utils/genericMutation.helper";

export const useStockMutations = ({ refetchQueries, setLoadingRows }: MutationHooks) => {
  const hooks = { refetchQueries, setLoadingRows };

  const refreshAuctionsMutation = createGenericMutation(
    {
      mutationFn: () => api.auction.refreshAuctions(),
      successKey: "refresh_auctions",
      errorKey: "refresh_auctions",
    },
    hooks,
  );

  const deleteAllAuctionsMutation = createGenericMutation(
    {
      mutationFn: () => api.auction.deleteAllAuctions(),
      successKey: "delete_all_auctions",
      errorKey: "delete_all_auctions",
    },
    hooks,
  );

  const deleteStockMutation = createGenericMutation(
    {
      mutationFn: (id: string) => api.auction.deleteById(id),
      successKey: "delete_auction",
      errorKey: "delete_auction",
      getLoadingId: (variables: string) => `${variables}`,
    },
    hooks,
  );
  const importStockMutation = createGenericMutation(
    {
      mutationFn: (data: { id: string; bought: number }) => api.auction.importById(data.id, data.bought),
      successKey: "create_stock_riven",
      errorKey: "create_stock_riven",
      getLoadingId: (variables: { id: string; bought: number }) => `${variables.id}`,
      getSuccessMessage: (data: any) => ({ name: `${data.weapon_name} ${data.mod_name}` }),
    },
    hooks,
  );

  return {
    refreshAuctionsMutation,
    deleteAllAuctionsMutation,
    deleteStockMutation,
    importStockMutation,
  };
};

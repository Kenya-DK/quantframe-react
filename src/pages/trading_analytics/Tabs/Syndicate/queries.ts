import { QuantframeApiTypes, ResponseError } from "$types";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";

interface QueriesHooks {
  queryData: QuantframeApiTypes.SyndicateItemPriceControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_item_pagination", queryData.page, queryData.limit, queryData.query, queryData.sort_by, queryData.sort_direction],
    queryFn: () => api.syndicate.getAll(queryData),
    retry: false,
    throwOnError(error: ResponseError, query) {
      console.error("Error in query:", query.queryKey, error);
      return false;
    },
    enabled: isActive,
  });
  const refetchQueries = () => {
    getPaginationQuery.refetch();
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    refetchQueries,
  };
};

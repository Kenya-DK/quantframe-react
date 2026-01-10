import { useQuery } from "@tanstack/react-query";
import { ResponseError, TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.WFGDPRTransactionControllerGetListParams;
  isActive?: boolean;
}
export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: [
      "get_wfgdpr_transaction_pagination",
      // queryData.from_date,
      // queryData.to_date,
      queryData.page,
      queryData.limit,
      queryData.query,
      queryData.sort_by,
      queryData.sort_direction,
    ],
    queryFn: () => api.log_parser.getTransactionPagination(queryData),
    retry: false,
    enabled: isActive === true,
    throwOnError(error: ResponseError, query) {
      console.error("Error in query:", query.queryKey, error);
      return false;
    },
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

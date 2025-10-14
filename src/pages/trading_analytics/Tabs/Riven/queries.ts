import { useQuery } from "@tanstack/react-query";
import { QuantframeApiTypes, ResponseError } from "$types";
import api from "@api/index";
import { notifications } from "@mantine/notifications";

interface QueriesHooks {
  queryData: QuantframeApiTypes.RivenPriceControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const IsValid = () => {
    if (queryData.to_date && queryData.from_date && isActive) {
      const fromDate = new Date(queryData.from_date);
      const toDate = new Date(queryData.to_date);
      return fromDate <= toDate;
    }
    return false;
  };
  // Pagination Query
  const getPaginationQuery = useQuery({
    queryKey: [
      "get_riven_pagination",
      queryData.from_date,
      queryData.to_date,
      queryData.page,
      queryData.limit,
      queryData.query,
      queryData.sort_by,
      queryData.sort_direction,
    ],
    queryFn: () => api.riven.getAll(queryData),
    retry: false,
    throwOnError(error: ResponseError, query) {
      if (error.context?.error.status_code === 429) {
        notifications.show({
          title: "Error 429",
          color: "red.7",
          message: "You are sending too many requests. Please try again later.",
        });
      }
      console.error("Error in query:", query.queryKey, error);
      return false;
    },
    enabled: IsValid(),
  });
  const refetchQueries = () => {
    if (!IsValid()) return;
    getPaginationQuery.refetch();
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    refetchQueries,
  };
};

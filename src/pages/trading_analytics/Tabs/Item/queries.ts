import { useQuery } from "@tanstack/react-query";
import { QuantframeApiTypes, ResponseError } from "$types";
import api from "@api/index";
import { notifications } from "@mantine/notifications";

interface QueriesHooks {
  queryData: QuantframeApiTypes.ItemPriceControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_item_pagination", queryData],
    queryFn: () => api.item.getAll(queryData),
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

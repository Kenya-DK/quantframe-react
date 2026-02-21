import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.EELogControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_ee_logs", queryData],
    queryFn: () => api.debug.get_ee_logs(queryData),
    retry: false,
    enabled: isActive,
  });

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
  };
};

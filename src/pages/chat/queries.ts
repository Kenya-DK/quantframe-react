import { useQuery } from "@tanstack/react-query";
import { WFMarketTypes } from "$types";
import api from "@api/index";
import { queryClient } from "../../App";
interface QueriesHooks {
  queryData: WFMarketTypes.WfmChatDataControllerGetListParams;
}

export const useChatQueries = ({ queryData }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_chat_pagination", queryData],
    queryFn: () => api.chat.getPagination(queryData),
    retry: false,
    enabled: true,
  });
  const refetchQueries = () => {
    getPaginationQuery.refetch();
  };

  const updateChatData = (chat: Partial<WFMarketTypes.ChatData>) => {
    queryClient.setQueryData(["get_chat_pagination", queryData], (oldData: any) => {
      if (!oldData) return;
      return {
        ...oldData,
        results: oldData.results.map((item: WFMarketTypes.ChatData) => {
          if (item.id !== chat.id) return item;
          if (chat.unread_count === -1) chat.unread_count = 0;
          if (chat.unread_count && chat.unread_count > 0) chat.unread_count += item.unread_count;
          item = { ...item, ...chat };
          return item;
        }),
      };
    });
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    updateChatData,
    refetchQueries,
  };
};

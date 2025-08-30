import { Box, Divider, Group, Pagination, ScrollArea, SimpleGrid } from "@mantine/core";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faCartShopping, faPen, faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../WarframeMarket.module.css";
import { TauriTypes, WFMarketTypes } from "$types";
import { useStockQueries } from "./queries";
import { useState } from "react";
import { useStockMutations } from "./mutations";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { Loading } from "@components/Shared/Loading";
import { useStockModals } from "./modals";
import { WFMAuction } from "../../../../components/DataDisplay/WFMAuction";
interface AuctionPanelProps {}
export const AuctionPanel = ({}: AuctionPanelProps) => {
  const [queryData, setQueryData] = useState<WFMarketTypes.WfmAuctionControllerGetListParams>({ page: 1, limit: 12 });
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [deletingOrders, setDeletingOrders] = useState<{ current: number; total: number }>({ current: 0, total: 0 });
  // Translate general
  const useTranslateTabOrder = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`warframe_market.tabs.orders.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`success.${key}`, { ...context }, i18Key);

  // Queries
  const { refetchQueries, paginationQuery } = useStockQueries({ queryData });

  // Mutations
  const { refreshAuctionsMutation, deleteAllAuctionsMutation, deleteStockMutation, createStockMutation, sellStockMutation } = useStockMutations({
    useTranslateSuccess,
    useTranslateErrors,
    refetchQueries,
    setLoadingRows,
  });

  // Modals
  const { OpenDeleteModal, HandleModalOrder } = useStockModals({
    createStockMutation,
    sellStockMutation,
    useTranslateBasePrompt,
    deleteStockMutation,
  });
  const handleRefresh = (_data: any) => {
    refetchQueries();
  };
  const handleDelete = (data: { current: number; total: number }) => {
    setDeletingOrders(data);
  };

  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshWfmAuctions, handleRefresh, [refetchQueries]);
  useTauriEvent(TauriTypes.Events.OnDeleteWfmAuctions, handleDelete, []);
  return (
    <Box>
      <SearchField
        value={queryData.query || ""}
        onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))}
        rightSectionWidth={63}
        rightSection={
          <Group gap={5}>
            <ActionWithTooltip
              tooltip={useTranslateButtons("refresh_tooltip")}
              icon={faRefresh}
              color={"green.7"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={(e) => {
                e.stopPropagation();
                refreshAuctionsMutation.mutateAsync(1);
              }}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("delete_all_tooltip")}
              icon={faTrashCan}
              color={"red.7"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={(e) => {
                e.stopPropagation();
                deleteAllAuctionsMutation.mutateAsync(1);
              }}
            />
          </Group>
        }
      />
      <ScrollArea mt={"md"} className={classes.orders} data-has-alert={useHasAlert()}>
        {deleteAllAuctionsMutation.isPending && <Loading text={`${deletingOrders.current} / ${deletingOrders.total}`} />}
        <SimpleGrid cols={4} spacing="sm">
          {paginationQuery.data?.results?.map((order) => (
            <WFMAuction
              display_style="grid"
              key={order.id}
              auction={order}
              footer={
                <>
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("sell_manual.sell_tooltip")}
                    icon={faPen}
                    loading={loadingRows.includes(`${order.id}`)}
                    color={"blue.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      // HandleModalOrder(order);
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("sell_auto.sell_tooltip")}
                    icon={faCartShopping}
                    loading={loadingRows.includes(`${order.id}`)}
                    color={"green.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("delete_tooltip")}
                    icon={faTrashCan}
                    loading={loadingRows.includes(`${order.id}`)}
                    color={"red.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      OpenDeleteModal(order.id);
                    }}
                  />
                </>
              }
            />
          ))}
        </SimpleGrid>
      </ScrollArea>
      <Divider mt={"md"} />
      <Group grow mt={"md"}>
        <Group justify="flex-end">
          <Pagination
            value={queryData.page}
            onChange={(page) => setQueryData((prev) => ({ ...prev, page }))}
            total={Math.ceil((paginationQuery.data?.total || 0) / queryData.limit)}
          />
        </Group>
      </Group>
    </Box>
  );
};

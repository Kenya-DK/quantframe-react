import { ActionIcon, Box, Divider, Group, Pagination, ScrollArea, Select, SimpleGrid } from "@mantine/core";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faArrowDown, faArrowUp, faCartShopping, faPen, faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../WarframeMarket.module.css";
import { WFMOrder } from "@components/DataDisplay/WFMOrder";
import { TauriTypes, WFMarketTypes } from "$types"; // Adjust the path if needed
import { ColorInfo } from "@components/Shared/ColorInfo";
import { useStockQueries } from "./queries";
import { useState } from "react";
import { useStockMutations } from "./mutations";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { Loading } from "@components/Shared/Loading";
import { useStockModals } from "./modals";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

interface OrderPanelProps {
  isActive?: boolean;
}
export const OrderPanel = ({ isActive }: OrderPanelProps) => {
  const [queryData, setQueryData] = useState<WFMarketTypes.WfmOrderControllerGetListParams>({
    page: 1,
    limit: 25,
    sort_by: "order_type",
    sort_direction: "desc",
  });
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [deletingOrders, setDeletingOrders] = useState<{ current: number; total: number }>({ current: 0, total: 0 });
  // Translate general
  const useTranslateTabOrder = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`warframe_market.tabs.orders.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateOrderType = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`order_type.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`success.${key}`, { ...context }, i18Key);

  // Queries
  const { refetchQueries, paginationQuery, statusCountsQuery } = useStockQueries({ queryData, isActive });

  // Mutations
  const { refreshOrdersMutation, deleteAllOrdersMutation, deleteStockMutation, createStockMutation, sellStockMutation } = useStockMutations({
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
    refetchQueries(true);
  };
  const handleDelete = (data: { current: number; total: number }) => {
    setDeletingOrders(data);
  };

  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshWfmOrders, handleRefresh, [refetchQueries]);
  useTauriEvent(TauriTypes.Events.OnDeleteWfmOrders, handleDelete, []);
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
                refreshOrdersMutation.mutateAsync(1);
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
                deleteAllOrdersMutation.mutateAsync(1);
              }}
            />
          </Group>
        }
      />

      <Group gap={"sm"} mt={"md"} justify="space-between">
        <Group>
          {Object.entries(statusCountsQuery.data || {})
            .sort(([a], [b]) => a.localeCompare(b))
            .map(([key, count]) => (
              <ColorInfo
                active={key == queryData.order_type}
                key={key}
                onClick={() =>
                  setQueryData((prev) => ({
                    ...prev,
                    order_type: (key as WFMarketTypes.OrderType) == prev.order_type ? undefined : (key as WFMarketTypes.OrderType),
                  }))
                }
                infoProps={{
                  "data-color-mode": "bg",
                  "data-order-type": key,
                }}
                text={useTranslateOrderType(`${key}`) + ` (${count[0]})` + ` (${count[1]})`}
                tooltip={useTranslateOrderType(`details.${key}`)}
              />
            ))}
        </Group>
        <Group gap="xs">
          <Select
            value={queryData.sort_by}
            allowDeselect={false}
            onChange={(value) => setQueryData((prev) => ({ ...prev, sort_by: value || "platinum" }))}
            data={["created_at", "platinum", "updated_at", "order_type"].map((key) => ({ label: useTranslateCommon(`sort_by.${key}`), value: key }))}
            size="xs"
          />
          <ActionIcon
            variant="light"
            color="blue"
            size="lg"
            onClick={() => {
              const direction = queryData.sort_direction === "asc" ? "desc" : "asc";
              setQueryData((prev) => ({ ...prev, sort_direction: direction }));
            }}
          >
            {queryData.sort_direction === "asc" ? <FontAwesomeIcon icon={faArrowUp} /> : <FontAwesomeIcon icon={faArrowDown} />}
          </ActionIcon>
        </Group>
      </Group>
      <ScrollArea mt={"md"} className={classes.orders} data-has-alert={useHasAlert()}>
        {deleteAllOrdersMutation.isPending && <Loading text={`${deletingOrders.current} / ${deletingOrders.total}`} />}
        <SimpleGrid cols={4} spacing="sm">
          {paginationQuery.data?.results?.map((order) => (
            <WFMOrder
              display_style="grid"
              key={order.id}
              order={order}
              footer={
                <>
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("sell_manual." + (order.type == WFMarketTypes.OrderType.Buy ? "buy_tooltip" : "sell_tooltip"))}
                    icon={faPen}
                    loading={loadingRows.includes(`${order.id}`)}
                    color={"blue.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      HandleModalOrder(order);
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("sell_auto." + (order.type == WFMarketTypes.OrderType.Buy ? "buy_tooltip" : "sell_tooltip"))}
                    icon={faCartShopping}
                    loading={loadingRows.includes(`${order.id}`)}
                    color={"green.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      switch (order.type) {
                        case WFMarketTypes.OrderType.Buy:
                          createStockMutation.mutateAsync(order);
                          break;
                        case WFMarketTypes.OrderType.Sell:
                          sellStockMutation.mutateAsync(order);
                          break;
                      }
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

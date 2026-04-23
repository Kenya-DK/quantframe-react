import { ActionIcon, Box, Divider, Group, ScrollArea, Select, SimpleGrid, Tooltip, useMantineTheme, Image, Rating, Badge } from "@mantine/core";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faArrowDown, faArrowUp, faCartShopping, faInfoCircle, faPen, faRefresh, faSackDollar, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../WarframeMarket.module.css";
import { TauriTypes, WFMarketTypes } from "$types"; // Adjust the path if needed
import { ColorInfo } from "@components/Shared/ColorInfo";
import { useStockQueries } from "./queries";
import { useState } from "react";
import { useStockMutations } from "./mutations";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { Loading } from "@components/Shared/Loading";
import { useStockModals } from "./modals";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { PaginationFooter } from "@components/Shared/PaginationFooter";
import { PreviewCard } from "@components/Shared/PreviewCard/PreviewCard";
import { WFMThumbnail } from "../../../../api";
import { faAmberStar, faCyanStar } from "../../../../icons";
import { upperFirst } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";

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
  const theme = useMantineTheme();
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
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`fields.${key}`, { ...context }, i18Key);
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
  const { OpenDeleteAllModal, OpenInfoModal, OpenDeleteModal, HandleModalOrder } = useStockModals({
    createStockMutation,
    sellStockMutation,
    useTranslateBasePrompt,
    deleteStockMutation,
    deleteAllOrdersMutation,
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
                OpenDeleteAllModal();
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
                text={
                  <TextTranslate
                    i18nKey={useTranslateTabOrder(`status`)}
                    size="lg"
                    values={{
                      type: useTranslateOrderType(`${key}`),
                      total: count[0],
                      platinum: count[1],
                      profit: count[2].toFixed(2),
                    }}
                    td={key == queryData.order_type ? "line-through" : ""}
                    components={{
                      profitIco: (
                        <Tooltip label={useTranslateTabOrder("tooltip_profit")}>
                          <FontAwesomeIcon icon={faSackDollar} />
                        </Tooltip>
                      ),
                    }}
                  />
                }
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
      <ScrollArea mt={"md"} className={classes.orders} data-has-alert={useHasAlert()} scrollbarSize={3}>
        {deleteAllOrdersMutation.isPending && <Loading text={`${deletingOrders.current} / ${deletingOrders.total}`} />}
        <SimpleGrid cols={4} spacing="sm">
          {paginationQuery.data?.results?.map((order, i) => (
            <PreviewCard
              key={i}
              value={order}
              headerLeft={{
                fz: "lg",
                fw: 700,
                color: "white",
                onClick: () => {
                  let name = order?.properties?.name || "Unknown Item";
                  navigator.clipboard.writeText(name);
                  notifications.show({
                    title: useTranslateCommon("notifications.copy_to_clipboard.title"),
                    message: useTranslateCommon("notifications.copy_to_clipboard.message", { message: name }),
                    color: "green.7",
                  });
                },
                i18nKey: useTranslateTabOrder("order_card.header_left"),
                values: { name: order?.properties?.name || "Unknown Item", mod_name: "" },
              }}
              headerRight={{
                fz: "lg",
                i18nKey: useTranslateTabOrder("order_card.header_right", undefined, true),
                values: {
                  quantity: order.quantity,
                },
              }}
              renderBody={() => (
                <Group grow>
                  <Group>
                    <Image
                      w={"50%"}
                      ml={"sm"}
                      height={64}
                      fit="contain"
                      src={order?.properties?.image ? WFMThumbnail(order.properties.image) : undefined}
                    />
                  </Group>
                  <Group justify="flex-end">
                    <Box>
                      {order.rank != undefined && (
                        <TextTranslate
                          size="lg"
                          i18nKey={useTranslateFields("mod_rank", undefined, true)}
                          values={{ mod_rank: order.rank, mod_max_rank: order.properties?.t_type?.max_rank || "?" }}
                        />
                      )}
                      {order.amberStars != undefined && (
                        <Rating
                          fullSymbol={<FontAwesomeIcon icon={faAmberStar} color={theme.colors.yellow[7]} />}
                          value={order.amberStars}
                          count={order.amberStars}
                          readOnly
                        />
                      )}
                      {order.cyanStars != undefined && (
                        <Rating
                          fullSymbol={<FontAwesomeIcon icon={faCyanStar} color={theme.colors.blue[7]} />}
                          value={order.cyanStars}
                          count={order.cyanStars}
                          readOnly
                        />
                      )}
                      {order.subtype && (
                        <TextTranslate
                          size="lg"
                          i18nKey={useTranslateFields("subtype", undefined, true)}
                          values={{ sub_type: order.subtype ? `${upperFirst(order.subtype)}` : "" }}
                        />
                      )}
                    </Box>
                  </Group>
                </Group>
              )}
              footerLeft={{
                fz: "lg",
                i18nKey: useTranslateTabOrder("order_card.footer_left"),
                values: {
                  price: order.platinum,
                },
              }}
              footerCenter={
                <Badge data-color-mode="bg" data-order-type={order.type}>
                  {useTranslateOrderType(order.type)}
                </Badge>
              }
              footerRight={
                <Group gap={3}>
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
                      let temp = { ...order };
                      temp.quantity = 1;
                      switch (temp.type) {
                        case WFMarketTypes.OrderType.Buy:
                          createStockMutation.mutateAsync(temp);
                          break;
                        case WFMarketTypes.OrderType.Sell:
                          sellStockMutation.mutateAsync(temp);
                          break;
                      }
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("info_tooltip")}
                    icon={faInfoCircle}
                    color={"blue.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      OpenInfoModal(order);
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
                </Group>
              }
            />
          ))}
        </SimpleGrid>
      </ScrollArea>
      <Divider mt={"md"} />
      <PaginationFooter
        page={queryData.page}
        limit={queryData.limit || 50}
        total={paginationQuery.data?.total || 0}
        onPageChange={(page) => setQueryData((prev) => ({ ...prev, page }))}
        onLimitChange={(limit) => setQueryData((prev) => ({ ...prev, page: 1, limit }))}
      />
    </Box>
  );
};

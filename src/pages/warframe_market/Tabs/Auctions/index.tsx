import { ActionIcon, Box, Divider, Group, Pagination, ScrollArea, Select, SimpleGrid, Tooltip } from "@mantine/core";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faArrowDown, faArrowUp, faFileImport, faRefresh, faSackDollar, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../WarframeMarket.module.css";
import { TauriTypes, WFMarketTypes } from "$types";
import { useStockQueries } from "./queries";
import { useState } from "react";
import { useStockMutations } from "./mutations";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { Loading } from "@components/Shared/Loading";
import { useStockModals } from "./modals";
import { WFMAuction } from "@components/DataDisplay/WFMAuction";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { TextTranslate } from "../../../../components/Shared/TextTranslate";
interface AuctionPanelProps {
  isActive?: boolean;
}
export const AuctionPanel = ({ isActive }: AuctionPanelProps) => {
  const [queryData, setQueryData] = useState<WFMarketTypes.WfmAuctionControllerGetListParams>({
    page: 1,
    limit: 12,
    sort_by: "created_at",
    sort_direction: "desc",
  });
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [deletingOrders, setDeletingOrders] = useState<{ current: number; total: number }>({ current: 0, total: 0 });
  // Translate general
  const useTranslateTabOrder = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`warframe_market.tabs.auctions.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`success.${key}`, { ...context }, i18Key);

  // Queries
  const { refetchQueries, paginationQuery, overviewQuery } = useStockQueries({ queryData, isActive });

  // Mutations
  const { refreshAuctionsMutation, importStockMutation, deleteAllAuctionsMutation, deleteStockMutation } = useStockMutations({
    useTranslateSuccess,
    useTranslateErrors,
    refetchQueries,
    setLoadingRows,
  });

  // Modals
  const { OpenDeleteModal, OpenImportModal } = useStockModals({
    useTranslateBasePrompt,
    importStockMutation,
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
      <Group gap={"sm"} mt={"md"} justify="space-between">
        <TextTranslate
          size="lg"
          i18nKey={useTranslateTabOrder("overview", undefined, true)}
          values={{
            total: overviewQuery.data?.[0] || 0,
            revenue: overviewQuery.data?.[1] || 0,
            profit: overviewQuery.data?.[2].toFixed(2) || 0,
          }}
          components={{
            profitIco: (
              <Tooltip label={useTranslateTabOrder("tooltip_profit")}>
                <FontAwesomeIcon icon={faSackDollar} />
              </Tooltip>
            ),
          }}
        />
        <Group>
          <Select
            value={queryData.sort_by}
            allowDeselect={false}
            onChange={(value) => setQueryData((prev) => ({ ...prev, sort_by: value || "platinum" }))}
            data={["created_at", "platinum", "updated_at"].map((key) => ({ label: useTranslateCommon(`sort_by.${key}`), value: key }))}
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
        {deleteAllAuctionsMutation.isPending && <Loading text={`${deletingOrders.current} / ${deletingOrders.total}`} />}
        <SimpleGrid cols={2} spacing="sm">
          {paginationQuery.data?.results?.map((order) => (
            <WFMAuction
              display_style="grid"
              show_user
              key={order.id}
              auction={order}
              header={
                <Group gap={"xs"}>
                  {order.properties?.can_import && (
                    <ActionWithTooltip
                      tooltip={useTranslateButtons("import_tooltip")}
                      icon={faFileImport}
                      loading={loadingRows.includes(`${order.id}`)}
                      color={"blue.7"}
                      actionProps={{ size: "sm" }}
                      iconProps={{ size: "xs" }}
                      onClick={(e) => {
                        e.stopPropagation();
                        OpenImportModal(order.id);
                      }}
                    />
                  )}
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

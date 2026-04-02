import { ActionIcon, Box, Divider, Group, ScrollArea, Select, SimpleGrid, Tooltip } from "@mantine/core";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faArrowDown, faArrowUp, faFileImport, faInfoCircle, faRefresh, faSackDollar, faTrashCan } from "@fortawesome/free-solid-svg-icons";
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
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { PaginationFooter } from "@components/Shared/PaginationFooter";
import { PreviewCard } from "@components/Shared/PreviewCard/PreviewCard";
import { RivenAttribute } from "@components/DataDisplay/RivenAttribute";
import { upperFirst } from "@mantine/hooks";
interface AuctionPanelProps {
  isActive?: boolean;
}
export const AuctionPanel = ({ isActive }: AuctionPanelProps) => {
  const [queryData, setQueryData] = useState<WFMarketTypes.WfmAuctionControllerGetListParams>({
    page: 1,
    limit: 12,
    sort_by: "created_at",
    sort_direction: "desc",
    auction_type: "riven",
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
  const { OpenDeleteModal, OpenImportModal, OpenInfoModal } = useStockModals({
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
        <SimpleGrid cols={{ base: 2, xl: 3 }} spacing="sm">
          {paginationQuery.data?.results?.map((auction, i) => (
            <PreviewCard
              key={i}
              value={auction}
              headerRight={{
                fz: "lg",
                i18nKey: useTranslateTabOrder("auction_card.header_right", undefined, true),
                values: { price: auction.starting_price || 0 },
              }}
              headerCenter={{
                fz: "lg",
                i18nKey: "components.riven_preview.riven_name",
                values: { name: auction.properties.name, mod_name: upperFirst(auction.item.name) },
              }}
              renderBody={() =>
                auction.item.attributes.map((attr) => (
                  <RivenAttribute key={attr.url_name} i18nKey="full" groupProps={{ p: 1 }} value={attr} hideDetails centered hideGrade />
                ))
              }
              footerCenter={
                <Group gap={3}>
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("delete_tooltip")}
                    icon={faTrashCan}
                    loading={loadingRows.includes(`${auction.id}`)}
                    color={"red.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      OpenDeleteModal(auction.id);
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("import_tooltip")}
                    icon={faFileImport}
                    loading={loadingRows.includes(`${auction.id}`)}
                    color={"blue.7"}
                    actionProps={{ size: "sm", display: auction.properties?.can_import ? "inline-flex" : "none" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      OpenImportModal(auction.id);
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateButtons("info_tooltip")}
                    icon={faInfoCircle}
                    loading={loadingRows.includes(`${auction.id}`)}
                    color={"blue.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      OpenInfoModal(auction);
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

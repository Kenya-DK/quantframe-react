import { Text, Box, Divider, Group, Pagination, ScrollArea, SimpleGrid } from "@mantine/core";
import { useEffect, useState } from "react";
import { Wfm } from "$types/index";
import { faFileImport, faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { paginate } from "@utils/helper";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import api from "@api/index";
import { notifications } from "@mantine/notifications";
import { useMutation } from "@tanstack/react-query";
import { modals } from "@mantine/modals";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { AuctionListItem } from "@components/AuctionListItem";
import { Loading } from "@components/Loading";
import { SearchField } from "@components/SearchField";
import { useStockContextContext } from "@contexts/stock.context";
import { useWarframeMarketContextContext } from "@contexts/warframeMarket.context";
import { searchProperties, Query } from "@utils/search.helper";
import classes from "../../WarframeMarket.module.css";
import { useHasAlert } from "@hooks/useHasAlert.hook";
interface AuctionPanelProps {}
export const AuctionPanel = ({}: AuctionPanelProps) => {
  // State's
  const [query, setQuery] = useState<string>("");
  const [page, setPage] = useState(1);
  const pageSizes = [1, 5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, _setPageSize] = useState(pageSizes[4]);
  const [totalPages, setTotalPages] = useState(0);
  const [rows, setRows] = useState<Wfm.Auction<string>[]>([]);
  const { auctions } = useWarframeMarketContextContext();
  const { rivens } = useStockContextContext();

  // Update Database Rows
  useEffect(() => {
    let filteredRecords = auctions;
    let filter: Query = {};
    if (!rivens) return;

    if (query)
      filter = {
        $or: [
          { $combined: ["item.weapon_url_name", "item.name"], value: query },
          { "item.name": { $contains: query } },
          { "item.weapon_url_name": { $contains: query } },
        ],
      };

    filteredRecords = searchProperties(filteredRecords, filter, false);
    // Update total pages
    setTotalPages(Math.ceil(filteredRecords.length / pageSize));

    setRows(paginate(filteredRecords, page, pageSize));
  }, [auctions, query, pageSize, page]);

  // Translate general
  const useTranslateTabOrder = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`warframe_market.tabs.auctions.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`buttons.${key}`, { ...context }, i18Key);
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`success.${key}`, { ...context }, i18Key);

  // Mutations
  const refreshAuctionsMutation = useMutation({
    mutationFn: () => api.auction.refresh(),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("refresh.title"),
        message: useTranslateSuccess("refresh.message", { count: u }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("refresh.title"), message: useTranslateErrors("refresh.message"), color: "red.7" });
    },
  });
  const deleteAuctionsMutation = useMutation({
    mutationFn: (id: string) => api.auction.delete(id),
    onSuccess: async (name) => {
      notifications.show({ title: useTranslateSuccess("delete.title"), message: useTranslateSuccess("delete.message", { name }), color: "green.7" });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("delete.title"), message: useTranslateErrors("delete.message"), color: "red.7" });
    },
  });
  const deleteAllAuctionsMutation = useMutation({
    mutationFn: () => api.auction.deleteAll(),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("delete_all.title"),
        message: useTranslateSuccess("delete_all.message", { count: u }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("delete_all.title"), message: useTranslateErrors("delete_all.message"), color: "red.7" });
    },
  });
  const createRivenFromAuctionsMutation = useMutation({
    mutationFn: (a: Wfm.Auction<string> & { price: number }) => api.auction.import_auction(a, a.price),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("import_riven.title"),
        message: useTranslateSuccess("import_riven.message", { count: u }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("import_riven.title"), message: useTranslateErrors("import_riven.message"), color: "red.7" });
    },
  });
  const OpenCreateStockRiven = (auction: Wfm.Auction<string>) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslatePrompt(`import_riven.title`),
      innerProps: {
        fields: [
          {
            name: "price",
            label: useTranslatePrompt(`import_riven.bought.label`),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { price: number }) => {
          if (!auction) return;
          createRivenFromAuctionsMutation.mutateAsync({ ...auction, price: data.price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  return (
    <Box>
      <SearchField
        value={query}
        onChange={(text) => setQuery(text)}
        rightSectionWidth={63}
        rightSection={
          <Group gap={5}>
            <ActionWithTooltip
              tooltip={useTranslateButtons("refresh.tooltip")}
              icon={faRefresh}
              loading={refreshAuctionsMutation.isPending || deleteAuctionsMutation.isPending || deleteAllAuctionsMutation.isPending}
              color={"green.7"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={(e) => {
                e.stopPropagation();
                refreshAuctionsMutation.mutateAsync();
              }}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("delete_all.tooltip")}
              icon={faTrashCan}
              loading={refreshAuctionsMutation.isPending || deleteAuctionsMutation.isPending || deleteAllAuctionsMutation.isPending}
              color={"red.7"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={(e) => {
                e.stopPropagation();
                modals.openConfirmModal({
                  title: useTranslatePrompt("delete_all.title"),
                  children: <Text size="sm">{useTranslatePrompt("delete_all.message")}</Text>,
                  labels: { confirm: useTranslatePrompt("delete_all.confirm"), cancel: useTranslatePrompt("delete.cancel") },
                  onConfirm: async () => await deleteAllAuctionsMutation.mutateAsync(),
                });
              }}
            />
          </Group>
        }
      />
      <ScrollArea mt={"md"} className={classes.auctions} data-has-alert={useHasAlert()}>
        {refreshAuctionsMutation.isPending || deleteAuctionsMutation.isPending || deleteAllAuctionsMutation.isPending ? <Loading /> : null}
        <SimpleGrid cols={{ base: 1, sm: 2, lg: 2 }} spacing="lg">
          {rows.map((order, i) => (
            <AuctionListItem
              key={i}
              // compacted
              show_image
              auction={order}
              header={
                <Group gap={5}>
                  {!rivens.find((r) => r.wfm_order_id == order.id) && order.is_direct_sell ? (
                    <ActionWithTooltip
                      tooltip={useTranslateButtons("import.tooltip")}
                      icon={faFileImport}
                      color={"blue.7"}
                      actionProps={{ size: "sm" }}
                      iconProps={{ size: "xs" }}
                      onClick={(e) => {
                        e.stopPropagation();
                        OpenCreateStockRiven(order);
                      }}
                    />
                  ) : null}

                  <ActionWithTooltip
                    tooltip={useTranslateButtons("delete.tooltip")}
                    icon={faTrashCan}
                    color={"red.7"}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={(e) => {
                      e.stopPropagation();
                      modals.openConfirmModal({
                        title: useTranslatePrompt("delete.title"),
                        children: <Text size="sm">{useTranslatePrompt("delete.message")}</Text>,
                        labels: { confirm: useTranslatePrompt("delete.confirm"), cancel: useTranslatePrompt("delete.cancel") },
                        onConfirm: async () => await deleteAuctionsMutation.mutateAsync(order.id),
                      });
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
        <Group></Group>
        <Group justify="flex-end">
          <Pagination value={page} onChange={setPage} total={totalPages} />
        </Group>
      </Group>
    </Box>
  );
};

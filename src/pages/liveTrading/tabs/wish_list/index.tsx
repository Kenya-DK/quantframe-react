import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useEffect, useState } from "react";
import { Box, Grid, Group, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { faEdit, faEye, faEyeSlash, faHammer, faInfo, faPen, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { ColorInfo } from "@components/ColorInfo";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { CreateStockItemForm } from "@components/Forms/CreateStockItem";
import { useMutation, useQuery } from "@tanstack/react-query";
import api, { OnTauriEvent } from "@api/index";
import { TauriTypes } from "$types";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { TextTranslate } from "@components/TextTranslate";
import { getCssVariable, GetSubTypeDisplay } from "@utils/helper";
import { modals } from "@mantine/modals";
import { StatsWithSegments } from "@components/StatsWithSegments";
import { WishItemInfo } from "@components/Modals/WishItemInfo";
import classes from "../../LiveTrading.module.css";
import { useLocalStorage } from "@mantine/hooks";
import { SearchField } from "../../../../components/SearchField";
import { DataTable } from "mantine-datatable";
import { ButtonIntervals } from "../../../../components/ButtonIntervals";
interface WishListPanelProps {}
export const WishListPanel = ({}: WishListPanelProps) => {
  // States Context
  const { is_running } = useLiveScraperContext();

  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.WishListControllerGetListParams>({
    key: "wish_list_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10 },
  });
  const [loadingRows, setLoadingRows] = useState<string[]>([]);

  // States
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.WishListItem[]>([]);
  const [statusCount, setStatusCount] = useState<{ [key: string]: number }>({});
  const [segments, setSegments] = useState<{ label: string; count: number; part: number; color: string }[]>([]);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`liveTrading.${key}`, { ...context }, i18Key);
  const useTranslateSegments = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`segments.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.wish_list.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateDataGridBaseColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`success.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`prompts.${key}`, { ...context }, i18Key);
  // const useTranslateNotifications = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslate(`notifications.${key}`, { ...context }, i18Key);

  // Queys
  let { data, isFetching, refetch } = useQuery({
    queryKey: ["wish_list", queryData.page, queryData.limit, queryData.sort_by, queryData.sort_direction, queryData.status],
    queryFn: () => api.stock.wishList.getAll(queryData),
    refetchOnWindowFocus: true,
  });

  // Member
  useEffect(() => {
    const wish_lists = data?.results || [];
    if (!wish_lists) return;
    const totalListedPrice = wish_lists.reduce((a, b) => a + (b.list_price || 0) * b.quantity, 0);
    const totalTrades = wish_lists.filter((item) => item.status === TauriTypes.StockStatus.Live).reduce((a, b) => a + b.quantity, 0) / 6;
    // Round up to the nearest whole number
    let totalTradesRounded = Math.ceil(totalTrades);
    setSegments([
      { label: useTranslateSegments("total_plat"), count: totalListedPrice, part: 0, color: getCssVariable("--positive-value") },
      { label: useTranslateSegments("trades"), count: totalTradesRounded, part: 0, color: getCssVariable("--profit-value") },
    ]);
    setStatusCount(
      Object.values(TauriTypes.StockStatus).reduce((acc, status) => {
        acc[status] = wish_lists.filter((item) => item.status === status).length;
        return acc;
      }, {} as { [key: string]: number })
    );
  }, [data]);
  // Calculate Stats
  useEffect(() => {}, [data]);

  // Mutations
  const createItemMutation = useMutation({
    mutationFn: (data: TauriTypes.CreateWishListItem) => api.stock.wishList.create(data),
    onSuccess: async (u) => {
      refetch();
      notifications.show({
        title: useTranslateSuccess("create_item.title"),
        message: useTranslateSuccess("create_item.message", { name: u.item_name }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("create_item.title"), message: useTranslateErrors("create_item.message"), color: "red.7" });
    },
  });
  const updateItemMutation = useMutation({
    mutationFn: (data: TauriTypes.UpdateWishListItem) => api.stock.wishList.update(data),
    onMutate: (row) => setLoadingRows((prev) => [...prev, `${row.id}`]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => id !== `${variables.id}`)),
    onSuccess: async (u) => {
      refetch();
      notifications.show({
        title: useTranslateSuccess("update_item.title"),
        message: useTranslateSuccess("update_item.message", { name: u.item_name }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("update_item.title"), message: useTranslateErrors("update_item.message"), color: "red.7" });
    },
  });
  const deleteItemMutation = useMutation({
    mutationFn: (id: number) => api.stock.wishList.delete(id),
    onMutate: (row) => setLoadingRows((prev) => [...prev, `${row}`]),
    onSettled: (_data, _error, row) => setLoadingRows((prev) => prev.filter((id) => id !== `${row}`)),
    onSuccess: async () => {
      refetch();
      notifications.show({
        title: useTranslateSuccess("delete_item.title"),
        message: useTranslateSuccess("delete_item.message"),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("delete_item.title"), message: useTranslateErrors("delete_item.message"), color: "red.7" });
    },
  });
  const boughtItemMutation = useMutation({
    mutationFn: (data: TauriTypes.BoughtWishListItem) => api.stock.wishList.bought(data),
    onMutate: (row) => setLoadingRows((prev) => [...prev, `${row.id}`]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => id !== `${variables.id}`)),
    onSuccess: async (u) => {
      refetch();
      notifications.show({
        title: useTranslateSuccess("sell_stock.title"),
        message: useTranslateSuccess("sell_stock.message", { name: u.item_name }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("sell_stock.title"), message: useTranslateErrors("sell_stock.message"), color: "red.7" });
    },
  });
  // Modal's
  const OpenMinimumPriceModal = (id: number, maximum_price: number) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("maximum_price.title"),
      innerProps: {
        fields: [
          {
            name: "maximum_price",
            label: useTranslateBasePrompt("maximum_price.fields.maximum_price.label"),
            attributes: {
              min: 0,
              description: useTranslateBasePrompt("maximum_price.fields.maximum_price.description"),
            },
            value: maximum_price,
            type: "number",
          },
        ],
        onConfirm: async (data: { maximum_price: number }) => {
          if (!id) return;
          const { maximum_price } = data;
          await updateItemMutation.mutateAsync({ id, maximum_price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  const OpenInfoModal = (item: TauriTypes.WishListItem) => {
    modals.open({
      size: "100%",
      title: item.item_name,
      children: <WishItemInfo value={item} />,
    });
  };
  const OpenBoughtModal = (stock: TauriTypes.WishListItem) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("bought.title"),
      innerProps: {
        fields: [
          {
            name: "bought",
            label: useTranslateBasePrompt("bought.fields.bought.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { bought: number }) => {
          if (!stock) return;
          const { bought } = data;
          await boughtItemMutation.mutateAsync({ id: stock.id, price: bought });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  useEffect(() => {
    OnTauriEvent<any>(TauriTypes.Events.RefreshWishListItems, () => refetch());
    return () => api.events.CleanEvent(TauriTypes.Events.RefreshWishListItems);
  }, []);
  return (
    <Box>
      <Grid>
        <Grid.Col span={8}>
          <CreateStockItemForm
            hide_bought
            disabled={createItemMutation.isPending}
            onSubmit={async (item) => {
              createItemMutation.mutate({ ...item });
            }}
          />
          <Group gap={"md"} mt={"md"}>
            {[TauriTypes.StockStatus.Pending, TauriTypes.StockStatus.Live, TauriTypes.StockStatus.NoSellers, TauriTypes.StockStatus.InActive].map(
              (status) => (
                <ColorInfo
                  active={status == queryData.status}
                  key={status}
                  onClick={() => setQueryData((prev) => ({ ...prev, status: status == prev.status ? undefined : status }))}
                  infoProps={{
                    "data-color-mode": "bg",
                    "data-stock-status": status,
                  }}
                  text={useTranslateStockStatus(`${status}`) + `${statusCount[status] == 0 ? "" : ` (${statusCount[status]})`}`}
                  tooltip={useTranslateStockStatus(`details.${status}`)}
                />
              )
            )}
          </Group>
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithSegments showPercent segments={segments} />
        </Grid.Col>
      </Grid>
      <SearchField value={queryData.query || ""} onSearch={() => refetch()} onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))} />
      <DataTable
        className={`${classes.databaseStockWishlist} ${useHasAlert() ? classes.alert : ""} ${is_running ? classes.running : ""}`}
        customRowAttributes={(record) => {
          return {
            "data-color-mode": "box-shadow",
            "data-stock-status": record.status,
          };
        }}
        mt={"md"}
        striped
        fetching={isFetching}
        records={data?.results || []}
        page={queryData.page || 1}
        onPageChange={(page) => setQueryData((prev) => ({ ...prev, page }))}
        totalRecords={data?.total}
        recordsPerPage={queryData.limit || 10}
        recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
        onRecordsPerPageChange={(limit) => setQueryData((prev) => ({ ...prev, limit }))}
        selectedRecords={selectedRecords}
        onSelectedRecordsChange={setSelectedRecords}
        sortStatus={{
          columnAccessor: queryData.sort_by || "name",
          direction: queryData.sort_direction || "desc",
        }}
        onSortStatusChange={(sort) => {
          if (!sort || !sort.columnAccessor) return;
          setQueryData((prev) => ({ ...prev, sort_by: sort.columnAccessor as string, sort_direction: sort.direction }));
        }}
        // define columns
        columns={[
          {
            accessor: "item_name",
            title: useTranslateDataGridBaseColumns("name.title"),
            sortable: true,
            render: ({ item_name, sub_type }) => (
              <TextTranslate
                color="gray.4"
                i18nKey={useTranslateDataGridBaseColumns("name.value", undefined, true)}
                values={{
                  name: item_name,
                  sub_type: GetSubTypeDisplay(sub_type),
                }}
              />
            ),
          },
          {
            accessor: "quantity",
            title: useTranslateDataGridColumns("quantity"),
          },
          {
            accessor: "maximum_price",
            width: 310,
            title: useTranslateDataGridColumns("maximum_price.title"),
            render: ({ id, maximum_price }) => (
              <Group gap={"sm"} justify="space-between">
                <Text>{maximum_price || "N/A"}</Text>
                <Group gap={"xs"}>
                  <ButtonIntervals
                    intervals={[5, 10]}
                    minimum_price={maximum_price || 0}
                    OnClick={async (val) => {
                      if (!id) return;
                      console.log("Update minimum price to:", val);
                      await updateItemMutation.mutateAsync({ id, maximum_price: val });
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateDataGridBaseColumns("minimum_price.btn.edit.tooltip")}
                    icon={faEdit}
                    onClick={(e) => {
                      e.stopPropagation();
                      if (!id) return;
                      OpenMinimumPriceModal(id, maximum_price || 0);
                    }}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                  />
                </Group>
              </Group>
            ),
          },
          {
            accessor: "list_price",
            title: useTranslateDataGridBaseColumns("list_price"),
          },
          {
            accessor: "actions",
            title: useTranslateDataGridBaseColumns("actions.title"),
            width: 185,
            render: (row) => (
              <Group gap={"sm"} justify="flex-end">
                <ActionWithTooltip
                  tooltip={useTranslateDataGridColumns("actions.buttons.bought_manual.tooltip")}
                  icon={faPen}
                  loading={loadingRows.includes(`${row.id}`)}
                  color={"green.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    OpenBoughtModal(row);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridColumns("actions.buttons.bought_auto.tooltip")}
                  icon={faHammer}
                  actionProps={{ disabled: !row.list_price, size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    if (!row.id || !row.list_price) return;
                    await boughtItemMutation.mutateAsync({
                      id: row.id,
                      price: row.list_price,
                    });
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridBaseColumns("actions.buttons.info.tooltip")}
                  icon={faInfo}
                  loading={loadingRows.includes(`${row.id}`)}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    OpenInfoModal(row);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridBaseColumns(`actions.buttons.hide.${row.is_hidden ? "disabled_tooltip" : "enabled_tooltip"}`)}
                  icon={row.is_hidden ? faEyeSlash : faEye}
                  loading={loadingRows.includes(`${row.id}`)}
                  color={`${row.is_hidden ? "red.7" : "green.7"}`}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    await updateItemMutation.mutateAsync({ id: row.id, is_hidden: !row.is_hidden });
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridBaseColumns("actions.buttons.delete.tooltip")}
                  color={"red.7"}
                  icon={faTrashCan}
                  loading={loadingRows.includes(`${row.id}`)}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    modals.openConfirmModal({
                      title: useTranslateBasePrompt("delete.title"),
                      children: <Text size="sm">{useTranslateBasePrompt("delete.message", { count: 1 })}</Text>,
                      labels: { confirm: useTranslateBasePrompt("delete.confirm"), cancel: useTranslateBasePrompt("delete.cancel") },
                      onConfirm: async () => await deleteItemMutation.mutateAsync(row.id),
                    });
                  }}
                />
              </Group>
            ),
          },
        ]}
      />
    </Box>
  );
};

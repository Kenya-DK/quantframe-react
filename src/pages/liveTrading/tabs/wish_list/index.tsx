import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useEffect, useState } from "react";
import { CreateWishListItem, WishListItem, StockStatus, UpdateWishListItem, BoughtWishListItem } from "@api/types";
import { Box, Grid, Group, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { faEdit, faHammer, faInfo, faPen, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { ColorInfo } from "@components/ColorInfo";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { Loading } from "@components/Loading";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useStockContextContext } from "@contexts/stock.context";
import { DataTableSearch } from "@components/DataTableSearch";
import { Query } from "@utils/search.helper";
import { CreateStockItemForm } from "@components/Forms/CreateStockItem";
import { useMutation } from "@tanstack/react-query";
import api from "@api/index";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { TextTranslate } from "@components/TextTranslate";
import { getCssVariable, GetSubTypeDisplay } from "@utils/helper";
import { ButtonInterval } from "@components/ButtonInterval";
import { modals } from "@mantine/modals";
import { StatsWithSegments } from "@components/StatsWithSegments";
import { WishItemInfo } from "@components/Modals/WishItemInfo";
import classes from "../../LiveTrading.module.css";
interface WishListPanelProps {}
export const WishListPanel = ({}: WishListPanelProps) => {
  // States Context
  const { wish_lists } = useStockContextContext();
  const { is_running } = useLiveScraperContext();

  // States For DataGrid
  const [query, setQuery] = useState<string>("");
  const [filters, setFilters] = useState<Query>({});
  const [selectedRecords, setSelectedRecords] = useState<WishListItem[]>([]);

  const [filterStatus, setFilterStatus] = useState<StockStatus | undefined>(undefined);
  const [statusCount, setStatusCount] = useState<{ [key: string]: number }>({}); // Count of each status

  const [segments, setSegments] = useState<{ label: string; count: number; part: number; color: string }[]>([]);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`liveTrading.${key}`, { ...context }, i18Key);
  const useTranslateTabWishList = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.wish_list.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabWishList(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabWishList(`success.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabWishList(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateDataGridBaseColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateNotifications = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`notifications.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateSegments = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`segments.${key}`, { ...context }, i18Key);

  // Update Database Rows
  useEffect(() => {
    let filter: Query = {
      $or: [],
    };
    if (!wish_lists) return;

    setStatusCount(
      Object.values(StockStatus).reduce((acc, status) => {
        acc[status] = wish_lists.filter((item) => item.status === status).length;
        return acc;
      }, {} as { [key: string]: number })
    );

    if (filterStatus) filter = { $match: { status: filterStatus } };
    if (query) filter = { ...filter, $or: [{ item_name: { $contains: query } }] };

    setFilters(filter);
    setSelectedRecords([]);
  }, [wish_lists, query, filterStatus]);
  // Calculate Stats
  useEffect(() => {
    if (!wish_lists) return;
    const totalListedPrice = wish_lists.reduce((a, b) => a + (b.list_price || 0) * b.quantity, 0);
    const totalTrades = wish_lists.filter((item) => item.status === StockStatus.Live).reduce((a, b) => a + b.quantity, 0) / 6;
    // Round up to the nearest whole number
    let totalTradesRounded = Math.ceil(totalTrades);
    setSegments([
      { label: useTranslateSegments("total_plat"), count: totalListedPrice, part: 0, color: getCssVariable("--positive-value") },
      { label: useTranslateSegments("trades"), count: totalTradesRounded, part: 0, color: getCssVariable("--profit-value") },
    ]);
  }, [wish_lists]);
  // Mutations
  const createItemMutation = useMutation({
    mutationFn: (data: CreateWishListItem) => api.stock.wishList.create(data),
    onSuccess: async (u) => {
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
    mutationFn: (data: UpdateWishListItem) => api.stock.wishList.update(data),
    onSuccess: async (u) => {
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
    onSuccess: async () => {
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
    mutationFn: (data: BoughtWishListItem) => api.stock.wishList.bought(data),
    onSuccess: async (u) => {
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
  const OpenInfoModal = (item: WishListItem) => {
    modals.open({
      size: "100%",
      title: item.item_name,
      children: <WishItemInfo value={item} />,
    });
  };
  const OpenBoughtModal = (stock: WishListItem) => {
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
            {[StockStatus.Pending, StockStatus.Live, StockStatus.NoSellers].map((status) => (
              <ColorInfo
                active={status == filterStatus}
                key={status}
                onClick={() => {
                  setFilterStatus((s) => (s === status ? undefined : status));
                }}
                infoProps={{
                  "data-color-mode": "bg",
                  "data-stock-status": status,
                }}
                text={useTranslateStockStatus(`${status}`) + `${statusCount[status] == 0 ? "" : ` (${statusCount[status]})`}`}
                tooltip={useTranslateStockStatus(`details.${status}`)}
              />
            ))}
          </Group>
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithSegments segments={segments} />
        </Grid.Col>
      </Grid>
      <DataTableSearch
        className={`${classes.databaseStockWishlist} ${useHasAlert() ? classes.alert : ""} ${is_running ? classes.running : ""}`}
        mt={"md"}
        records={wish_lists || []}
        customRowAttributes={(record) => {
          return {
            "data-color-mode": "box-shadow",
            "data-stock-status": record.status,
          };
        }}
        query={query}
        filters={filters}
        onSearchChange={(text) => setQuery(text)}
        customLoader={<Loading />}
        idAccessor={"id"}
        selectedRecords={selectedRecords}
        onSelectedRecordsChange={setSelectedRecords}
        onCellClick={({ record, column }) => {
          switch (column.accessor) {
            case "item_name":
              navigator.clipboard.writeText(record.item_name);
              notifications.show({ title: useTranslateNotifications("copied.title"), message: record.item_name, color: "green.7" });
              break;
          }
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
                  <ButtonInterval
                    color="red.7"
                    intervals={[5, 10]}
                    prefix="-"
                    OnClick={async (int) => {
                      if (!id) return;
                      maximum_price = maximum_price || 0;
                      if (maximum_price - int < 0) return;
                      await updateItemMutation.mutateAsync({ id, maximum_price: maximum_price - int });
                    }}
                  />
                  <ButtonInterval
                    color="green.7"
                    intervals={[5, 10]}
                    prefix="+"
                    OnClick={async (int) => {
                      if (!id) return;
                      maximum_price = maximum_price || 0;
                      await updateItemMutation.mutateAsync({ id, maximum_price: maximum_price + int });
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
            width: 145,
            render: (row) => (
              <Group gap={"sm"} justify="flex-end">
                <ActionWithTooltip
                  tooltip={useTranslateDataGridColumns("actions.buttons.bought_manual.tooltip")}
                  icon={faPen}
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
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    OpenInfoModal(row);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridBaseColumns("actions.buttons.delete.tooltip")}
                  color={"red.7"}
                  icon={faTrashCan}
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

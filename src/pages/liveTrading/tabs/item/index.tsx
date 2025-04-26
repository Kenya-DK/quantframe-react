import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { getCssVariable, GetSubTypeDisplay, CreateTradeMessage } from "@utils/helper";
import { useEffect, useState } from "react";
import { TauriTypes } from "$types";
import { Box, Grid, Group, NumberFormatter, Text } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import api from "@api/index";
import { notifications } from "@mantine/notifications";
import { faComment, faEdit, faEye, faEyeSlash, faHammer, faInfo, faPen, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { modals } from "@mantine/modals";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { StockItemInfo } from "@components/Modals/StockItemInfo";
import { ColorInfo } from "@components/ColorInfo";
import { StatsWithSegments } from "@components/StatsWithSegments";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { ButtonInterval } from "@components/ButtonInterval";
import { Loading } from "@components/Loading";
import { TextTranslate } from "@components/TextTranslate";
import { UpdateItemBulk } from "@components/Forms/UpdateItemBulk";
import { CreateStockItemForm } from "@components/Forms/CreateStockItem";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useStockContextContext } from "@contexts/stock.context";
import { DataTableSearch } from "@components/DataTableSearch";
import { ComplexFilter, Operator } from "@utils/filter.helper";
import classes from "../../LiveTrading.module.css";
interface StockItemPanelProps {}
export const StockItemPanel = ({}: StockItemPanelProps) => {
  // States Context
  const { items } = useStockContextContext();
  const { is_running } = useLiveScraperContext();

  // States For DataGrid
  const [query, setQuery] = useState<string>("");
  const [filters, setFilters] = useState<ComplexFilter>({});
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.StockItem[]>([]);

  const [filterStatus, setFilterStatus] = useState<TauriTypes.StockStatus | undefined>(undefined);
  const [statusCount, setStatusCount] = useState<{ [key: string]: number }>({}); // Count of each status

  const [segments, setSegments] = useState<{ label: string; count: number; part: number; color: string }[]>([]);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`liveTrading.${key}`, { ...context }, i18Key);
  const useTranslateSegments = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`segments.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.item.${key}`, { ...context }, i18Key);
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
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateNotifications = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`notifications.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`buttons.${key}`, { ...context }, i18Key);

  // Update Database Rows
  useEffect(() => {
    let filter: ComplexFilter = {
      OR: [],
    };
    if (!items) return;

    setStatusCount(
      Object.values(TauriTypes.StockStatus).reduce((acc, status) => {
        acc[status] = items.filter((item) => item.status === status).length;
        return acc;
      }, {} as { [key: string]: number })
    );

    if (filterStatus)
      filter.OR?.push({
        status: {
          [Operator.EQUALS]: filterStatus,
        },
      });
    if (query != "")
      filter.OR?.push({
        item_name: {
          [Operator.CONTAINS_VALUE]: query,
          isCaseSensitive: false,
        },
      });

    setFilters(filter);
    setSelectedRecords([]);
  }, [items, query, filterStatus]);

  // Calculate Stats
  useEffect(() => {
    if (!items) return;
    const totalPurchasePrice = items.reduce((a, b) => a + (b.bought || 0) * b.owned, 0);
    const totalListedPrice = items.reduce((a, b) => a + (b.list_price || 0) * b.owned, 0);
    const totalProfit = totalListedPrice > 0 ? totalListedPrice - totalPurchasePrice : 0;

    // Calculate the total count
    const totalCount = totalPurchasePrice + totalListedPrice + totalProfit;

    // Calculate the percentage of each count relative to the total count
    const boughtPercentage = (totalPurchasePrice / totalCount) * 100;
    const listedPercentage = (totalListedPrice / totalCount) * 100;
    const profitPercentage = (totalProfit / totalCount) * 100;
    setSegments([
      { label: useTranslateSegments("bought"), count: totalPurchasePrice, part: boughtPercentage, color: getCssVariable("--negative-value") },
      { label: useTranslateSegments("listed"), count: totalListedPrice, part: listedPercentage, color: getCssVariable("--positive-value") },
      { label: useTranslateSegments("profit"), count: totalProfit, part: profitPercentage, color: getCssVariable("--profit-value") },
    ]);
  }, [items]);
  // Functions
  const CreateWTSMessages = async (items: TauriTypes.StockItem[]) => {
    items = items
      .filter((x) => !!x.list_price)
      .sort((a, b) => {
        if (a.list_price && b.list_price) return b.list_price - a.list_price;
        return 0;
      });
    let msg = CreateTradeMessage(
      "WTS Rivens",
      items.map((x) => ({ price: x.list_price || 0, name: `[${x.item_name}]` })),
      ""
    );
    notifications.show({ title: useTranslateNotifications("copied.title"), message: msg.trim(), color: "green.7" });
    navigator.clipboard.writeText(msg.trim());
  };
  // Mutations
  const createStockMutation = useMutation({
    mutationFn: (data: TauriTypes.CreateStockItem) => api.stock.item.create(data),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("create_stock.title"),
        message: useTranslateSuccess("create_stock.message", { name: u.item_name }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("create_stock.title"), message: useTranslateErrors("create_stock.message"), color: "red.7" });
    },
  });
  const updateStockMutation = useMutation({
    mutationFn: (data: TauriTypes.UpdateStockItem) => api.stock.item.update(data),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("update_stock.title"),
        message: useTranslateSuccess("update_stock.message", { name: u.item_name }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("update_stock.title"), message: useTranslateErrors("update_stock.message"), color: "red.7" });
    },
  });
  const updateBulkStockMutation = useMutation({
    mutationFn: (data: { ids: number[]; entry: TauriTypes.UpdateStockItem }) => api.stock.item.updateBulk(data.ids, data.entry),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("update_bulk_stock.title"),
        message: useTranslateSuccess("update_bulk_stock.message", { count: u }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({
        title: useTranslateErrors("update_bulk_stock.title"),
        message: useTranslateErrors("update_bulk_stock.message"),
        color: "red.7",
      });
    },
  });
  const sellStockMutation = useMutation({
    mutationFn: (data: TauriTypes.SellStockItem) => api.stock.item.sell(data),
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
  const deleteStockMutation = useMutation({
    mutationFn: (id: number) => api.stock.item.delete(id),
    onSuccess: async () => {
      notifications.show({
        title: useTranslateSuccess("delete_stock.title"),
        message: useTranslateSuccess("delete_stock.message"),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("delete_stock.title"), message: useTranslateErrors("delete_stock.message"), color: "red.7" });
    },
  });
  const deleteBulkStockMutation = useMutation({
    mutationFn: (ids: number[]) => api.stock.item.deleteBulk(ids),
    onSuccess: async () => {
      notifications.show({
        title: useTranslateSuccess("delete_bulk_stock.title"),
        message: useTranslateSuccess("delete_bulk_stock.message"),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({
        title: useTranslateErrors("delete_bulk_stock.title"),
        message: useTranslateErrors("delete_bulk_stock.message"),
        color: "red.7",
      });
    },
  });

  // Modal's
  const OpenMinimumPriceModal = (id: number, minimum_price: number) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("minimum_price.title"),
      innerProps: {
        fields: [
          {
            name: "minimum_price",
            label: useTranslateBasePrompt("minimum_price.fields.minimum_price.label"),
            attributes: {
              min: 0,
              description: useTranslateBasePrompt("minimum_price.fields.minimum_price.description"),
            },
            value: minimum_price,
            type: "number",
          },
        ],
        onConfirm: async (data: { minimum_price: number }) => {
          if (!id) return;
          const { minimum_price } = data;
          await updateStockMutation.mutateAsync({ id, minimum_price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenSellModal = (stock: TauriTypes.StockItem) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("sell.title"),
      innerProps: {
        fields: [
          {
            name: "sell",
            label: useTranslateBasePrompt("sell.fields.sell.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { sell: number }) => {
          if (!stock) return;
          const { sell } = data;
          await sellStockMutation.mutateAsync({ url: stock.wfm_url, sub_type: stock.sub_type, price: sell, quantity: 1, is_from_order: false });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenInfoModal = (item: TauriTypes.StockItem) => {
    modals.open({
      size: "100%",
      title: item.item_name,
      children: <StockItemInfo value={item} />,
    });
  };

  const OpenUpdateModal = (items: TauriTypes.UpdateStockItem[]) => {
    modals.open({
      title: useTranslatePrompt("update_bulk.title"),
      children: (
        <UpdateItemBulk
          onSubmit={async (data) => {
            await updateBulkStockMutation.mutateAsync({ ids: items.map((x) => x.id || 0), entry: data });
            modals.closeAll();
          }}
        />
      ),
    });
  };
  return (
    <Box>
      <Grid>
        <Grid.Col span={8}>
          <CreateStockItemForm
            disabled={createStockMutation.isPending || updateStockMutation.isPending || sellStockMutation.isPending || deleteStockMutation.isPending}
            onSubmit={async (item) => {
              createStockMutation.mutate({ ...item, is_from_order: false });
            }}
          />
          <Group gap={"md"} mt={"md"}>
            {Object.values(TauriTypes.StockStatus).map((status) => (
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
          <StatsWithSegments showPercent segments={segments} />
        </Grid.Col>
      </Grid>
      <DataTableSearch
        className={`${classes.databaseStockItems} ${useHasAlert() ? classes.alert : ""} ${is_running ? classes.running : ""}`}
        mt={"md"}
        records={items || []}
        customRowAttributes={(record) => {
          return {
            "data-color-mode": "box-shadow",
            "data-stock-status": record.status,
          };
        }}
        query={query}
        filters={filters}
        onSearchChange={(text) => setQuery(text)}
        rightSectionWidth={115}
        rightSection={
          <Group gap={5}>
            <ActionWithTooltip
              tooltip={useTranslateButtons("update_bulk.tooltip")}
              icon={faEdit}
              color={"green.7"}
              actionProps={{
                disabled: selectedRecords.length < 1,
              }}
              onClick={(e) => {
                e.stopPropagation();
                OpenUpdateModal(selectedRecords);
              }}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("delete_bulk.tooltip")}
              icon={faTrashCan}
              color={"red.7"}
              actionProps={{
                disabled: selectedRecords.length < 1,
              }}
              onClick={async (e) => {
                e.stopPropagation();
                modals.openConfirmModal({
                  title: useTranslateBasePrompt("delete.title"),
                  children: <Text size="sm">{useTranslateBasePrompt("delete.message", { count: selectedRecords.length })}</Text>,
                  labels: { confirm: useTranslateBasePrompt("delete.confirm"), cancel: useTranslateBasePrompt("delete.cancel") },
                  onConfirm: async () => await deleteBulkStockMutation.mutateAsync(selectedRecords.map((x) => x.id)),
                });
              }}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("wts.tooltip")}
              icon={faComment}
              color={"green.7"}
              actionProps={{
                disabled: selectedRecords.length < 1,
              }}
              onClick={(e) => {
                e.stopPropagation();
                CreateWTSMessages(selectedRecords);
              }}
            />
          </Group>
        }
        customLoader={<Loading />}
        fetching={
          createStockMutation.isPending ||
          updateStockMutation.isPending ||
          sellStockMutation.isPending ||
          deleteStockMutation.isPending ||
          updateBulkStockMutation.isPending ||
          deleteBulkStockMutation.isPending
        }
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
            accessor: "bought",
            title: useTranslateDataGridBaseColumns("bought"),
            sortable: true,
            render: ({ bought }) => <NumberFormatter thousandSeparator="." decimalSeparator="," value={bought} />,
          },
          {
            accessor: "minimum_price",
            width: 310,
            title: useTranslateDataGridBaseColumns("minimum_price.title"),
            render: ({ id, minimum_price }) => (
              <Group gap={"sm"} justify="space-between">
                <Text>{minimum_price || "N/A"}</Text>
                <Group gap={"xs"}>
                  <ButtonInterval
                    color="red.7"
                    intervals={[5, 10]}
                    prefix="-"
                    OnClick={async (int) => {
                      if (!id) return;
                      minimum_price = minimum_price || 0;
                      if (minimum_price - int < 0) return;
                      await updateStockMutation.mutateAsync({ id, minimum_price: minimum_price - int });
                    }}
                  />
                  <ButtonInterval
                    color="green.7"
                    intervals={[5, 10]}
                    prefix="+"
                    OnClick={async (int) => {
                      if (!id) return;
                      minimum_price = minimum_price || 0;
                      await updateStockMutation.mutateAsync({ id, minimum_price: minimum_price + int });
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateDataGridBaseColumns("minimum_price.btn.edit.tooltip")}
                    icon={faEdit}
                    onClick={(e) => {
                      e.stopPropagation();
                      if (!id) return;
                      OpenMinimumPriceModal(id, minimum_price || 0);
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
            accessor: "owned",
            title: useTranslateDataGridColumns("owned"),
          },
          {
            accessor: "actions",
            title: useTranslateDataGridBaseColumns("actions.title"),
            width: 180,
            render: (row) => (
              <Group gap={"sm"} justify="flex-end">
                <ActionWithTooltip
                  tooltip={useTranslateDataGridBaseColumns("actions.buttons.sell_manual.tooltip")}
                  icon={faPen}
                  color={"green.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    OpenSellModal(row);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridBaseColumns("actions.buttons.sell_auto.tooltip")}
                  icon={faHammer}
                  actionProps={{ disabled: !row.list_price, size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    if (!row.id || !row.list_price) return;
                    await sellStockMutation.mutateAsync({
                      url: row.wfm_url,
                      sub_type: row.sub_type,
                      price: row.list_price,
                      quantity: 1,
                      is_from_order: false,
                    });
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridBaseColumns(`actions.buttons.hide.${row.is_hidden ? "disabled_tooltip" : "enabled_tooltip"}`)}
                  icon={row.is_hidden ? faEyeSlash : faEye}
                  color={`${row.is_hidden ? "red.7" : "green.7"}`}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    await updateStockMutation.mutateAsync({ id: row.id, is_hidden: !row.is_hidden });
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
                      onConfirm: async () => await deleteStockMutation.mutateAsync(row.id),
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

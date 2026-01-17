import { Box, Grid, Group, NumberFormatter, Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { useMutation, useQuery } from "@tanstack/react-query";
import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import api, { OnTauriEvent } from "@api/index";
import { SearchField } from "@components/SearchField";
import { DataTable } from "mantine-datatable";
import { TextTranslate } from "@components/TextTranslate";
import { getCssVariable, GetSubTypeDisplay } from "@utils/helper";
import { useEffect, useState } from "react";
import { StatsWithSegments } from "@components/StatsWithSegments";
import { ColorInfo } from "@components/ColorInfo";
import { CreateStockItemForm } from "@components/Forms/CreateStockItem";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { faEdit, faEye, faEyeSlash, faHammer, faInfo, faPen, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import classes from "../../LiveTrading.module.css";
import { notifications } from "@mantine/notifications";
import { useLocalStorage } from "@mantine/hooks";
import { ButtonIntervals } from "@components/ButtonIntervals";
import { modals } from "@mantine/modals";
import { StockItemInfo } from "@components/Modals/StockItemInfo";
import { UpdateItemBulk } from "@components/Forms/UpdateItemBulk";

interface StockItemPanelProps {}
export const StockItemPanel = ({}: StockItemPanelProps) => {
  // States Context
  const { is_running } = useLiveScraperContext();

  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.StockItemControllerGetListParams>({
    key: "stock_item_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10 },
  });
  const [loadingRows, setLoadingRows] = useState<string[]>([]);

  // States
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.StockItem[]>([]);
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
  // const useTranslateNotifications = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslate(`notifications.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`buttons.${key}`, { ...context }, i18Key);

  // Queys
  let { data, isFetching, refetch } = useQuery({
    queryKey: ["stock_item", queryData.page, queryData.limit, queryData.sort_by, queryData.sort_direction, queryData.status],
    queryFn: () => api.stock.item.getAll(queryData),
    refetchOnWindowFocus: true,
  });
  let { data: overviewData, refetch: refetchOverview } = useQuery({
    queryKey: ["stock_item_overview"],
    queryFn: () => api.stock.item.getOverview(),
    refetchOnWindowFocus: true,
  });
  // Member
  useEffect(() => {
    const items = data?.results || [];
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
  }, [data, overviewData]);

  // Mutations
  const createStockMutation = useMutation({
    mutationFn: (data: TauriTypes.CreateStockItem) => api.stock.item.create(data),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("create_stock.title"),
        message: useTranslateSuccess("create_stock.message", { name: u.item_name }),
        color: "green.7",
      });
      refetch();
      refetchOverview();
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("create_stock.title"), message: useTranslateErrors("create_stock.message"), color: "red.7" });
    },
  });
  const updateStockMutation = useMutation({
    mutationFn: (data: TauriTypes.UpdateStockItem) => api.stock.item.update(data),
    onMutate: (row) => setLoadingRows((prev) => [...prev, `${row.id}`]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => id !== `${variables.id}`)),
    onSuccess: async (u) => {
      refetch();
      refetchOverview();
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
    onMutate: (row) => setLoadingRows((prev) => [...prev, ...row.ids.map((id) => `${id}`)]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => !variables.ids.includes(Number(id)))),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("update_bulk_stock.title"),
        message: useTranslateSuccess("update_bulk_stock.message", { count: u }),
        color: "green.7",
      });
      refetch();
      refetchOverview();
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
    onMutate: (row) => setLoadingRows((prev) => [...prev, `${row.id}`]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => id !== `${variables.id}`)),
    onSuccess: async (u) => {
      console.log("Sell Stock Mutation Success:", u);
      refetch();
      refetchOverview();
      // queryClient.setQueryData(queryKey, (oldData: TauriTypes.StockItemControllerGetListData) => {
      //   if (!oldData || !oldData.results) return oldData;
      //   const updatedResults = oldData.results.map((item) => (item.id === u.id ? u : item));
      //   return { ...oldData, results: updatedResults };
      // });
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
    onMutate: (row) => setLoadingRows((prev) => [...prev, `${row}`]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => id !== `${variables}`)),
    onSuccess: async () => {
      refetch();
      refetchOverview();
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
    onMutate: (rows) => setLoadingRows((prev) => [...prev, ...rows.map((id) => `${id}`)]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => !variables.includes(Number(id)))),
    onSuccess: async () => {
      refetch();
      refetchOverview();
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
          await sellStockMutation.mutateAsync({ id: stock.id, wfm_url: stock.wfm_url, sub_type: stock.sub_type, price: sell, quantity: 1 });
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

  useEffect(() => {
    OnTauriEvent<any>(TauriTypes.Events.RefreshStockItems, () => {
      refetch();
      refetchOverview();
    });
    return () => api.events.CleanEvent(TauriTypes.Events.RefreshStockItems);
  }, []);
  return (
    <Box>
      <Grid>
        <Grid.Col span={8}>
          <CreateStockItemForm onSubmit={async (item) => createStockMutation.mutate(item)} />
          <Group gap={"md"} mt={"md"}>
            {overviewData?.map((entry) => (
              <ColorInfo
                active={entry.key == queryData.status}
                key={entry.key}
                onClick={() =>
                  setQueryData((prev) => ({
                    ...prev,
                    status: (entry.key as TauriTypes.StockStatus) == prev.status ? undefined : (entry.key as TauriTypes.StockStatus),
                  }))
                }
                infoProps={{
                  "data-color-mode": "bg",
                  "data-stock-status": entry.key,
                }}
                text={useTranslateStockStatus(`${entry.key}`) + ` (${entry.count})`}
                tooltip={useTranslateStockStatus(`details.${entry.key}`)}
              />
            ))}
          </Group>
        </Grid.Col>
        <Grid.Col span={4}>
          <StatsWithSegments showPercent segments={segments} />
        </Grid.Col>
      </Grid>
      <SearchField
        value={queryData.query || ""}
        onSearch={() => refetch()}
        onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))}
        rightSectionWidth={95}
        rightSection={
          <Group gap={5}>
            <ActionWithTooltip
              tooltip={useTranslateButtons("update_bulk.tooltip")}
              icon={faEdit}
              color={"green.7"}
              iconProps={{ size: "sm" }}
              actionProps={{
                disabled: selectedRecords.length < 1,
                size: "sm",
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
              iconProps={{ size: "sm" }}
              actionProps={{
                disabled: selectedRecords.length < 1,
                size: "sm",
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
          </Group>
        }
      />
      <DataTable
        className={`${classes.databaseStockItems} ${useHasAlert() ? classes.alert : ""} ${is_running ? classes.running : ""}`}
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
        onCellClick={({ record, column }) => {
          switch (column.accessor) {
            case "item_name":
              navigator.clipboard.writeText(record.item_name);
              notifications.show({ title: useTranslate("notifications.copied.title"), message: record.item_name, color: "green.7" });
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
            sortable: true,
            title: useTranslateDataGridBaseColumns("minimum_price.title"),
            render: ({ id, minimum_price }) => (
              <Group gap={"sm"} justify="space-between">
                <Text>{minimum_price || "N/A"}</Text>
                <Group gap={"xs"}>
                  <ButtonIntervals
                    intervals={[5, 10]}
                    minimum_price={minimum_price || 0}
                    OnClick={async (val) => {
                      if (!id) return;
                      console.log("Update minimum price to:", val);
                      await updateStockMutation.mutateAsync({ id, minimum_price: val });
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
            sortable: true,
            title: useTranslateDataGridBaseColumns("list_price"),
          },
          {
            accessor: "owned",
            sortable: true,
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
                  loading={loadingRows.includes(`${row.id}`)}
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
                  loading={loadingRows.includes(`${row.id}`)}
                  actionProps={{ disabled: !row.list_price, size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    if (!row.id || !row.list_price) return;
                    await sellStockMutation.mutateAsync({
                      id: row.id,
                      wfm_url: row.wfm_url,
                      sub_type: row.sub_type,
                      price: row.list_price,
                      quantity: 1,
                    });
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
                    await updateStockMutation.mutateAsync({ id: row.id, is_hidden: !row.is_hidden });
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

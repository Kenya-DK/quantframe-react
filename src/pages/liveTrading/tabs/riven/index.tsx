import { Box, Grid, Group, NumberFormatter, Text } from "@mantine/core";
import { useEffect, useState } from "react";
import { getCssVariable, GetSubTypeDisplay } from "@utils/helper";
import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { faEdit, faEye, faEyeSlash, faFilter, faHammer, faInfo, faPen, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useMutation, useQuery } from "@tanstack/react-query";
import api, { OnTauriEvent } from "@api/index";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { ColorInfo } from "@components/ColorInfo";
import { StatsWithSegments } from "@components/StatsWithSegments";
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import classes from "../../LiveTrading.module.css";
import { useLocalStorage } from "@mantine/hooks";
import { SearchField } from "../../../../components/SearchField";
import { DataTable } from "mantine-datatable";
import { TextTranslate } from "../../../../components/TextTranslate";
import { ButtonIntervals } from "../../../../components/ButtonIntervals";
import { RivenAttributeCom } from "../../../../components/RivenAttribute";
import { notifications } from "@mantine/notifications";
import { modals } from "@mantine/modals";
import { StockRivenInfo } from "../../../../components/Modals/StockRivenInfo";
import { CreateRiven } from "../../../../components/Forms/CreateRiven";
import { UpdateRivenBulk } from "../../../../components/Forms/UpdateRivenBulk";
import { RivenFilter } from "../../../../components/Forms/RivenFilter";
interface StockRivenPanelProps {}
export const StockRivenPanel = ({}: StockRivenPanelProps) => {
  // States Context
  const { is_running } = useLiveScraperContext();

  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.StockRivenControllerGetListParams>({
    key: "stock_riven_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10 },
  });
  // const [loadingRows, setLoadingRows] = useState<string[]>([]);

  // States
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.StockRiven[]>([]);
  const [statusCount, setStatusCount] = useState<{ [key: string]: number }>({});
  const [segments, setSegments] = useState<{ label: string; count: number; part: number; color: string }[]>([]);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`liveTrading.${key}`, { ...context }, i18Key);
  const useTranslateSegments = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`segments.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.riven.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  const useTranslateDataGridBaseColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`success.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`prompts.${key}`, { ...context }, i18Key);
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`buttons.${key}`, { ...context }, i18Key);

  // Queys
  let { data, isFetching, refetch } = useQuery({
    queryKey: ["stock_riven", queryData.page, queryData.limit, queryData.sort_by, queryData.sort_direction, queryData.status],
    queryFn: () => api.stock.riven.getAll(queryData),
    refetchOnWindowFocus: true,
  });
  // Member
  useEffect(() => {
    if (!data) return;
    const rivens = data.results || [];
    const totalPurchasePrice = rivens.reduce((a, b) => a + (b.bought || 0), 0);
    const totalListedPrice = rivens.reduce((a, b) => a + (b.list_price || 0), 0);
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
    setStatusCount(
      Object.values(TauriTypes.StockStatus).reduce((acc, status) => {
        acc[status] = rivens.filter((item) => item.status === status).length;
        return acc;
      }, {} as { [key: string]: number })
    );
  }, [data]);
  // Mutations

  const updateStockMutation = useMutation({
    mutationFn: (data: TauriTypes.UpdateStockRiven) => api.stock.riven.update(data),
    onSuccess: async (u) => {
      refetch();
      notifications.show({
        title: useTranslateSuccess("update_stock.title"),
        message: useTranslateSuccess("update_stock.message", { name: u.weapon_name + " " + u.mod_name }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("update_stock.title"), message: useTranslateErrors("update_stock.message"), color: "red.7" });
    },
  });

  const updateBulkStockMutation = useMutation({
    mutationFn: (data: { ids: number[]; entry: TauriTypes.UpdateStockRiven }) => api.stock.riven.updateBulk(data.ids, data.entry),
    onSuccess: async (u) => {
      refetch();
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
    mutationFn: (data: TauriTypes.SellStockRiven) => api.stock.riven.sell(data),
    onSuccess: async (u) => {
      refetch();
      notifications.show({
        title: useTranslateSuccess("sell_stock.title"),
        message: useTranslateSuccess("sell_stock.message", { name: u.weapon_name + " " + u.mod_name }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("sell_stock.title"), message: useTranslateErrors("sell_stock.message"), color: "red.7" });
    },
  });

  const deleteStockMutation = useMutation({
    mutationFn: (id: number) => api.stock.riven.delete(id),
    onSuccess: async () => {
      refetch();
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
    mutationFn: (ids: number[]) => api.stock.riven.deleteBulk(ids),
    onSuccess: async () => {
      refetch();
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

  const createStockMutation = useMutation({
    mutationFn: (riven: TauriTypes.CreateStockRiven) => api.stock.riven.create(riven),
    onSuccess: async (r) => {
      refetch();
      notifications.show({
        title: useTranslateSuccess("create_riven.title"),
        message: useTranslateSuccess("create_riven.message", { name: `${r.weapon_name} ${r.mod_name}` }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("create_riven.title"), message: useTranslateErrors("create_riven.message"), color: "red.7" });
    },
  });
  // Modal's
  const OpenSellModal = (id: number) => {
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
        onConfirm: async (data: any) => {
          if (!id) return;
          const { sell } = data;
          await sellStockMutation.mutateAsync({ id, price: sell, quantity: 1 });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };
  const OpenInfoModal = (item: TauriTypes.StockRiven) => {
    modals.open({
      size: "100%",
      title: item.weapon_name + " " + item.mod_name,
      children: <StockRivenInfo value={item} />,
    });
  };
  const OpenCreateRiven = () => {
    modals.open({
      title: useTranslatePrompt("create_riven.title"),
      size: "950px",
      children: (
        <CreateRiven
          onSubmit={async (_data) => {
            await createStockMutation.mutateAsync({
              wfm_url: _data.wfm_weapon_url,
              mod_name: _data.mod_name,
              mastery_rank: _data.mastery_rank,
              re_rolls: _data.re_rolls,
              polarity: _data.polarity,
              attributes: _data.attributes,
              bought: _data.bought || 0,
              rank: _data.sub_type?.rank || 0,
            });
            modals.closeAll();
          }}
        />
      ),
    });
  };
  const OpenUpdateModal = (items: TauriTypes.UpdateStockRiven[]) => {
    modals.open({
      title: useTranslatePrompt("update_bulk.title"),
      children: (
        <UpdateRivenBulk
          onSubmit={async (data) => {
            await updateBulkStockMutation.mutateAsync({ ids: items.map((x) => x.id || 0), entry: data });
            modals.closeAll();
          }}
        />
      ),
    });
  };
  const OpenRivenFilterModal = (item: TauriTypes.StockRiven) => {
    const filter = item.filter || { enabled: false, attributes: [] };
    if (!filter.attributes) filter.attributes = item.attributes.map((x) => ({ positive: x.positive, url_name: x.url_name, is_required: false }));

    modals.open({
      title: useTranslatePrompt("update_filter.title"),
      size: "75vw",
      children: (
        <RivenFilter
          value={filter}
          onSubmit={async (data) => {
            await updateStockMutation.mutateAsync({ id: item.id, filter: data });
            modals.closeAll();
          }}
        />
      ),
    });
  };
  useEffect(() => {
    OnTauriEvent<any>(TauriTypes.Events.RefreshStockRivens, () => refetch());
    return () => api.events.CleanEvent(TauriTypes.Events.RefreshStockRivens);
  }, []);
  return (
    <Box>
      <Grid>
        <Grid.Col span={8}>
          <Group gap={"md"} mt={"md"}>
            {Object.values(TauriTypes.StockStatus).map((status) => (
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
        rightSectionWidth={115}
        onCreate={() => OpenCreateRiven()}
        rightSection={
          <Group gap={5}>
            <ActionWithTooltip
              tooltip={useTranslateButtons("update_bulk.tooltip")}
              icon={faEdit}
              color={"green.7"}
              actionProps={{ size: "sm", disabled: selectedRecords.length < 1 }}
              iconProps={{ size: "xs" }}
              onClick={(e) => {
                e.stopPropagation();
                OpenUpdateModal(selectedRecords);
              }}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("delete_bulk.tooltip")}
              icon={faTrashCan}
              color={"red.7"}
              actionProps={{ size: "sm", disabled: selectedRecords.length < 1 }}
              iconProps={{ size: "xs" }}
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
        className={`${classes.databaseStockRivens} ${useHasAlert() ? classes.alert : ""} ${is_running ? classes.running : ""}`}
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
            accessor: "weapon_name",
            title: useTranslateDataGridBaseColumns("name.title"),
            sortable: true,
            width: 300,
            render: ({ weapon_name, mod_name, sub_type }) => (
              <TextTranslate
                color="gray.4"
                i18nKey={useTranslateDataGridBaseColumns("name.value", undefined, true)}
                values={{
                  name: weapon_name + " " + mod_name,
                  sub_type: GetSubTypeDisplay(sub_type),
                }}
              />
            ),
          },
          {
            accessor: "attributes",
            title: useTranslateDataGridColumns("attributes"),
            render: ({ attributes }) => (
              <Group gap={"sm"} justify="flex-start">
                {attributes?.map((attribute, index) => (
                  <RivenAttributeCom key={index} value={{ ...attribute }} />
                ))}
              </Group>
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
                      // await updateStockMutation.mutateAsync({ id, minimum_price: val });
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateDataGridBaseColumns("minimum_price.btn.edit.tooltip")}
                    icon={faEdit}
                    onClick={(e) => {
                      e.stopPropagation();
                      if (!id) return;
                      // OpenMinimumPriceModal(id, minimum_price || 0);
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
            accessor: "actions",
            title: useTranslateDataGridBaseColumns("actions.title"),
            width: 220,
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
                    OpenSellModal(row.id);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridColumns(`actions.buttons.filter.tooltip`)}
                  icon={faFilter}
                  color={row.filter?.enabled ? "blue.7" : "gray.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    OpenRivenFilterModal(row);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridBaseColumns("actions.buttons.sell_auto.tooltip")}
                  icon={faHammer}
                  actionProps={{ size: "sm", disabled: !row.list_price }}
                  iconProps={{ size: "xs" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    if (!row.id || !row.list_price) return;
                    await sellStockMutation.mutateAsync({ id: row.id, price: row.list_price, quantity: 1 });
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
                  icon={faTrashCan}
                  color="red.7"
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

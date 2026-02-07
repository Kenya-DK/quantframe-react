import { Box, Group, NumberInput, Paper, Pill, TagsInput, Text } from "@mantine/core";
import { useEffect, useState } from "react";
import { TauriTypes } from "$types";
import { useQueries } from "./queries";
import { SearchField } from "@components/Forms/SearchField";
import { useTranslateCommon, useTranslatePages } from "@hooks/useTranslate.hook";
import classes from "../TradeMessages.module.css";
import { DataTable } from "mantine-datatable";
import { getSafePage } from "@utils/helper";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useForm } from "@mantine/form";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faCalculator, faDownload, faEdit, faMessage, faPlus, faSearch, faTrash, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { HasPermission } from "@api/index";
import { useMutations } from "./mutations";
import { Loading } from "@components/Shared/Loading";
import { ItemName } from "@components/DataDisplay/ItemName";
import { ButtonIntervals } from "@components/Shared/ButtonIntervals";
import { useModals } from "./modals";
import { useLocalStorage } from "@mantine/hooks";
import { TimerStamp } from "../../../components/Shared/TimerStamp";
interface TradeEntryListProps {
  ref?: React.Ref<any>;
  isActive?: boolean;
  group: string;
  defaultDisplaySettings?: any;
  tradeEntry: TauriTypes.CreateTradeEntry & { wfm_url: string };
  setTradeEntry: (data: TauriTypes.CreateTradeEntry & { wfm_url: string }) => void;
  createComponent?: React.ReactNode;
  rightSection?: React.ReactNode;
  rightSectionWidth?: number;
  hideColumns?: string[];
  onFindInteresting?: (mutations: ReturnType<typeof useMutations>) => void;
  onCalibratePrices?: (mutations: ReturnType<typeof useMutations>) => void;
}

export const TradeEntryList = ({
  isActive,
  group,
  defaultDisplaySettings,
  tradeEntry,
  setTradeEntry,
  createComponent,
  rightSection,
  rightSectionWidth,
  onFindInteresting,
  onCalibratePrices,
  hideColumns,
}: TradeEntryListProps) => {
  // States For DataGrid
  const queryData = useForm({
    initialValues: { page: 1, limit: 50, query: "", group } as TauriTypes.TradeEntryControllerGetListParams,
    validate: {},
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trade_messages.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`buttons.${key}`, { ...context }, i18Key);

  // States
  const [generatedSettings, setGeneratedSettings] = useLocalStorage<any>({
    key: `trade_messages_generated_settings_${group}`,
    defaultValue: defaultDisplaySettings,
  });
  const [filterOpened, setFilterOpened] = useState<boolean>(false);
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [canExport, setCanExport] = useState<boolean>(false);
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.TradeEntry[]>([]);

  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.EXPORT_DATA).then((res) => setCanExport(res));
  }, []);
  // Queries
  const { paginationQuery, refetchQueries } = useQueries({ queryData: queryData.values, isActive });

  // Mutations
  const { createMutation, createMultipleMutation, updateMutation, updateMultipleMutation, deleteMutation, deleteMultipleMutation, exportMutation } =
    useMutations({
      refetchQueries,
      setLoadingRows,
    });

  // Modals
  const { OpenPriceModal, OpenGenerateTradeMessageModal, OpenDeleteMultipleModal, OpenUpdateMultipleModal, OpenDeleteModal } = useModals({
    updateMutation,
    updateMultipleMutation,
    deleteMutation,
    deleteMultipleMutation,
  });

  const IsLoading = () => (paginationQuery.isFetching || exportMutation.isPending) && isActive;

  useEffect(() => {
    setSelectedRecords([]);
  }, [deleteMultipleMutation.isSuccess, deleteMutation.isSuccess]);

  const GetRightSectionWidth = () => {
    let baseWidth = 35 * 4;
    if (onFindInteresting) baseWidth += 35;
    if (onCalibratePrices) baseWidth += 35;
    return rightSectionWidth || baseWidth;
  };

  return (
    <Box p={"md"}>
      <Group mb="md" gap="md" align="flex-end">
        {createComponent}
        <NumberInput
          label={useTranslateFields("price_label")}
          placeholder={useTranslateFields("price_placeholder")}
          value={tradeEntry.price}
          min={1}
          onChange={(value) => setTradeEntry({ ...tradeEntry, price: Number(value) })}
        />
        <TagsInput
          label={useTranslateFields("tags_label")}
          placeholder={useTranslateFields("tags_placeholder")}
          value={tradeEntry.tags || []}
          onChange={(value) => setTradeEntry({ ...tradeEntry, tags: value })}
          rightSection={
            <ActionWithTooltip
              tooltip={useTranslateButtons("add_tooltip")}
              color="green.7"
              actionProps={{ disabled: tradeEntry.raw == "" }}
              onClick={() => createMutation.mutate(tradeEntry)}
              icon={faPlus}
            />
          }
        />
      </Group>
      <SearchField
        value={queryData.values.query || ""}
        searchDisabled={paginationQuery.isLoading}
        onChange={(text) => queryData.setFieldValue("query", text)}
        onFilterToggle={(s) => setFilterOpened(s)}
        rightSectionWidth={GetRightSectionWidth()}
        rightSection={
          <Group gap={3}>
            {rightSection}
            {onFindInteresting && (
              <ActionWithTooltip
                tooltip={useTranslateButtons("find_interesting_tooltip")}
                icon={faSearch}
                iconProps={{ size: "xs" }}
                actionProps={{ size: "sm" }}
                onClick={() =>
                  onFindInteresting({
                    createMutation,
                    createMultipleMutation,
                    updateMutation,
                    updateMultipleMutation,
                    deleteMutation,
                    deleteMultipleMutation,
                    exportMutation,
                  })
                }
              />
            )}
            {onCalibratePrices && (
              <ActionWithTooltip
                tooltip={useTranslateButtons("calibrate_prices_tooltip")}
                icon={faCalculator}
                iconProps={{ size: "xs" }}
                actionProps={{ size: "sm" }}
                onClick={() =>
                  onCalibratePrices({
                    createMutation,
                    createMultipleMutation,
                    updateMutation,
                    updateMultipleMutation,
                    deleteMutation,
                    deleteMultipleMutation,
                    exportMutation,
                  })
                }
              />
            )}
            <ActionWithTooltip
              tooltip={useTranslateButtons("generate_trade_messages_tooltip", {
                count: selectedRecords.length > 0 ? selectedRecords.length : paginationQuery.data?.results?.length || 0,
              })}
              icon={faMessage}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm" }}
              onClick={async () => {
                let filteredRecords = selectedRecords.length > 0 ? selectedRecords : paginationQuery.data?.results || [];
                OpenGenerateTradeMessageModal({
                  ...generatedSettings,
                  items: filteredRecords,
                  onSave: (data: any) => setGeneratedSettings(data),
                });
              }}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("export_json_tooltip")}
              icon={faDownload}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: !canExport || IsLoading() }}
              onClick={() => exportMutation.mutate(queryData.values)}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("update_multiple_tooltip")}
              icon={faEdit}
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: selectedRecords.length === 0 }}
              onClick={() => OpenUpdateMultipleModal(selectedRecords.map((r) => r.id))}
            />
            <ActionWithTooltip
              tooltip={useTranslateButtons("delete_multiple_tooltip", { count: selectedRecords.length })}
              icon={faTrashCan}
              color="red.7"
              iconProps={{ size: "xs" }}
              actionProps={{ size: "sm", disabled: selectedRecords.length === 0 }}
              onClick={() => OpenDeleteMultipleModal(selectedRecords.map((r) => r.id))}
            />
          </Group>
        }
        filter={
          <Paper p={"sm"} mt={"md"}>
            <TagsInput
              label={useTranslateFields("tags_label")}
              placeholder={useTranslateFields("tags_placeholder")}
              value={queryData.values.tags || []}
              onChange={(value) => queryData.setFieldValue("tags", value)}
            />
          </Paper>
        }
      />
      <DataTable
        className={`${classes.databaseItem} ${useHasAlert() ? classes.alert : ""} ${filterOpened ? classes.filterOpened : ""}`}
        mt={"md"}
        striped
        customLoader={<Loading />}
        fetching={IsLoading()}
        records={IsLoading() ? [] : (paginationQuery.data?.results || [])}
        page={getSafePage(queryData.values.page, paginationQuery.data?.total_pages)}
        onPageChange={(page) => queryData.setFieldValue("page", page)}
        totalRecords={paginationQuery.data?.total || 0}
        recordsPerPage={queryData.values.limit || 10}
        recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
        onRecordsPerPageChange={(limit) => queryData.setFieldValue("limit", limit)}
        sortStatus={{
          columnAccessor: queryData.values.sort_by || "name",
          direction: queryData.values.sort_direction || "desc",
        }}
        onSortStatusChange={(sort) => {
          if (!sort || !sort.columnAccessor) return;
          queryData.setFieldValue("sort_by", sort.columnAccessor as any);
          queryData.setFieldValue("sort_direction", sort.direction);
        }}
        selectedRecords={selectedRecords}
        onSelectedRecordsChange={setSelectedRecords}
        // define columns
        columns={[
          {
            accessor: "name",
            title: useTranslateCommon("item_name.title"),
            sortable: true,
            width: 250,
            render: (row) => <ItemName color="gray.4" size="md" value={row} />,
          },
          {
            accessor: "price",
            title: useTranslate("datatable.columns.price"),
            sortable: true,
            render: ({ id, price }) => (
              <Group gap={"sm"} justify="space-between">
                <Text>{price}</Text>
                <Group gap={"xs"}>
                  <ButtonIntervals
                    intervals={[5, 10]}
                    minimum_price={price || 0}
                    OnClick={async (value) => {
                      updateMutation.mutateAsync({ id, price: value });
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslate("edit_tooltip")}
                    icon={faEdit}
                    onClick={(e) => {
                      e.stopPropagation();
                      OpenPriceModal(id, price || 0);
                    }}
                    actionProps={{ size: "sm" }}
                    iconProps={{ size: "xs" }}
                  />
                </Group>
              </Group>
            ),
          },
          {
            accessor: "min_price",
            hidden: hideColumns?.includes("min_price"),
            title: useTranslate("datatable.columns.min_price"),
            sortable: true,
            render: ({ properties }) => properties?.min_price || "N/A",
          },
          {
            accessor: "updated_at",
            hidden: hideColumns?.includes("updated_at"),
            title: useTranslate("datatable.columns.updated_at"),
            sortable: true,
            render: (row) => <TimerStamp date={new Date(row.updated_at)} />,
          },
          {
            accessor: "potential_profit",
            hidden: hideColumns?.includes("potential_profit"),
            title: useTranslate("datatable.columns.potential_profit"),
            sortable: true,
            render: ({ properties }) => properties?.potential_profit || "N/A",
          },
          {
            accessor: "tags",
            title: useTranslate("datatable.columns.tags"),
            sortable: true,
            render: (row) => row.tags?.split(",").map((tag: string, index: number) => <Pill key={index}>{tag}</Pill>),
          },
          {
            accessor: "actions",
            title: useTranslateCommon("datatable_columns.actions.title"),
            width: 75,
            render: (row) => (
              <Group gap={3}>
                <ActionWithTooltip
                  tooltip={useTranslateCommon("datatable_columns.actions.buttons.edit_tooltip")}
                  icon={faEdit}
                  loading={loadingRows.includes(`${row.id}`)}
                  iconProps={{ size: "xs" }}
                  actionProps={{ size: "sm" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    OpenUpdateMultipleModal([row.id]);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateCommon("datatable_columns.actions.buttons.delete_tooltip")}
                  icon={faTrash}
                  color="red"
                  loading={loadingRows.includes(`${row.id}`)}
                  iconProps={{ size: "xs" }}
                  actionProps={{ size: "sm" }}
                  onClick={async (e) => {
                    e.stopPropagation();
                    OpenDeleteModal(row.id);
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

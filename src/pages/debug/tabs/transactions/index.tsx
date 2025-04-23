import { Box, Group, Text } from "@mantine/core";
import { useWarframeMarketContextContext } from "@contexts/warframeMarket.context";
import { useEffect, useState } from "react";
import { TransactionDto, TransactionItemType, TransactionType, UpdateTransactionDto } from "@api/types";
import { DataTable, DataTableSortStatus } from "mantine-datatable";
import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { paginate, GetSubTypeDisplay } from "@utils/helper";
import classes from "../../Debug.module.css";
import dayjs from "dayjs";
import { faEdit, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useMutation } from "@tanstack/react-query";
import api from "@api/index";
import { notifications } from "@mantine/notifications";
import { modals } from "@mantine/modals";
import { UpdateTransaction } from "@components/Forms/UpdateTransaction";
import { ColorInfo } from "@components/ColorInfo";
import { Loading } from "@components/Loading";
import { TextTranslate } from "@components/TextTranslate";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { SearchField } from "@components/SearchField";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { SortDirection, SortItems } from "@utils/sorting.helper";

interface TransactionPanelProps {}
export const TransactionPanel = ({}: TransactionPanelProps) => {
  // States Context
  const { transactions } = useWarframeMarketContextContext();

  // States For DataGrid
  const [page, setPage] = useState(1);
  const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, setPageSize] = useState(pageSizes[4]);
  const [rows, setRows] = useState<TransactionDto[]>([]);
  const [totalRecords, setTotalRecords] = useState<number>(0);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus<TransactionDto>>({ columnAccessor: "id", direction: "desc" });
  const [selectedRecords, setSelectedRecords] = useState<TransactionDto[]>([]);

  const [query, setQuery] = useState<string>("");
  const [statusCount, setStatusCount] = useState<{ [key: string]: number }>({}); // Count of each status
  const [filterTransactionType, setFilterTransactionType] = useState<TransactionType | undefined>(undefined);
  const [filterItemType, setFilterItemType] = useState<TransactionItemType | undefined>(undefined);

  // Translate general
  const useTranslateTabTransactions = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`debug.tabs.transaction.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabTransactions(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateTransactionType = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`transaction_type.${key}`, { ...context }, i18Key);
  const useTranslateTransactionItemType = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`item_type.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabTransactions(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabTransactions(`success.${key}`, { ...context }, i18Key);
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabTransactions(`prompts.${key}`, { ...context }, i18Key);
  // Update Database Rows
  useEffect(() => {
    if (!transactions) return;
    let items = transactions;
    setStatusCount(() => {
      let items: { [key: string]: number } = {};
      // Create a transaction type count
      Object.values(TransactionType).forEach((status) => {
        items[status] = transactions.filter((item) => item.transaction_type === status).length;
      });

      // Create a transaction item type count
      Object.values(TransactionItemType).forEach((type) => {
        items[type] = transactions.filter((item) => item.item_type === type).length;
      });
      return items;
    });

    if (query !== "") items = items.filter((item) => item.item_name.toLowerCase().includes(query.toLowerCase()));

    if (filterTransactionType) items = items.filter((item) => item.transaction_type === filterTransactionType);

    if (filterItemType) items = items.filter((item) => item.item_type === filterItemType);

    setTotalRecords(items.length);
    items = SortItems(items, {
      field: sortStatus.columnAccessor,
      direction: sortStatus.direction as SortDirection,
    });

    items = paginate(items, page, pageSize);
    setRows(items);
    setSelectedRecords([]);
  }, [transactions, query, pageSize, page, sortStatus, filterTransactionType, filterItemType]);
  useEffect(() => {
    setSelectedRecords([]);
  }, [query, pageSize, page, sortStatus, filterTransactionType, filterItemType]);

  // Mutations
  const updateTransactionMutation = useMutation({
    mutationFn: (data: UpdateTransactionDto) => api.transaction.update(data),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("update_transaction.title"),
        message: useTranslateSuccess("update_transaction.message", { name: u.item_name }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({
        title: useTranslateErrors("update_transaction.title"),
        message: useTranslateErrors("update_transaction.message"),
        color: "red.7",
      });
    },
  });
  const deleteTransactionMutation = useMutation({
    mutationFn: (id: number) => api.transaction.delete(id),
    onSuccess: async () => {
      notifications.show({
        title: useTranslateSuccess("delete_transaction.title"),
        message: useTranslateSuccess("delete_transaction.message"),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({
        title: useTranslateErrors("delete_transaction.title"),
        message: useTranslateErrors("delete_transaction.message"),
        color: "red.7",
      });
    },
  });
  // Modal's
  const OpenUpdateModal = (transaction: UpdateTransactionDto) => {
    modals.open({
      title: useTranslatePrompt("update.title"),
      children: (
        <UpdateTransaction
          value={transaction}
          onSubmit={async (data) => {
            await updateTransactionMutation.mutateAsync(data);
            modals.closeAll();
          }}
        />
      ),
    });
  };
  return (
    <Box>
      <SearchField value={query} onChange={(text) => setQuery(text)} rightSectionWidth={115} rightSection={<Group gap={5}></Group>} />
      <Group gap={"md"} mt={"md"} grow>
        <Group>
          {Object.values([TransactionType.Purchase, TransactionType.Sale]).map((status) => (
            <ColorInfo
              active={status == filterTransactionType}
              key={status}
              onClick={() => {
                setFilterTransactionType((s) => (s === status ? undefined : status));
              }}
              infoProps={{
                "data-color-mode": "bg",
                "data-trade-type": status,
              }}
              text={useTranslateTransactionType(`${status}`) + `${statusCount[status] == 0 ? "" : ` (${statusCount[status]})`}`}
              tooltip={useTranslateTransactionType(`details.${status}`)}
            />
          ))}
        </Group>
        <Group justify="flex-end">
          {Object.values(TransactionItemType).map((type) => (
            <ColorInfo
              active={type == filterItemType}
              key={type}
              onClick={() => {
                setFilterItemType((s) => (s === type ? undefined : type));
              }}
              infoProps={{
                "data-color-mode": "bg",
                "data-item-type": type,
              }}
              text={useTranslateTransactionItemType(`${type}`) + `${statusCount[type] == 0 ? "" : ` (${statusCount[type]})`}`}
              tooltip={useTranslateTransactionItemType(`details.${type}`)}
            />
          ))}
        </Group>
      </Group>
      <DataTable
        className={`${classes.transactions} ${useHasAlert() ? classes.alert : ""}`}
        mt={"md"}
        records={rows}
        totalRecords={totalRecords}
        customRowAttributes={(record) => {
          return {
            "data-color-mode": "box-shadow",
            "data-trade-type": record.transaction_type,
          };
        }}
        withTableBorder
        customLoader={<Loading />}
        // fetching={createStockMutation.isPending || updateStockMutation.isPending || sellStockMutation.isPending || deleteStockMutation.isPending || updateBulkStockMutation.isPending || deleteBulkStockMutation.isPending}
        withColumnBorders
        page={page}
        recordsPerPage={pageSize}
        idAccessor={"id"}
        onPageChange={(p) => setPage(p)}
        recordsPerPageOptions={pageSizes}
        onRecordsPerPageChange={setPageSize}
        sortStatus={sortStatus}
        onSortStatusChange={setSortStatus}
        selectedRecords={selectedRecords}
        onSelectedRecordsChange={setSelectedRecords}
        // define columns
        columns={[
          {
            accessor: "id",
            title: useTranslateDataGridColumns("id.title"),
            sortable: true,
          },
          {
            accessor: "item_name",
            title: useTranslateDataGridColumns("name.title"),
            sortable: true,
            render: ({ item_name, sub_type, properties }) => (
              <TextTranslate
                color="gray.4"
                i18nKey={useTranslateDataGridColumns("name.value", undefined, true)}
                values={{
                  name: item_name,
                  mod_name: properties ? properties.mod_name : "",
                  sub_type: GetSubTypeDisplay(sub_type),
                }}
              />
            ),
          },
          {
            accessor: "item_type",
            title: useTranslateDataGridColumns("item_type.title"),
            sortable: true,
            render: ({ item_type }) => (
              <Text data-color-mode="text" data-item-type={item_type}>
                {useTranslateTransactionItemType(item_type)}
              </Text>
            ),
          },
          {
            accessor: "quantity",
            title: useTranslateDataGridColumns("quantity.title"),
            sortable: true,
          },
          {
            accessor: "price",
            title: useTranslateDataGridColumns("price.title"),
            sortable: true,
          },
          {
            accessor: "created_at",
            title: useTranslateDataGridColumns("created_at.title"),
            sortable: true,
            render: ({ created_at }) => {
              return <Text>{dayjs(created_at).format("DD.MM.YYYY HH:mm")}</Text>;
            },
          },
          {
            accessor: "actions",
            title: useTranslateDataGridColumns("actions.title"),
            width: 100,
            render: (row) => (
              <Group gap={"sm"}>
                <ActionWithTooltip
                  tooltip={useTranslateDataGridColumns("actions.buttons.update.tooltip")}
                  icon={faEdit}
                  color={"blue.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    OpenUpdateModal(row);
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateDataGridColumns("actions.buttons.delete.tooltip")}
                  icon={faTrashCan}
                  color={"red.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    modals.openConfirmModal({
                      title: useTranslatePrompt("delete.title"),
                      children: <Text size="sm">{useTranslatePrompt("delete.message", { name: row.item_name })}</Text>,
                      labels: { confirm: useTranslatePrompt("delete.confirm"), cancel: useTranslatePrompt("delete.cancel") },
                      onConfirm: async () => await deleteTransactionMutation.mutateAsync(row.id),
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

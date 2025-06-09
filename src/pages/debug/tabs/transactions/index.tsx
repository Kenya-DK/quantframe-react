import { Box, Group, Text } from "@mantine/core";
import { useEffect, useState } from "react";
import { TauriTypes } from "$types";
import { DataTable } from "mantine-datatable";
import { useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { GetSubTypeDisplay } from "@utils/helper";
import classes from "../../Debug.module.css";
import dayjs from "dayjs";
import { faEdit, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useMutation, useQuery } from "@tanstack/react-query";
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
import { useLiveScraperContext } from "@contexts/liveScraper.context";
import { useLocalStorage } from "@mantine/hooks";

interface TransactionPanelProps {}
export const TransactionPanel = ({}: TransactionPanelProps) => {
  // States Context
  useLiveScraperContext();

  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.TransactionControllerGetListParams>({
    key: "transaction_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10 },
  });
  const [loadingRows, setLoadingRows] = useState<string[]>([]);
  const [statusCount, setStatusCount] = useState<{ [key: string]: number }>({});

  // States
  const [selectedRecords, setSelectedRecords] = useState<TauriTypes.TransactionDto[]>([]);

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

  // Queys
  let { data, isFetching, refetch } = useQuery({
    queryKey: [
      "transactions",
      queryData.page,
      queryData.limit,
      queryData.sort_by,
      queryData.sort_direction,
      queryData.transaction_type,
      queryData.item_type,
    ],
    queryFn: () => api.transaction.getAll(queryData),
    refetchOnWindowFocus: true,
  });
  // Member
  useEffect(() => {
    const transactions = data?.results || [];

    setStatusCount(() => {
      let items: { [key: string]: number } = {};
      // Create a transaction type count
      Object.values(TauriTypes.TransactionType).forEach((status) => {
        items[status] = transactions.filter((item) => item.transaction_type === status).length;
      });

      // Create a transaction item type count
      Object.values(TauriTypes.TransactionItemType).forEach((type) => {
        items[type] = transactions.filter((item) => item.item_type === type).length;
      });
      return items;
    });
  }, [data]);
  // Mutations
  const updateTransactionMutation = useMutation({
    mutationFn: (data: TauriTypes.UpdateTransactionDto) => api.transaction.update(data),
    onMutate: (row) => setLoadingRows((prev) => [...prev, `${row}`]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => id !== `${variables}`)),
    onSuccess: async (u) => {
      refetch();
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
    onMutate: (row) => setLoadingRows((prev) => [...prev, `${row}`]),
    onSettled: (_data, _error, variables) => setLoadingRows((prev) => prev.filter((id) => id !== `${variables}`)),
    onSuccess: async () => {
      refetch();
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
  const OpenUpdateModal = (transaction: TauriTypes.UpdateTransactionDto) => {
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
      <SearchField value={queryData.query || ""} onSearch={() => refetch()} onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))} />
      <Group gap={"md"} mt={"md"} grow>
        <Group>
          {Object.values([TauriTypes.TransactionType.Purchase, TauriTypes.TransactionType.Sale]).map((status) => (
            <ColorInfo
              active={status == queryData.transaction_type}
              key={status}
              onClick={() => setQueryData((prev) => ({ ...prev, transaction_type: status == prev.transaction_type ? undefined : status }))}
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
          {Object.values(TauriTypes.TransactionItemType).map((type) => (
            <ColorInfo
              active={type == queryData.item_type}
              key={type}
              onClick={() => setQueryData((prev) => ({ ...prev, item_type: type == prev.item_type ? undefined : type }))}
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
        customRowAttributes={(record) => {
          return {
            "data-color-mode": "box-shadow",
            "data-trade-type": record.transaction_type,
          };
        }}
        withTableBorder
        customLoader={<Loading />}
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
                  loading={loadingRows.includes(`${row.id}`)}
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
                  loading={loadingRows.includes(`${row.id}`)}
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

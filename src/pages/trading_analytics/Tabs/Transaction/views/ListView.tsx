import { TauriTypes } from "$types";
import { ItemName } from "@components/DataDisplay/ItemName";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faHammer, faTrash } from "@fortawesome/free-solid-svg-icons";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { getSafePage } from "@utils/helper";
import { Group, NumberFormatter, Text } from "@mantine/core";
import dayjs from "dayjs";
import { DataTable, DataTableSortStatus } from "mantine-datatable";
import classes from "../../../TradingAnalytics.module.css";

export type ListViewProps = {
  records: TauriTypes.TransactionDto[];
  fetching: boolean;
  page: number;
  totalPages?: number;
  totalRecords: number;
  recordsPerPage: number;
  filterOpened: boolean;
  loadingRows: string[];
  selectedRecords: TauriTypes.TransactionDto[];
  sortStatus: DataTableSortStatus<TauriTypes.TransactionDto>;
  onPageChange: (page: number) => void;
  onRecordsPerPageChange: (limit: number) => void;
  onSelectedRecordsChange: (records: TauriTypes.TransactionDto[]) => void;
  onSortStatusChange: (sort: DataTableSortStatus<TauriTypes.TransactionDto>) => void;
  onEdit: (record: TauriTypes.TransactionDto) => void;
  onDelete: (id: number) => void;
};

export function ListView({
  records,
  fetching,
  page,
  totalPages,
  totalRecords,
  recordsPerPage,
  filterOpened,
  loadingRows,
  selectedRecords,
  sortStatus,
  onPageChange,
  onRecordsPerPageChange,
  onSelectedRecordsChange,
  onSortStatusChange,
  onEdit,
  onDelete,
}: ListViewProps) {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.${key}`, { ...context }, i18Key);
  const useTranslateTabItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.transaction.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabItem(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateTransactionItemType = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`item_type.${key}`, { ...context }, i18Key);

  return (
    <DataTable
      className={`${classes.databaseTransactions} ${useHasAlert() ? classes.alert : ""} ${filterOpened ? classes.filterOpened : ""}`}
      mt={"md"}
      striped
      fetching={fetching}
      records={records}
      page={getSafePage(page, totalPages)}
      onPageChange={onPageChange}
      totalRecords={totalRecords}
      recordsPerPage={recordsPerPage}
      recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
      onRecordsPerPageChange={onRecordsPerPageChange}
      customRowAttributes={(record) => {
        return {
          "data-color-mode": "box-shadow",
          "data-transaction-type": record.transaction_type,
        };
      }}
      selectedRecords={selectedRecords}
      onSelectedRecordsChange={onSelectedRecordsChange}
      sortStatus={sortStatus}
      onSortStatusChange={onSortStatusChange}
      // define columns
      columns={[
        {
          accessor: "item_name",
          title: useTranslateCommon("item_name.title"),
          sortable: true,
          width: 250,
          render: (row) => <ItemName color="gray.4" size="md" value={row} />,
        },
        {
          accessor: "item_type",
          title: useTranslateDataGridColumns("item_type"),
          sortable: true,
          render: ({ item_type }) => (
            <Text data-color-mode="text" data-item-type={item_type}>
              {useTranslateTransactionItemType(item_type)}
            </Text>
          ),
        },
        {
          accessor: "user_name",
          title: useTranslateDataGridColumns("user_name"),
          sortable: true,
        },
        {
          accessor: "quantity",
          title: useTranslateCommon("datatable_columns.quantity.title"),
          sortable: true,
        },
        {
          accessor: "price",
          title: useTranslateDataGridColumns("price"),
          sortable: true,
        },
        {
          accessor: "profit",
          title: useTranslateDataGridColumns("profit"),
          sortable: true,
          render: ({ profit }) => (profit ? <Text c={profit >= 0 ? "green.7" : "red.7"}>{profit.toFixed(2)}</Text> : <Text>N/A</Text>),
        },
        {
          accessor: "credits",
          title: useTranslateDataGridColumns("credits"),
          sortable: true,
          render: ({ credits }) => <NumberFormatter value={credits} thousandSeparator="," thousandsGroupStyle="thousand" />,
        },
        {
          accessor: "created_at",
          title: useTranslateDataGridColumns("created_at"),
          sortable: true,
          render: ({ created_at }) => {
            return <Text>{dayjs(created_at).format("DD.MM.YYYY HH:mm")}</Text>;
          },
        },
        {
          accessor: "actions",
          title: useTranslateCommon("datatable_columns.actions.title"),
          width: 75,
          render: (row) => (
            <Group gap={3}>
              <ActionWithTooltip
                tooltip={useTranslateCommon("datatable_columns.actions.buttons.edit_tooltip")}
                icon={faHammer}
                loading={loadingRows.includes(`${row.id}`)}
                iconProps={{ size: "xs" }}
                actionProps={{ size: "sm" }}
                onClick={async (e) => {
                  e.stopPropagation();
                  onEdit(row);
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
                  onDelete(row.id);
                }}
              />
            </Group>
          ),
        },
      ]}
    />
  );
}

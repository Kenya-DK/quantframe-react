import { Box, Text } from "@mantine/core";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { useQueries } from "./queries";
import { SearchField } from "@components/Forms/SearchField";
import { DataTable } from "mantine-datatable";
import { getSafePage } from "@utils/helper";
import { Loading } from "@components/Shared/Loading";
import dayjs from "dayjs";
import { modals } from "@mantine/modals";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";

interface PurchasePanelProps {
  isActive?: boolean;
}

export const PurchasePanel = ({ isActive }: PurchasePanelProps = {}) => {
  // States For DataGrid
  const queryData = useForm({
    initialValues: { page: 1, limit: 10, query: "" } as TauriTypes.WFGDPRPurchaseControllerGetListParams,
  });

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trading_analytics.tabs.wfgdpr.purchase.${key}`, { ...context }, i18Key);
  const useTranslateColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`datatable.columns.${key}`, { ...context }, i18Key);
  // Queries
  const { paginationQuery, refetchQueries } = useQueries({ queryData: queryData.values, isActive });
  const handleRefresh = () => {
    refetchQueries();
  };
  // Use the custom hook for Tauri events
  useTauriEvent(TauriTypes.Events.RefreshWFGDPRAll, handleRefresh, []);
  return (
    <Box h={"85vh"} p={"md"}>
      <SearchField
        value={queryData.values.query || ""}
        onSearch={() => {
          queryData.validate();
          if (queryData.isValid()) refetchQueries();
        }}
        onChange={(text) => queryData.setFieldValue("query", text)}
      />
      <DataTable
        mt={"md"}
        height={"75vh"}
        fetching={paginationQuery.isFetching}
        records={paginationQuery.data?.results || []}
        page={getSafePage(queryData.values.page, paginationQuery.data?.total_pages)}
        onPageChange={(page) => queryData.setFieldValue("page", page)}
        totalRecords={paginationQuery.data?.total || 0}
        recordsPerPage={queryData.values.limit || 10}
        recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
        onRecordsPerPageChange={(limit) => queryData.setFieldValue("limit", limit)}
        customLoader={<Loading />}
        sortStatus={{
          columnAccessor: queryData.values.sort_by || "name",
          direction: queryData.values.sort_direction || "desc",
        }}
        onSortStatusChange={(sort) => {
          if (!sort || !sort.columnAccessor) return;
          queryData.setFieldValue("sort_by", sort.columnAccessor as any);
          queryData.setFieldValue("sort_direction", sort.direction);
        }}
        idAccessor={(record) => record.date}
        onRowClick={({ record }) => {
          modals.open({
            title: "Purchase Details",
            size: "lg",
            children: (
              <Box>
                <Text>
                  <strong>Shop ID:</strong> {record.shop_id}
                </Text>
                <Text>
                  <strong>Price:</strong> {record.price}
                </Text>
                <Text>
                  <strong>Date:</strong> {dayjs(record.date).format("DD.MM.YYYY HH:mm")}
                </Text>
                <Text>
                  <strong>Items Received:</strong>
                </Text>
                <Box ml={20}>
                  {record.items_received.map(([itemName, quantity], index) => (
                    <Text key={index}>
                      {itemName} x{quantity}
                    </Text>
                  ))}
                </Box>
              </Box>
            ),
          });
        }}
        // define columns
        columns={[
          {
            accessor: "shop_id",
            title: useTranslateColumns("shop_id"),
            sortable: true,
          },
          {
            accessor: "price",
            title: useTranslateColumns("price"),
            sortable: true,
          },
          {
            accessor: "date",
            title: useTranslateColumns("date"),
            sortable: true,
            render: ({ date }) => {
              return <Text>{dayjs(date).format("DD.MM.YYYY HH:mm")}</Text>;
            },
          },
        ]}
      />
    </Box>
  );
};

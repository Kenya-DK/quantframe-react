import { Box, Group } from "@mantine/core";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import api from "@api/index";
import { SearchField } from "../../../../components/SearchField";
import { DataTable } from "mantine-datatable";

interface StockItemPanelProps {}
export const StockItemPanel = ({}: StockItemPanelProps) => {
  // States For DataGrid
  const dataGridState = useForm({ initialValues: { page: 1, limit: 10, query: "" } as TauriTypes.StockItemControllerGetListParams });
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`roles.${key}`, { ...context }, i18Key);
  const useTranslateDataTable = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`datatable.${key}`, { ...context }, i18Key);
  const useTranslateDataGridBaseColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`datatable.columns.${key}`, { ...context }, i18Key);

  // Queys
  const { data, isFetching, refetch } = useQuery({
    queryKey: ["role", dataGridState.values.page, dataGridState.values.limit, dataGridState.values.sort_by, dataGridState.values.sort_direction],
    queryFn: () => api.stock.item.getAll(dataGridState.values),
    refetchOnWindowFocus: false,
  });
  return (
    <Box>
      <SearchField
        value={dataGridState.values.query || ""}
        onSearch={() => refetch()}
        onChange={(text) => dataGridState.setFieldValue("query", text)}
      />
      <DataTable
        mt={"md"}
        height={`calc(100vh - 234px)`}
        striped
        fetching={isFetching}
        records={data?.results || []}
        page={dataGridState.values.page}
        onPageChange={(page) => dataGridState.setFieldValue("page", page)}
        totalRecords={data?.total}
        recordsPerPage={dataGridState.values.limit}
        recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
        onRecordsPerPageChange={(limit) => dataGridState.setFieldValue("limit", limit)}
        sortStatus={{
          columnAccessor: dataGridState.values.sort_by || "name",
          direction: dataGridState.values.sort_direction || "desc",
        }}
        onSortStatusChange={(sort) => {
          if (!sort || !sort.columnAccessor) return;
          dataGridState.setFieldValue("sort_by", sort.columnAccessor as any);
          dataGridState.setFieldValue("sort_direction", sort.direction);
        }}
        // define columns
        columns={[
          {
            accessor: "name",
            sortable: true,
            title: useTranslateDataTable("columns.name"),
          },
          {
            accessor: "actions",
            title: useTranslateDataGridBaseColumns("actions.title"),
            width: 100,
            render: () => <Group> </Group>,
          },
        ]}
      />
    </Box>
  );
};

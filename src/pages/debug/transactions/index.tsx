import { Text, Card, Group, Tooltip, ActionIcon, Box, TextInput, Select } from "@mantine/core";
import { useWarframeMarketContextContext } from "../../../contexts";
import { DataTable, DataTableSortStatus } from "mantine-datatable";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCheck, faCopy, faEdit, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useEffect, useState } from "react";
import { paginate, sortArray } from "../../../utils";
import { SearchField } from "../../../components/searchfield";
import { TransactionEntryDto } from "../../../types";
import { useMutation } from "@tanstack/react-query";
import api from "../../../api";
import { notifications } from "@mantine/notifications";
import { modals } from "@mantine/modals";
import { useForm } from "@mantine/form";
import dayjs from "dayjs";

export const Transactions = () => {
  const { transactions } = useWarframeMarketContextContext();
  // States For DataGrid
  const [page, setPage] = useState(1);
  const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, setPageSize] = useState(pageSizes[4]);
  const [rows, setRows] = useState<TransactionEntryDto[]>([]);
  const [totalRecords, setTotalRecords] = useState<number>(0);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus>({ columnAccessor: 'price', direction: 'desc' });
  const [query, setQuery] = useState<string>("");

  const filterForm = useForm({
    initialValues: {
      name: "",
      item_type: "",
      transaction_type: "",
      rank: "",
      quantity: "",
      price: "",
      created: "",
    },
  });


  // Update DataGrid Rows
  useEffect(() => {
    if (!transactions)
      return;
    let rivensFilter = transactions;
    if (query !== "") {
      rivensFilter = rivensFilter.filter((riven) => riven.name.toLowerCase().includes(query.toLowerCase()));
    }


    if (filterForm.values.name !== "") {
      rivensFilter = rivensFilter.filter((riven) => riven.name.toLowerCase().includes(filterForm.values.name.toLowerCase()));
    }

    if (filterForm.values.item_type !== "") {
      rivensFilter = rivensFilter.filter((riven) => riven.item_type.toLowerCase().includes(filterForm.values.item_type.toLowerCase()));
    }

    if (filterForm.values.transaction_type !== "") {
      rivensFilter = rivensFilter.filter((riven) => riven.transaction_type.toLowerCase().includes(filterForm.values.transaction_type.toLowerCase()));
    }

    setTotalRecords(rivensFilter.length);
    rivensFilter = sortArray([{
      field: sortStatus.columnAccessor,
      direction: sortStatus.direction
    }], rivensFilter);
    rivensFilter = paginate(rivensFilter, page, pageSize);
    setRows(rivensFilter);
  }, [transactions, query, pageSize, page, sortStatus, filterForm.values])

  const deleteEntryMutation = useMutation((id: number) => api.transactions.delete(id), {
    onSuccess: async () => {
      notifications.show({
        title: "Success",
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: "Entry deleted",
        color: "green"
      });
    },
    onError: () => {

    },
  })
  const updateEntryMutation = useMutation((data: { id: number, transaction: Partial<TransactionEntryDto> }) => api.transactions.update(data.id, data.transaction), {
    onSuccess: async () => {
      notifications.show({
        title: "Success",
        icon: <FontAwesomeIcon icon={faCheck} />,
        message: "Entry updated",
        color: "green"
      });
    },
    onError: () => { },
  })
  return (
    <Card>
      <Group position="apart" mb="xs">
        <Text weight={500}>Transactions</Text>
      </Group>
      <SearchField value={query} onChange={(text) => setQuery(text)} />
      <DataTable
        sx={{ marginTop: "20px" }}
        striped
        withColumnBorders
        records={rows}
        page={page}
        onPageChange={setPage}
        totalRecords={totalRecords}
        recordsPerPage={pageSize}
        recordsPerPageOptions={pageSizes}
        onRecordsPerPageChange={setPageSize}
        sortStatus={sortStatus}
        onSortStatusChange={setSortStatus}
        // define columns
        columns={[
          {
            accessor: 'id',
            title: "Id",
          },
          {
            accessor: 'name',
            title: "Name",
            sortable: true,
            filter: (
              <TextInput
                value={filterForm.values.name}
                onChange={(event) => filterForm.setFieldValue('name', event.currentTarget.value)}
                error={filterForm.errors.webhook && 'Invalid Webhook'}
              />
            ),
          },
          {
            accessor: 'item_type',
            title: "Item Type",
            sortable: true,
            filter: (
              <Select
                value={filterForm.values.item_type}
                onChange={(event) => filterForm.setFieldValue('item_type', event || "")}
                data={[
                  { value: "", label: "Any" },
                  { value: "item", label: "Item" },
                  { value: "riven", label: "Riven" },
                ]}
              />
            ),
          },
          {
            accessor: 'transaction_type',
            title: "Type",
            sortable: true,
            filter: (
              <Select
                value={filterForm.values.transaction_type}
                onChange={(event) => filterForm.setFieldValue('transaction_type', event || "")}
                data={[
                  { value: "", label: "Any" },
                  { value: "buy", label: "Buy" },
                  { value: "sell", label: "Sell" },
                ]}
              />
            ),
            render: ({ id, transaction_type }) => <Group grow position="apart" >
              <Text>{transaction_type || 0}</Text>
              <Box w={25} display="flex" sx={{ justifyContent: "flex-end" }}>
                <Tooltip label={"Edit Type"}>
                  <ActionIcon size={"sm"} color={"blue.7"} variant="filled" onClick={async (e) => {
                    e.stopPropagation();
                    modals.openContextModal({
                      modal: 'prompt',
                      title: "Type",
                      innerProps: {
                        fields: [
                          {
                            name: 'transaction_type',
                            label: "Type",
                            value: transaction_type,
                            type: 'text',
                          },
                        ],
                        onConfirm: async (data: { transaction_type: string }) => {
                          if (!id) return;
                          const { transaction_type } = data;
                          updateEntryMutation.mutateAsync({ id, transaction: { transaction_type } })
                        },
                        onCancel: (id: string) => modals.close(id),
                      },
                    })
                  }} >
                    <FontAwesomeIcon size="xs" icon={faEdit} />
                  </ActionIcon>
                </Tooltip>
              </Box>
            </Group>
          },
          {
            accessor: 'rank',
            title: "Rank",
            sortable: true,
            render: ({ id, rank }) => <Group grow position="apart" >
              <Text>{rank || 0}</Text>
              <Box w={25} display="flex" sx={{ justifyContent: "flex-end" }}>
                <Tooltip label={"Edit Rank"}>
                  <ActionIcon size={"sm"} color={"blue.7"} variant="filled" onClick={async (e) => {
                    e.stopPropagation();
                    modals.openContextModal({
                      modal: 'prompt',
                      title: "Rank",
                      innerProps: {
                        fields: [
                          {
                            name: 'rank',
                            label: "Rank",
                            value: rank,
                            type: 'number',
                          },
                        ],
                        onConfirm: async (data: { rank: number }) => {
                          if (!id) return;
                          const { rank } = data;
                          updateEntryMutation.mutateAsync({ id, transaction: { rank } })
                        },
                        onCancel: (id: string) => modals.close(id),
                      },
                    })
                  }} >
                    <FontAwesomeIcon size="xs" icon={faEdit} />
                  </ActionIcon>
                </Tooltip>
              </Box>
            </Group>
          },
          {
            accessor: 'quantity',
            title: "Quantity",
            render: ({ id, quantity }) => <Group grow position="apart" >
              <Text>{quantity || 0}</Text>
              <Box w={25} display="flex" sx={{ justifyContent: "flex-end" }}>
                <Tooltip label={"Edit Quantity"}>
                  <ActionIcon size={"sm"} color={"blue.7"} variant="filled" onClick={async (e) => {
                    e.stopPropagation();
                    modals.openContextModal({
                      modal: 'prompt',
                      title: "Quantity",
                      innerProps: {
                        fields: [
                          {
                            name: 'quantity',
                            label: "Quantity",
                            value: quantity,
                            type: 'number',
                          },
                        ],
                        onConfirm: async (data: { quantity: number }) => {
                          if (!id) return;
                          const { quantity } = data;
                          updateEntryMutation.mutateAsync({ id, transaction: { quantity } })
                        },
                        onCancel: (id: string) => modals.close(id),
                      },
                    })
                  }} >
                    <FontAwesomeIcon size="xs" icon={faEdit} />
                  </ActionIcon>
                </Tooltip>
              </Box>
            </Group>
          },
          {
            accessor: 'price',
            title: "Price",
            sortable: true,
            render: ({ id, price }) => <Group grow position="apart" >
              <Text>{price || 0}</Text>
              <Box w={25} display="flex" sx={{ justifyContent: "flex-end" }}>
                <Tooltip label={"Edit Price"}>
                  <ActionIcon size={"sm"} color={"blue.7"} variant="filled" onClick={async (e) => {
                    e.stopPropagation();
                    modals.openContextModal({
                      modal: 'prompt',
                      title: "Price",
                      innerProps: {
                        fields: [
                          {
                            name: 'price',
                            label: "Price",
                            value: price,
                            type: 'number',
                          },
                        ],
                        onConfirm: async (data: { price: number }) => {
                          if (!id) return;
                          const { price } = data;
                          updateEntryMutation.mutateAsync({ id, transaction: { price } })
                        },
                        onCancel: (id: string) => modals.close(id),
                      },
                    })
                  }} >
                    <FontAwesomeIcon size="xs" icon={faEdit} />
                  </ActionIcon>
                </Tooltip>
              </Box>
            </Group>
          },
          {
            accessor: 'created',
            title: "Date",
            sortable: true,
            render: ({ created }) => {
              return (
                <Text>{dayjs(created).format("DD.MM.YYYY HH:mm")}</Text>
              )
            }
          },
          {
            accessor: 'actions',
            width: 150,
            title: "Actions",
            render: ({ id, url, created, transaction_type, price }) =>
              <Group position="right" >
                <ActionIcon variant="filled" onClick={async () => {
                  // Set the text to be copied
                  await navigator.clipboard.writeText(`INSERT INTO transactions (name, datetime, transactionType, price) VALUES ('${url}', '${created}', '${transaction_type}', ${price})`);

                }} >
                  <FontAwesomeIcon icon={faCopy} />
                </ActionIcon>
                <ActionIcon color="red.7" variant="filled" onClick={async () => {
                  if (!id)
                    return;
                  // Set the text to be copied
                  deleteEntryMutation.mutate(id);
                }} >
                  <FontAwesomeIcon icon={faTrashCan} />
                </ActionIcon>
              </Group>
          },
        ]}
      />
    </Card>
  );
}
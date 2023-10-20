import { ActionIcon, Grid, Group, TextInput, Tooltip, Image } from "@mantine/core";
import { useCacheContext } from "@contexts/index";
import AvailableRivens from "./availableRivens";
import { useLocalStorage } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { DataTable, DataTableSortStatus } from "mantine-datatable";
import { useTranslatePage } from "@hooks/index";
import { modals } from "@mantine/modals";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCopy, faEdit, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { wfmThumbnail } from "@api/index";
import { useMutation } from "@tanstack/react-query";
import { generateWtbMessage } from "./helper";
import { Wfm } from "$types/index";
import { OffTauriEvent, OnTauriEvent, paginate, sortArray } from "../../../../../utils";
import { ButtonProgress } from "../../../../../components/buttonProgress";
import { SearchField } from "../../../../../components/searchfield";


export interface WTBEntry {
  url: string;
  name: string;
  icon: string;
  price: number;
  hidden?: boolean;
}
export default function WTBMessagePage() {
  const useTranslateWTBMessage = (key: string, context?: { [key: string]: any }) => useTranslatePage(`wtbMessage.${key}`, { ...context })
  const useTranslateDataGrid = (key: string, context?: { [key: string]: any }) => useTranslateWTBMessage(`datagrid.${key}`, { ...context })
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }) => useTranslateDataGrid(`columns.${key}`, { ...context });

  const { riven_items } = useCacheContext();
  const [wtbList, setWtbList] = useLocalStorage<WTBEntry[]>({ key: "wtbList", defaultValue: [] });
  const [wtbMessage, setWtbMessage] = useState<string>("");

  // States For DataGrid
  const [page, setPage] = useState(1);
  const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, setPageSize] = useState(pageSizes[4]);
  const [rows, setRows] = useState<WTBEntry[]>([]);
  const [totalRecords, setTotalRecords] = useState<number>(0);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus>({ columnAccessor: 'price', direction: 'desc' });
  const [query, setQuery] = useState<string>("");

  // Update DataGrid Rows
  useEffect(() => {
    if (!wtbList)
      return;
    let rivensFilter = wtbList;
    if (query !== "") {
      rivensFilter = rivensFilter.filter((riven) => riven.name.toLowerCase().includes(query.toLowerCase()));
    }

    setTotalRecords(rivensFilter.length);
    rivensFilter = sortArray([{
      field: sortStatus.columnAccessor,
      direction: sortStatus.direction
    }], rivensFilter);
    rivensFilter = paginate(rivensFilter, page, pageSize);
    setRows(rivensFilter);
  }, [wtbList, query, pageSize, page, sortStatus])

  const [progressState, setProgressState] = useState<{
    [key: string]: {
      total: number,
      current: number,
      message: string
    }
  }>({});
  useEffect(() => {
    // Group by price
    const groupByPrice: Record<number, WTBEntry[]> = {};
    wtbList.sort((a, b) => a.price - b.price).forEach((riven) => {
      if (!groupByPrice[riven.price])
        groupByPrice[riven.price] = [];
      groupByPrice[riven.price].push(riven);
    })

    // Create message
    // Sort by price
    const prices = Object.keys(groupByPrice).map((key) => parseInt(key)).sort((a, b) => b - a);

    const message = prices.map((key) => {
      const rivens = groupByPrice[key];
      const msg = rivens.map((riven) => `[${riven.name}]`).join("");
      return `${msg} ${key}p`;
    }).join(" ");
    setWtbMessage("WTB RIVENS FOR " + message);
  }, [wtbList])

  // Http Requests
  const generateWtbMessageMutation = useMutation((data: { rivenTypes: Wfm.RivenItemTypeDto[], minSellers: number, lowestPrice: number, discount: number }) => generateWtbMessage(data.rivenTypes, data.minSellers, data.lowestPrice, data.discount), {
    onSuccess: async (data) => {
      if (!data[0])
        return;
      setWtbList(data)
    },
    onError: () => { },
  })

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriEvent("GenerateWtbMessage:Progress", (data: {
      id: string, data: {
        total: number,
        current: number,
        message: string
      }
    }) => {
      const newProgressState = { ...progressState };
      newProgressState[data.id] = data.data;

      setProgressState(newProgressState);
    });
    return () => {
      OffTauriEvent("Cache:Update:Items");
    }
  }, []);


  return (
    <Grid>
      <Grid.Col md={6}>
        <AvailableRivens onAddRiven={(riven) => {
          if (wtbList.find((r) => r.url === riven.url_name))
            return;
          setWtbList([...wtbList, { name: riven.item_name, icon: riven.icon, url: riven.url_name, price: 0 }]);
        }} />
      </Grid.Col>
      <Grid.Col md={6}>
        <SearchField value={query} onChange={(text) => setQuery(text)} />
        <ButtonProgress
          onStart={async () => {
            generateWtbMessageMutation.mutate({
              rivenTypes: riven_items,
              minSellers: 15,
              lowestPrice: 50,
              discount: 0.5,
            });
          }}
          max={progressState["generate-wtb-message"]?.total == 0 ? 1 : progressState["generate-wtb-message"]?.total}
          current={progressState["generate-wtb-message"]?.current}
          label={("price_scraper_start")}
          progressLabel={progressState["generate-wtb-message"]?.message}
        />
        <DataTable
          sx={{ marginTop: "20px" }}
          height={"75vh"}
          striped
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
              accessor: 'name',
              title: useTranslateDataGridColumns("name"),
              sortable: true,
              render: ({ name, icon }) =>
                <Group >
                  <Image width={48} height={48} fit="contain" src={wfmThumbnail(icon || "")} />
                  {name}
                </Group>
            },
            {
              accessor: 'price',
              title: useTranslateDataGridColumns("bought_price"),
              sortable: true,
            },
            {
              accessor: 'actions',
              title: useTranslateDataGridColumns("actions.title"),
              width: 150,
              render: ({ url, price }) =>
                <Group>
                  <Tooltip label={useTranslateDataGridColumns("actions.edit")}>
                    <ActionIcon color="blue.9" variant="filled" size={"sm"} onClick={async () => {
                      modals.openContextModal({
                        modal: 'prompt',
                        title: useTranslateWTBMessage("prompt.sell_price.title"),
                        innerProps: {
                          fields: [{ name: 'price', description: useTranslateWTBMessage("prompt.sell_price.description"), label: useTranslateWTBMessage("prompt.sell_price.label"), type: 'number', value: price, placeholder: useTranslateWTBMessage("prompt.sell_price.placeholder") }],
                          onConfirm: async (data: { price: number }) => {

                            if (data.price <= 0)
                              return;
                            const newWtbList = [...wtbList];
                            const index = newWtbList.findIndex((r) => r.url === url);
                            if (index === -1)
                              return;
                            newWtbList[index].price = data.price;
                            setWtbList(newWtbList);

                          },
                          onCancel: (id: string) => modals.close(id),
                        },
                      })
                    }}>
                      <FontAwesomeIcon icon={faEdit} />
                    </ActionIcon>
                  </Tooltip>
                  <Tooltip label={useTranslateDataGridColumns("actions.delete")}>
                    <ActionIcon color="red.7" size={"sm"} variant="filled" onClick={async () => {
                      const newWtbList = [...wtbList];
                      const index = newWtbList.findIndex((r) => r.url === url);
                      if (index === -1)
                        return;
                      newWtbList.splice(index, 1);
                      setWtbList(newWtbList);
                    }}>
                      <FontAwesomeIcon size="1x" icon={faTrashCan} />
                    </ActionIcon>
                  </Tooltip>
                </Group>
            },
          ]}
        />
        <TextInput
          readOnly
          value={wtbMessage}
          error={wtbMessage.length > 300 ? useTranslateWTBMessage("wtb_message_max_length") : undefined}
          rightSection={
            <Group>
              <Tooltip label={useTranslateWTBMessage("copy_to_clipboard")}>
                <ActionIcon color="blue.7" size={"sm"} variant="filled" onClick={async () => {
                  await navigator.clipboard.writeText(wtbMessage);
                }}>
                  <FontAwesomeIcon size="1x" icon={faCopy} />
                </ActionIcon>
              </Tooltip>
            </Group>
          }
        />
      </Grid.Col>
    </Grid>
  );
}

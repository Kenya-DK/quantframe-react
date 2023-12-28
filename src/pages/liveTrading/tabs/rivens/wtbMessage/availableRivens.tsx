import { ActionIcon, Box, Button, Grid, Group, Image, Text, Stack, Title, Tooltip } from "@mantine/core";
import { useCacheContext } from "@contexts/index";
import { useTranslateComponent, useTranslateRustError } from "@hooks/index";
import { SearchField } from "@components/searchfield";
import { useEffect, useState } from "react";
import { DataTable, DataTableSortStatus } from "mantine-datatable";
import { Wfm, RustError } from "$types/index";
import { paginate, sortArray, SendNotificationToWindow } from "@utils/index";
import api, { wfmThumbnail } from "@api/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPlus } from "@fortawesome/free-solid-svg-icons";
import { useMutation } from "@tanstack/react-query";
import { TextColor } from "@components/textColor";
import { RivenAttributes } from "@components/auction/rivenAttributes";


interface WeaponInfo {
  wfm: {
    sellers: number;
    auction: Wfm.Auction<Wfm.AuctionOwner> | undefined;
  }
}
interface AvailableRivensProps {
  onAddRiven?: (riven: Wfm.RivenItemTypeDto) => void;
}
export default function AvailableRivens({ onAddRiven }: AvailableRivensProps) {
  const useTranslateAvailableRivens = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`availableRivens.${key}`, { ...context })
  const useTranslateDataGrid = (key: string, context?: { [key: string]: any }) => useTranslateAvailableRivens(`datagrid.${key}`, { ...context })
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }) => useTranslateDataGrid(`columns.${key}`, { ...context });

  const { riven_items } = useCacheContext();
  // States For DataGrid
  const [page, setPage] = useState(1);
  const pageSizes = [5, 10, 15, 20, 25, 30, 50, 100];
  const [pageSize, setPageSize] = useState(pageSizes[4]);
  const [rows, setRows] = useState<Wfm.RivenItemTypeDto[]>([]);
  const [totalRecords, setTotalRecords] = useState<number>(0);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus>({ columnAccessor: 'item_name', direction: 'desc' });
  const [query, setQuery] = useState<string>("");


  const [weaponInfo, setWeaponInfo] = useState<Record<string, WeaponInfo>>({});

  // Update DataGrid Rows
  useEffect(() => {
    if (!riven_items)
      return;
    let rivensFilter = riven_items;
    if (query !== "") {
      rivensFilter = rivensFilter.filter((riven) => riven.item_name.toLowerCase().includes(query.toLowerCase()));
    }

    setTotalRecords(rivensFilter.length);
    rivensFilter = sortArray([{
      field: sortStatus.columnAccessor,
      direction: sortStatus.direction
    }], rivensFilter);
    rivensFilter = paginate(rivensFilter, page, pageSize);
    setRows(rivensFilter);
  }, [riven_items, query, pageSize, page, sortStatus])

  // Http Requests
  const getWeaponAuctions = useMutation((data: { weapon: string }) => api.auction.search({ auction_type: "riven", weapon_url_name: data.weapon, buyout_policy: "direct", polarity: "any", sort_by: "price_asc" }), {
    onSuccess: async (data) => {
      if (!data[0])
        return;

      const filtered = data.filter(x => x.visible && x.closed == false && x.owner.status == "ingame" && x.is_direct_sell);

      const url = filtered[0].item.weapon_url_name;
      setWeaponInfo({
        ...weaponInfo,
        [url]: {
          wfm: {
            sellers: filtered.length,
            auction: filtered[0]
          }
        }
      })
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })

  return (
    <Box>
      <SearchField value={query} onChange={(text) => setQuery(text)} />
      <DataTable
        sx={{ marginTop: "20px" }}
        height={`calc(100vh - 163px)`}
        withBorder
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
        rowExpansion={{
          content: ({ record }) => {
            const weapon = weaponInfo[record.url_name];
            return (
              <Stack style={{ overflow: "hidden" }}>
                <Grid>
                  <Grid.Col span={6}>
                    <Stack>
                      <Title order={3} style={{ marginBottom: "10px" }} >Warframe Market</Title>
                      <Group>
                        <TextColor size={"md"} i18nKey="components.availableRivens.weaponInfo.wfm.sellers" values={{ sellers: weapon?.wfm.sellers || "N/A" }} />
                        <TextColor size={"md"} i18nKey="components.availableRivens.weaponInfo.wfm.username" values={{ username: weapon?.wfm.auction?.owner?.ingame_name || "N/A" }} />
                        <TextColor size={"md"} i18nKey="components.availableRivens.weaponInfo.wfm.lowestPrice" values={{ price: weapon?.wfm.auction?.buyout_price || "N/A" }} />
                      </Group>
                      <Group >
                        <TextColor i18nKey="components.auction.mastery_rank" values={{ mastery_rank: weapon?.wfm.auction?.item.mastery_level || "N/A" }} />
                        <TextColor i18nKey="components.auction.rank" values={{ rank: weapon?.wfm.auction?.item.mod_rank || 0 }} />
                        <TextColor i18nKey="components.auction.re_rolls" values={{ re_rolls: weapon?.wfm.auction?.item.re_rolls || 0 }} />
                        <TextColor i18nKey="components.auction.polarity" values={{ polarity: weapon?.wfm.auction?.item.polarity || "N/A" }} />
                      </Group>
                      <Text size={"md"}>
                        Attributes
                      </Text>
                      <RivenAttributes attributes={weapon?.wfm.auction?.item.attributes || []} />
                    </Stack>
                  </Grid.Col>
                </Grid>
                <Button loading={getWeaponAuctions.isLoading} onClick={() => getWeaponAuctions.mutate({ weapon: record.url_name })}>Get Info</Button>
              </Stack>
            )
          }
        }}
        // define columns
        columns={[
          {
            accessor: 'item_name',
            title: useTranslateDataGridColumns("name"),
            sortable: true,
            render: ({ item_name, icon }) =>
              <Group >
                <Image width={48} height={48} fit="contain" src={wfmThumbnail(icon || "")} />
                {item_name}
              </Group>
          },
          {
            accessor: 'riven_type',
            title: useTranslateDataGridColumns("riven_type"),
            sortable: true,
            render: ({ riven_type }) => riven_type?.charAt(0).toUpperCase() + riven_type.slice(1)
          },
          {
            accessor: 'group',
            title: useTranslateDataGridColumns("group"),
            sortable: true,
            render: ({ group }) => group.charAt(0).toUpperCase() + group.slice(1)
          },
          {
            accessor: 'mastery_level',
            title: useTranslateDataGridColumns("mastery_level"),
            sortable: true,
          },
          {
            accessor: 'actions',
            title: useTranslateDataGridColumns("actions.title"),
            width: 100,
            render: (row) =>
              <Group>
                <Tooltip label={useTranslateDataGridColumns("actions.add")}>
                  <ActionIcon color="green.9" size={"sm"} variant="filled" onClick={async (e) => {
                    e.stopPropagation();
                    onAddRiven && onAddRiven(row);
                  }}>
                    <FontAwesomeIcon size="1x" icon={faPlus} />
                  </ActionIcon>
                </Tooltip>
              </Group>
          },
        ]}
      />
    </Box>
  );
}

import { Box } from "@mantine/core";
interface ItemPanelProps {}
export const ItemPanel = ({}: ItemPanelProps) => {
  // const theme = useMantineTheme();
  // // States
  // const [item, setItem] = useState<SelectCacheTradableItem | undefined>(undefined);
  // const [value, setValue] = useState<[Date | null, Date | null]>([null, null]);
  // const [view, setView] = useState<"chart" | "table">("table");
  // // States For DataGrid
  // const [page, setPage] = useState(1);
  // const pageSizes = [1, 5, 10, 15, 20, 25, 30, 50, 100];
  // const [pageSize, setPageSize] = useState(pageSizes[4]);
  // const [rows, setRows] = useState<ItemPrice[]>([]);
  // const [totalRecords, setTotalRecords] = useState<number>(0);
  // // Translate general
  // const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslatePages(`prices.${key}`, { ...context }, i18Key);
  // const useTranslateTabSyndicate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslate(`tabs.item.${key}`, { ...context }, i18Key);
  // const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslateTabSyndicate(`datatable.columns.${key}`, { ...context }, i18Key);
  // const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslateTabSyndicate(`fields.${key}`, { ...context }, i18Key);
  // const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslateTabSyndicate(`errors.${key}`, { ...context }, i18Key);
  // const useTranslateCharts = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslateTabSyndicate(`charts.${key}`, { ...context }, i18Key);
  // // Fetch data from rust side
  // const { data, isFetching, error } = useQuery({
  //   queryKey: ["itemPrices", page, pageSize, value[0], value[1], item?.wfm_url_name, item?.sub_type, view],
  //   queryFn: () =>
  //     api.items.getItemsPrices(
  //       page,
  //       pageSize,
  //       value[0] as Date,
  //       value[1] as Date,
  //       "closed",
  //       item?.wfm_url_name,
  //       item?.sub_type,
  //       view == "chart" ? "chart" : undefined,
  //       "day"
  //     ),
  //   enabled: item != undefined && value[0] != null && value[1] != null,
  //   retry: 0,
  // });
  // // Update Database Rows
  // useEffect(() => {
  //   if (!data) return;
  //   setTotalRecords(data.total);
  //   setRows(data.results);
  // }, [data]);

  // const IsLessThanMaxDays = (maxDays: number) => {
  //   if (!value[0] || !value[1]) return undefined;
  //   const startDate = dayjs(value[0]);
  //   const endDate = dayjs(value[1]);
  //   if (!startDate.isValid() || !endDate.isValid()) return useTranslateErrors("invalid_date");
  //   if (startDate.isAfter(endDate)) return useTranslateErrors("invalid_date_range");
  //   if (endDate.diff(startDate, "day") >= maxDays) return useTranslateErrors("date_range_max_days", { days: maxDays });
  //   return undefined;
  // };

  // return (
  //   <Box pos="relative">
  //     <Box>
  //       <Group align="start">
  //         <SelectTradableItem
  //           decoration={useTranslateFields("item.description")}
  //           value={item?.wfm_url_name || ""}
  //           onChange={(item) => setItem(item)}
  //         />
  //         <DatePickerInput
  //           type="range"
  //           required
  //           label={useTranslateFields("date_range.label")}
  //           placeholder={useTranslateFields("date_range.placeholder")}
  //           description={useTranslateFields("date_range.description")}
  //           value={value}
  //           onChange={setValue}
  //           error={IsLessThanMaxDays(90)}
  //         />
  //         <Select
  //           required
  //           data={[
  //             { value: "table", label: useTranslateFields("view.options.table") },
  //             { value: "chart", label: useTranslateFields("view.options.chart") },
  //           ]}
  //           label={useTranslateFields("view.label")}
  //           description={useTranslateFields("view.description")}
  //           value={view}
  //           onChange={(value) => setView(value as "chart" | "table")}
  //         />
  //       </Group>
  //     </Box>
  //     <Collapse mt="md" in={!!error}>
  //       <pre>{JSON.stringify(error, null, 2)}</pre>
  //     </Collapse>
  //     {view == "chart" && (
  //       <Box className={`${classes.itemCardChart} ${useHasAlert() ? classes.alert : ""}`}>
  //         {isFetching && <Loading />}
  //         <BarCardChart
  //           title={useTranslateCharts("title")}
  //           showDatasetLabels
  //           labels={data?.include.labels || []}
  //           chartStyle={{ background: getGradient({ deg: 180, from: "grape.8", to: "grape.9" }, theme), height: "200px" }}
  //           datasets={[
  //             {
  //               label: useTranslateCharts("datasets.volume"),
  //               data: data?.include.volume_chart || [],
  //               backgroundColor: theme.colors.red[7],
  //             },
  //             {
  //               label: useTranslateCharts("datasets.min_price"),
  //               data: data?.include.min_price_chart || [],
  //               backgroundColor: theme.colors.blue[6],
  //             },
  //             {
  //               label: useTranslateCharts("datasets.max_price"),
  //               data: data?.include.max_price_chart || [],
  //               backgroundColor: theme.colors.pink[6],
  //             },
  //             {
  //               label: useTranslateCharts("datasets.avg_price"),
  //               data: data?.include.avg_price || [],
  //               backgroundColor: theme.colors.yellow[6],
  //             },
  //             {
  //               label: useTranslateCharts("datasets.median_price"),
  //               data: data?.include.median_price_chart || [],
  //               backgroundColor: theme.colors.violet[6],
  //             },
  //             {
  //               label: useTranslateCharts("datasets.moving_avg"),
  //               data: data?.include.moving_avg_chart || [],
  //               backgroundColor: theme.colors.cyan[6],
  //             },
  //           ]}
  //           context={<Paper h={"100%"}>WHat to put here</Paper>}
  //         />
  //       </Box>
  //     )}
  //     {view == "table" && (
  //       <DataTable
  //         className={`${classes.databaseItem} ${useHasAlert() ? classes.alert : ""}`}
  //         mt={"md"}
  //         records={rows}
  //         totalRecords={totalRecords}
  //         fetching={isFetching}
  //         page={page}
  //         recordsPerPage={pageSize}
  //         onPageChange={(p) => setPage(p)}
  //         recordsPerPageOptions={pageSizes}
  //         onRecordsPerPageChange={setPageSize}
  //         customLoader={<Loading />}
  //         // define columns
  //         columns={[
  //           {
  //             accessor: "name",
  //             title: useTranslateDataGridColumns("name.title"),
  //             sortable: true,
  //             render: (row) => (
  //               <TextTranslate
  //                 i18nKey={useTranslateDataGridColumns("name.value", undefined, true)}
  //                 values={{ name: item?.name || "", sub_type: GetSubTypeDisplay({ rank: row.mod_rank }) }}
  //               />
  //             ),
  //           },
  //           {
  //             accessor: "volume",
  //             title: useTranslateDataGridColumns("volume"),
  //             sortable: true,
  //           },
  //           {
  //             accessor: "min_price",
  //             title: useTranslateDataGridColumns("min_price"),
  //             sortable: true,
  //           },
  //           {
  //             accessor: "max_price",
  //             title: useTranslateDataGridColumns("max_price"),
  //             sortable: true,
  //           },
  //           {
  //             accessor: "avg_price",
  //             title: useTranslateDataGridColumns("avg_price"),
  //             sortable: true,
  //           },
  //           {
  //             accessor: "moving_avg",
  //             title: useTranslateDataGridColumns("moving_avg"),
  //             sortable: true,
  //           },
  //         ]}
  //       />
  //     )}
  //   </Box>
  // );
  return <Box>Item</Box>;
};

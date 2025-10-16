import { Box, Button } from "@mantine/core";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { DataTable } from "mantine-datatable";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { GetSubTypeDisplay, paginate } from "@utils/helper";
import { useMemo, useState } from "react";

export type SummaryAccordionProps = {
  value: TauriTypes.SettingsStockItem;
};

export const SummaryAccordion = ({ value }: SummaryAccordionProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_trading.item.summary.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`datatable_columns.${key}`, { ...context }, i18Key);
  const pageSizes = [5, 10, 20, 50, 100];
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(pageSizes[3]);

  // Fetch data from rust side
  const data = useQuery({
    queryKey: ["cache_items"],
    queryFn: () => api.cache.getTradableItems(),
  });

  const getInterestingWtbItems = useQuery({
    queryKey: ["get_interesting_wtb_items"],
    queryFn: () => api.live_scraper.get_interesting_wtb_items(value),
    retry: false,
    enabled: false,
  });
  const rows = useMemo(() => {
    return paginate(getInterestingWtbItems.data || [], page, pageSize);
  }, [getInterestingWtbItems.data, page, pageSize]);
  const GetItemName = (wfm_id: string) => {
    const item = data.data?.find((i) => i.wfm_id === wfm_id);
    return item ? item.name : wfm_id;
  };
  return (
    <Box h="100%">
      <Button
        mb={"md"}
        onClick={async () => {
          await getInterestingWtbItems.refetch();
          setPage(1);
        }}
      >
        {useTranslateForm("buttons.show_interesting_wtb_items")}
      </Button>
      <DataTable
        height={400}
        striped
        withTableBorder
        withColumnBorders
        idAccessor={"uuid"}
        records={rows}
        totalRecords={getInterestingWtbItems.data?.length || 0}
        page={page}
        recordsPerPage={pageSize}
        onPageChange={setPage}
        recordsPerPageOptions={pageSizes}
        onRecordsPerPageChange={setPageSize}
        columns={[
          {
            accessor: "name",
            title: useTranslateCommon("item_name.title"),
            render: ({ wfm_id, sub_type }) => (
              <TextTranslate
                color="gray.4"
                i18nKey={useTranslateCommon("item_name.value", undefined, true)}
                values={{
                  name: GetItemName(wfm_id),
                  sub_type: GetSubTypeDisplay(sub_type),
                }}
              />
            ),
          },
          { accessor: "volume", title: useTranslateCommon("datatable_columns.volume.title") },
          { accessor: "min_price", title: useTranslateCommon("datatable_columns.minimum_price.title") },
          { accessor: "profit", title: useTranslateDataGridColumns("profit") },
          { accessor: "trading_tax", title: useTranslateDataGridColumns("trading_tax") },
        ]}
      />
    </Box>
  );
};

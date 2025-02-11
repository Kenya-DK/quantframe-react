import { Box, MultiSelect } from "@mantine/core";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useEffect, useState } from "react";
import { ComplexFilter, Operator } from "@utils/filter.helper";
import { SyndicatesPrice } from "@api/types";
import dayjs from "dayjs";
import { GetSubTypeDisplay } from "@utils/helper";
import { DataTableSearch } from "@components/DataTableSearch";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { Loading } from "@components/Loading";
import { TextTranslate } from "@components/TextTranslate";
import classes from "../../Prices.module.css";
import { useHasAlert } from "@hooks/useHasAlert.hook";
interface SyndicatesPanelProps {}
export const SyndicatesPanel = ({}: SyndicatesPanelProps) => {
  // States For DataGrid
  const [query, setQuery] = useState<string>("");
  const [filters, setFilters] = useState<ComplexFilter>({});
  const [items, setItems] = useState<SyndicatesPrice[]>([]);
  const [syndicates, setSyndicates] = useState<string[]>([]);
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`prices.${key}`, { ...context }, i18Key);
  const useTranslateTabSyndicate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`tabs.syndicate.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabSyndicate(`datatable.columns.${key}`, { ...context }, i18Key);

  // Fetch data from rust side
  const { data, isFetching } = useQuery({
    queryKey: ["syndicatesPrices"],
    queryFn: () => api.items.getSyndicatesPrices(1, -1),
    retry: 0,
  });
  useEffect(() => {
    if (data) setItems(data.results);
  }, [data]);
  // Update Database Rows
  useEffect(() => {
    let filter: ComplexFilter = {
      OR: [],
      AND: [],
    };
    if (!items) return;

    if (syndicates.length > 0)
      filter.AND?.push(
        ...syndicates.map((syndicate) => ({
          syndicate_id: {
            [Operator.EQUALS]: syndicate,
          },
        }))
      );

    if (query != "")
      filter.OR?.push(
        ...[
          {
            name: {
              [Operator.CONTAINS_VALUE]: query,
              isCaseSensitive: false,
            },
          },
          {
            syndicate: {
              [Operator.CONTAINS_VALUE]: query,
              isCaseSensitive: false,
            },
          },
        ]
      );

    setFilters(filter);
  }, [items, query, syndicates]);
  return (
    <Box pos="relative">
      <DataTableSearch
        className={`${classes.databaseSyndicate} ${useHasAlert() ? classes.alert : ""}`}
        mt={"md"}
        records={items || []}
        onSearchChange={(text) => setQuery(text)}
        query={query}
        fetching={isFetching}
        filters={filters}
        customLoader={<Loading />}
        // define columns
        columns={[
          {
            accessor: "syndicate",
            title: useTranslateDataGridColumns("syndicate"),
            sortable: true,
            filter: (
              <MultiSelect
                label="Departments"
                description="Show all employees working at the selected departments"
                data={[
                  {
                    value: "arbiters_of_hexis",
                    label: "Arbiters of Hexis",
                  },
                  {
                    value: "necraloid",
                    label: "NecraLoid",
                  },
                  {
                    value: "new_loka",
                    label: "New Loka",
                  },
                  {
                    value: "the_hex",
                    label: "The Hex",
                  },
                  {
                    value: "kahl's_garrison",
                    label: "Kahl's Garrison",
                  },
                  {
                    value: "operational_supply",
                    label: "Operational Supply",
                  },
                  {
                    value: "cavia",
                    label: "Cavia",
                  },
                  {
                    value: "solaris_united",
                    label: "Solaris United",
                  },
                  {
                    value: "the_quills",
                    label: "The Quills",
                  },
                  {
                    value: "cephalon_simaris",
                    label: "Cephalon Simaris",
                  },
                  {
                    value: "entrati",
                    label: "Entrati",
                  },
                  {
                    value: "cephalon_suda",
                    label: "Cephalon Suda",
                  },
                  {
                    value: "the_perrin_sequence",
                    label: "The Perrin Sequence",
                  },
                  {
                    value: "the_holdfasts",
                    label: "The Holdfasts",
                  },
                  {
                    value: "ventkids",
                    label: "Ventkids",
                  },
                  {
                    value: "steel_meridian",
                    label: "Steel Meridian",
                  },
                  {
                    value: "red_veil",
                    label: "Red Veil",
                  },
                  {
                    value: "ostron",
                    label: "Ostron",
                  },
                  {
                    value: "vox_solaris",
                    label: "Vox Solaris",
                  },
                  {
                    value: "conclave",
                    label: "Conclave",
                  },
                ]}
                value={syndicates}
                placeholder="Search departments…"
                onChange={setSyndicates}
                comboboxProps={{ withinPortal: false }}
                clearable
                searchable
              />
            ),
            filtering: syndicates.length > 0,
          },
          {
            accessor: "name",
            title: useTranslateDataGridColumns("name.title"),
            sortable: true,
            render: (row) => (
              <TextTranslate
                i18nKey={useTranslateDataGridColumns("name.value", undefined, true)}
                values={{ name: row.name, sub_type: GetSubTypeDisplay(row.sub_type) }}
              />
            ),
          },
          {
            accessor: "min_price",
            title: useTranslateDataGridColumns("min_price"),
            sortable: true,
          },
          {
            accessor: "volume",
            title: useTranslateDataGridColumns("volume"),
            sortable: true,
          },
          {
            accessor: "max_price",
            title: useTranslateDataGridColumns("max_price"),
            sortable: true,
          },
          {
            accessor: "avg_price",
            title: useTranslateDataGridColumns("avg_price"),
            sortable: true,
          },
          {
            accessor: "standing",
            title: useTranslateDataGridColumns("standing"),
            sortable: true,
          },
          {
            accessor: "datetime",
            title: useTranslateDataGridColumns("datetime"),
            sortable: true,
            render: (row) => dayjs(row.datetime).format("YYYY-MM-DD HH:mm:ss"),
          },
        ]}
      />
    </Box>
  );
};

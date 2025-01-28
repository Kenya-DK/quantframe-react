import { Container, MultiSelect } from "@mantine/core";
import { useEffect, useState } from "react";
import api from "@api/index";
import { Loading } from "@components/Loading";
import { useQuery } from "@tanstack/react-query";
import { DataTableSearch } from "@components/DataTableSearch";
import { ComplexFilter, Operator } from "@utils/filter.helper";
import { SyndicatesPrice } from "@api/types";
import dayjs from "dayjs";
import { GetSubTypeDisplay } from "../../utils/helper";

export default function TestPage() {
  // States For DataGrid
  const [query, setQuery] = useState<string>("");
  const [syndicates, setSyndicates] = useState<string[]>([]);
  const [filters, setFilters] = useState<ComplexFilter>({});
  const [items, setItems] = useState<SyndicatesPrice[]>([]);
  // Fetch data from rust side
  const { data, isFetching } = useQuery({
    queryKey: ["syndicatesPrices"],
    queryFn: () => api.items.getSyndicatesPrices(1, -1),
  });

  // Update State
  useEffect(() => {
    console.log(data);
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
    <Container size={"100%"}>
      <DataTableSearch
        height={500}
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
            title: "Syndicate",
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
            title: "Item",
            sortable: true,
          },
          {
            accessor: "min_price",
            title: "Min Price",
            sortable: true,
          },
          {
            accessor: "volume",
            title: "Volume",
            sortable: true,
          },
          {
            accessor: "max_price",
            title: "Max Price",
            sortable: true,
          },
          {
            accessor: "avg_price",
            title: "Avg Price",
            sortable: true,
          },
          {
            accessor: "standing",
            title: "Standing",
            sortable: true,
          },
          {
            accessor: "datetime",
            title: "Datetime",
            sortable: true,
            render: (row) => dayjs(row.datetime).format("YYYY-MM-DD HH:mm:ss"),
          },
          {
            accessor: "sub_type",
            title: "Sub Type",
            sortable: true,
            render: (row) => GetSubTypeDisplay(row.sub_type),
          },
        ]}
      />
    </Container>
  );
}

import { Container, Group } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "../../api";
import { DataTableSearch } from "@components/DataTableSearch";
import { Loading } from "../../components/Loading";
import { useEffect, useState } from "react";
import { ISearchKeyParameter } from "$types/index";
import { ActionWithTooltip } from "../../components/ActionWithTooltip";
import { faEdit } from "@fortawesome/free-solid-svg-icons";

export default function TestPage() {

  const [query, setQuery] = useState<string>("");
  const [filters, setFilters] = useState<ISearchKeyParameter>({});

  useEffect(() => {
    let filter: ISearchKeyParameter = {};
    if (query)
      filter = {
        ...filter,
        name: {
          filters: [
            {
              value: query,
              operator: "like",
              isCaseSensitive: false,
            }
          ]
        }
      };
    setFilters(filter);
  }, [query]);

  const { data } = useQuery({
    queryKey: ['cache_items'],
    queryFn: () => api.cache.getTradableItems(),
  })
  return (
    <Container size={"100%"}>
      <DataTableSearch
        height={500}
        customLoader={<Loading />}
        query={query}
        onSearchChange={(text) => setQuery(text)}
        filters={filters}
        rightSectionWidth={145}
        filter={<Group gap={5}>
          <ActionWithTooltip
            tooltip={"Create New Item"}
            icon={faEdit}
            color={"green.7"}
            actionProps={{ size: "sm" }}
            iconProps={{ size: "xs" }}
            onClick={(e) => {
              e.stopPropagation();
            }}
          />
        </Group>
        }
        rightSection={
          <Group gap={5}>
            <ActionWithTooltip
              tooltip={"Create New Item"}
              icon={faEdit}
              color={"green.7"}
              actionProps={{ size: "sm" }}
              iconProps={{ size: "xs" }}
              onClick={(e) => {
                e.stopPropagation();
              }}
            />
          </Group>
        }
        columns={[
          {
            accessor: 'name',
          },
          {
            accessor: 'wfm_id',
          },
          {
            accessor: 'wfm_url_name',
          },
          {
            accessor: 'trade_tax',
            sortable: true,
          }
        ]}
        records={data || []}
      />
    </Container>
  );
}

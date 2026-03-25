import { Box, Center, Progress, ScrollArea, SimpleGrid, Stack, Text } from "@mantine/core";
import { useQueries } from "./queries";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import classes from "../../WFInventory.module.css";
import { useLocalStorage } from "@mantine/hooks";
import { TauriTypes } from "$types";
import { SearchField } from "@components/Forms/SearchField";
import { PreviewCard } from "@components/Shared/PreviewCard/PreviewCard";
import { Line } from "react-chartjs-2";

interface RivenPanelProps {
  isActive: boolean;
}

export const SyndicatePanel = ({ isActive }: RivenPanelProps) => {
  // States For DataGrid
  const [queryData, setQueryData] = useLocalStorage<TauriTypes.WFItemControllerGetListParams>({
    key: "syndicate_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 50 },
  });

  // Queries
  const { syndicatesQuery } = useQueries({ queryData, isActive });
  const data = [
    {
      date: "Mar 22",
      Oranges: 10,
    },
    {
      date: "Mar 23",
      Oranges: 20,
    },
    {
      date: "Mar 24",
      Oranges: 10,
    },
    {
      date: "Mar 25",
      Oranges: 50,
    },
    {
      date: "Mar 26",
      Oranges: 100,
    },
  ];
  return (
    <Box p={"md"}>
      <SearchField value={queryData.query || ""} onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))} />
      <Box h={32} w={150}>
        <Line
          height={32}
          width={150}
          options={{
            responsive: true,
            maintainAspectRatio: false,
            scales: {
              y: {
                display: false,
              },
              x: {
                display: false,
              },
            },
            plugins: {
              legend: {
                display: false,
              },
              title: {
                display: false,
                text: "Syndicate Standing Over Time",
              },
              tooltip: {
                enabled: false,
              },
            },
          }}
          data={{
            labels: data.map((d) => d.date),
            datasets: [
              {
                label: "Oranges",
                data: data.map((d) => d.Oranges),
                borderColor: "rgba(54, 162, 235, 1)",
                backgroundColor: "rgba(54, 162, 235, 0.2)",
              },
            ],
          }}
        />
      </Box>
      <ScrollArea mt={"md"} className={classes.veiledRivens} data-has-alert={useHasAlert()}>
        <SimpleGrid cols={{ base: 4 }} spacing="sm">
          {syndicatesQuery.data?.map((item, i) => (
            <PreviewCard
              h={100}
              key={i}
              pos={"relative"}
              value={item}
              headerLeft={<Text>R: {item.sub_type?.rank || "0"}</Text>}
              headerCenter={<Text>{item.name}</Text>}
              renderBody={() => (
                <Center h={"100%"}>
                  <Stack gap={0}>
                    <Text>{item.quantity}</Text>
                  </Stack>
                </Center>
              )}
              footerCenter={
                <Box w={"100%"}>
                  <Progress
                    h={20}
                    value={
                      ((item.properties.total - item.properties.min_standing) / (item.properties.max_standing - item.properties.min_standing)) * 100
                    }
                  />
                </Box>
              }
            />
          ))}
        </SimpleGrid>
      </ScrollArea>
    </Box>
  );
};

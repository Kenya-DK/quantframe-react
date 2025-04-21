import { Box, Container, getGradient, Select, useMantineTheme } from "@mantine/core";

import { useAppContext } from "@contexts/app.context";
import { BarCardChart } from "../../components/BarCardChart";
import { useState } from "react";
const data = {
  most_traded: {
    labels: [
      "Arcane Hot Shot (R 0)",
      "Ayatan Anasa Sculpture A 2",
      "Rifle Riven Mod (Veiled) (unrevealed)",
      "Melee Riven Mod (Veiled) (unrevealed)",
      "Melee Influence (R 0)",
      "Arcane Universal Fallout (R 0)",
      "Molt Augmented (R 0)",
      "Pistol Riven Mod (Veiled) (unrevealed)",
      "Wisp Prime Set",
      "Arcane Energize (R 0)",
    ],
    values: [
      729.6666666666666, 664.5333333333333, 423.73333333333335, 377.26666666666665, 359.26666666666665, 348.93333333333334, 344.06666666666666,
      331.26666666666665, 319.06666666666666, 305.2,
    ],
    avg: 420.2999999999999,
  },
  profit_margin: {
    labels: [
      "Primed Shotgun Ammo Mutation (R 10)",
      "Arcane Pistoleer (R 5)",
      "Arcane Backdraft Helmet",
      "Deimos Catacombs Scene",
      "Braton Vandal Set",
      "Saxum Spittle (R 5)",
      "Arcane Hot Shot (R 5)",
      "Axi P3 Relic (intact)",
      "Chamber Of The Lotus Scene",
      "Hammer Shot (R 3)",
    ],
    values: [55, 52, 50, 50, 40, 36, 35, 30, 30, 25],
    avg: 40.3,
  },
  return_on_investment: {
    labels: [
      "Axi P3 Relic (intact)",
      "Furor (R 0)",
      "Blood For Ammo",
      "Sicarus Prime Blueprint",
      "Kinetic Diversion (R 3)",
      "Neo K6 Relic (intact)",
      "Ambassador Receiver (blueprint)",
      "Arcane Camisado (R 0)",
      "Axi G1 Relic (intact)",
      "Bronco Prime Blueprint",
    ],
    values: [600, 500, 400, 400, 300, 300, 200, 200, 200, 200],
    avg: 330,
  },
  supply_and_demand: {
    labels: [
      "Ayatan Cyan Star",
      "Faceted Tiametrite",
      "Purged Dagonic",
      "Ammo Drum (R 0)",
      "Radian Sentirum",
      "Stellated Necrathene",
      "Star Crimzian",
      "Purified Heciphron",
      "Heart Nyth",
      "Marquise Veridos",
      "Ghoulsaw Set (blueprint)",
      "Pistol Amp (R 0)",
      "Redirection (R 0)",
      "Heart Noctrul",
      "Tear Azurite",
      "Smooth Phasmin",
      "Goblite Tears",
      "Marquise Thyst",
      "Ayatan Anasa Sculpture A 2",
      "Intruder",
    ],
    supply: [
      39726.6, 30328.333333333332, 23804.733333333334, 22300, 21172.866666666665, 19968.6, 19178.6, 18449.6, 17111.466666666667, 16659.133333333335,
      15660.733333333334, 15298.6, 15130.8, 14401.2, 13920.4, 13853.866666666667, 13682.6, 13309.866666666667, 13302.066666666668, 13281.4,
    ],
    demand: [0.06666666666666667, 0, 0, 0, 46, 2.7333333333333334, 10.933333333333334, 9, 33.733333333333334, 0, 0, 6, 0, 0, 0, 0, 2, 17, 61927.6, 0],
  },
};
export default function TestPage() {
  const [chartData, setChartData] = useState<{
    labels: string[];
    datasets: any[];
  }>({
    labels: [],
    datasets: [],
  });
  const theme = useMantineTheme();
  useAppContext();
  return (
    <Container size={"100%"}>
      <BarCardChart
        horizontal
        title={"Most Traded"}
        labels={chartData.labels || []}
        chartStyle={{ background: getGradient({ deg: 180, from: theme.colors.gray[8], to: theme.colors.gray[9] }, theme), height: "300px" }}
        datasets={chartData.datasets || []}
        context={
          <Box>
            <Select
              data={[
                { value: "most_traded", label: "Most Traded" },
                { value: "profit_margin", label: "Profit Margin" },
                { value: "return_on_investment", label: "Return on Investment" },
              ]}
              onChange={(e) => {
                console.log(e);
                const chartData = data[e as keyof typeof data] as any;
                data[e as keyof typeof data].labels;
                setChartData({
                  labels: chartData.labels,
                  datasets: [
                    {
                      label: "today.bar_chart.datasets.profit",
                      data: chartData.values,
                      backgroundColor: "rgba(255, 99, 132, 0.5)",
                    },
                  ],
                });
              }}
              placeholder={"Select a chart"}
              label={"Select a chart"}
            />
          </Box>
        }
      />
      <Box w={"100%"}>
        <BarCardChart
          title={"Supply And Demand"}
          labels={data["supply_and_demand"].labels || []}
          chartStyle={{ background: getGradient({ deg: 180, from: theme.colors.gray[8], to: theme.colors.gray[9] }, theme), height: "300px" }}
          datasets={[
            {
              label: "Supply",
              data: data["supply_and_demand"].supply,
              backgroundColor: "rgba(255, 99, 132, 0.5)",
            },
            {
              label: "Demand",
              data: data["supply_and_demand"].demand,
              backgroundColor: "rgba(255, 0, 0, 0.5)",
            },
          ]}
        />
      </Box>
    </Container>
  );
}

import { Box, Paper, Title, useMantineTheme } from "@mantine/core";
import classes from "./BarCardChart.module.css";

import { Chart as ChartJS, CategoryScale, LinearScale, BarElement, Tooltip, Legend, TooltipCallbacks } from "chart.js";
ChartJS.register(CategoryScale, LinearScale, BarElement, Tooltip, Legend);

import { Bar } from "react-chartjs-2";
import GetDefaultOptions from "./default.options";
import { useMemo } from "react";
import GetDefaultDatasetStyle from "./default.dataSetStyle";

// Type definitions
export type ClickColumn = {
  datasetIndex: number;
  datasetLabel: string;
  index: number;
  column: string;
  value: number;
};

export type BarCardChartProps = {
  title: string;
  labels: string[];
  showDatasetLabels?: boolean;
  horizontal?: boolean;
  chartStyle?: React.CSSProperties;
  context?: React.ReactNode;
  onColumnClick?: (event: ClickColumn) => void;
  tooltipShowColor?: boolean;
  boxWidth?: number;
  boxHeight?: number;
  tooltipCallback?: Partial<TooltipCallbacks<"bar">>;
  datasets: {
    label?: string;
    data: number[];
    backgroundColor?: string;
  }[];
};

export function BarCardChart({
  labels,
  title,
  context,
  chartStyle,
  tooltipShowColor,
  boxWidth,
  boxHeight,
  tooltipCallback,
  showDatasetLabels,
  datasets,
  horizontal,
}: BarCardChartProps) {
  const theme = useMantineTheme();
  const cData = {
    labels: labels,
    datasets: datasets.map((dataset) => ({
      ...GetDefaultDatasetStyle(theme.colors.gray[0]),
      ...dataset,
    })),
  };
  return (
    <Paper className={classes.root}>
      <Box className={classes.chartContainer}>
        {useMemo(
          () => (
            <Bar
              style={{ ...chartStyle }}
              className={classes.chartCanvas}
              options={
                GetDefaultOptions(theme.colors.gray[0], showDatasetLabels, tooltipShowColor, boxWidth, boxHeight, tooltipCallback, horizontal) as any
              }
              data={cData}
            />
          ),
          [datasets]
        )}
      </Box>
      <Title mt={5} order={4}>
        {title}
      </Title>

      {context && <Box pt={1}>{context}</Box>}
    </Paper>
  );
}

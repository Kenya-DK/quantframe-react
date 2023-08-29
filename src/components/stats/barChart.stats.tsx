import { Box, Paper, useMantineTheme, Title } from '@mantine/core';
import type { InteractionItem, TooltipCallbacks } from 'chart.js';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Tooltip,
  Legend,

} from 'chart.js';
import { Bar, getDatasetAtEvent, getElementAtEvent } from "react-chartjs-2";
import React, { useMemo, useRef, MouseEvent } from 'react';


ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Tooltip,
  Legend
);

// Type definitions
type ClickColumn = {
  datasetIndex: number;
  datasetLabel: string;
  index: number;
  column: string;
  value: number;
};

interface BarChartProps {
  title: string;
  chartType?: "bar" | "line";
  labels: string[],
  showDatasetLabels?: boolean;
  // color: string;
  // icon: React.ReactNode;
  // description: string | React.ReactNode;
  // onClick?: () => void;
  // date: | React.ReactNode;
  chartStyle?: React.CSSProperties;
  context?: React.ReactNode;
  onColumnClick?: (event: ClickColumn) => void;
  tooltipShowColor?: boolean;
  boxWidth?: number;
  boxHeight?: number;
  precision?: number;
  tooltipCallback?: Partial<TooltipCallbacks<"bar">>;
  datasets: {
    label: string,
    data: number[],
    backgroundColor?: string;
  }[],
}


export const BarChart = ({ tooltipShowColor, chartType, boxWidth, boxHeight, tooltipCallback, onColumnClick, context, showDatasetLabels, chartStyle, labels, title, datasets }: BarChartProps) => {
  const theme = useMantineTheme();
  const chartRef = useRef(null);

  const frontColor = theme.colors.gray[0];

  const onClick = (event: MouseEvent<HTMLCanvasElement>) => {
    const { current: chart } = chartRef;

    if (!chart) {
      return;
    }
    const dataset: InteractionItem[] = getDatasetAtEvent(chart, event);
    if (!dataset.length) return;

    const clickLabel: InteractionItem[] = getElementAtEvent(chart, event);
    if (!clickLabel.length) return;
    const { datasetIndex, index } = clickLabel[0];

    const cEvent: ClickColumn = {
      datasetLabel: datasets[datasetIndex].label,
      datasetIndex,
      index,
      column: labels[index],
      value: datasets[datasetIndex].data[index],
    }
    onColumnClick?.(cEvent);
  };
  const options = {
    responsive: true,
    maintainAspectRatio: false,
    devicePixelRatio: 4,
    scale: {
      ticks: {
        precision: 0
      }
    },
    plugins: {
      legend: {
        labels: {
          color: frontColor
        },
        display: (showDatasetLabels == undefined) ? false : showDatasetLabels,
      },
      tooltip: {
        displayColors: tooltipShowColor == undefined ? false : tooltipShowColor,
        boxWidth: boxWidth,
        boxHeight: boxHeight,

        callbacks: {
          ...tooltipCallback,
        }
      }
    },
    scales: {
      x: {
        grid: {
          display: true,
          drawBorder: false,
          drawOnChartArea: true,
          drawTicks: false,
          borderDash: [5, 5],
          color: "rgba(255, 255, 255, .2)",
        },
        ticks: {
          display: true,
          color: frontColor,
          padding: 10
        },
      },
      y: {
        grid: {
          drawBorder: false,
          display: true,
          drawOnChartArea: true,
          drawTicks: false,
          borderDash: [5, 5],
          color: "rgba(255, 255, 255, .2)",
        },
        ticks: {
          suggestedMin: 0,
          suggestedMax: 500,
          beginAtZero: true,
          padding: 10,
          color: frontColor,
        },
      }
    },
  };

  const defaultDatasetsStyle = {
    color: frontColor,
    backgroundColor: "rgba(255, 255, 255, 0.8)",
    label: "Default",
    tension: 0.4,
    borderWidth: 0,
    borderRadius: 4,
    data: [],
    maxBarThickness: 6,
    pointRadius: 0
  }

  const cData = {
    labels: labels,
    datasets: datasets.map((dataset) => ({
      ...defaultDatasetsStyle,
      ...dataset,
    })),
  };

  return (
    <Box sx={{ paddingTop: "24px" }}>
      <Paper sx={{ padding: "1rem", }} >
        {useMemo(
          () => (
            <Box
              h="12.5rem"
              py={2}
              pr={0.5}
              mt={-50}
              sx={{ borderRadius: "0.5rem", }}
              style={{ ...chartStyle }}
            >
              {(chartType == "bar" || chartType == undefined) && <Bar ref={chartRef} onClick={onClick} data={cData} options={options} />}
              {/* {chartType == "line" && <Line ref={chartRef} onClick={onClick} data={cData} options={options} />} */}
            </Box>
          ),
          [datasets]
        )}
        <Title mt={5} order={4}>{title}</Title>

        {context &&
          <Box pt={1}>
            {context}
          </Box>
        }
      </Paper>
    </Box>
  );
}

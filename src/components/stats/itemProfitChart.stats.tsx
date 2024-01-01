import type { TooltipItem } from 'chart.js';
import { BarChart } from './barChart.stats';
import { ChartMultipleDto } from '$types/index';
import { useEffect, useState } from 'react';
import { Box, Group, Select } from '@mantine/core';
import { useTranslateComponent } from '../../hooks';
import { TextColor } from '../textColor';

interface DataSet extends ChartMultipleDto {
  label?: string,
  negativeBackgroundColor?: string;
  backgroundColor?: string;
}
interface ItemProfitChartProps {
  type: "revenue" | "quantity" | "shipping" | "average";
  title: string;
  background: string;
  precision?: number;
  labels: string[],
  // context?: React.ReactNode;
  orderWithRevenues: DataSet[],
  showDatasetLabels?: boolean;
}

export const ItemProfitChart = ({ showDatasetLabels, title, precision, background, orderWithRevenues, labels }: ItemProfitChartProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`transactionRevenueChart.${key}`, { ...context }, i18Key)
  const useTranslateContext = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateSearch(`context.${key}`, { ...context }, i18Key)
  const [datasets, setDatasets] = useState<{ label: string, data: number[], backgroundColor?: any; }[]>([]);
  const modes = [
    { value: "0", label: useTranslateSearch("modes.sales.title",) },
    { value: "1", label: useTranslateSearch("modes.purchases.title") },
    { value: "2", label: useTranslateSearch("modes.quantity.title") },
    { value: "3", label: useTranslateSearch("modes.profit.title") },
  ]
  const [mode, setMode] = useState<string>("3");
  useEffect(() => {
    if (!orderWithRevenues) return;
    setDatasets(orderWithRevenues.map((orderWithRevenue) => {
      let chart_values = orderWithRevenue?.values[Number(mode)];
      return {
        label: orderWithRevenue?.label || "",
        data: chart_values,
        // backgroundColor: orderWithRevenue?.backgroundColor || "",
        backgroundColor: (ctx2: any) => orderWithRevenue?.negativeBackgroundColor ? ctx2.parsed.y > 0 ? orderWithRevenue?.backgroundColor : orderWithRevenue.negativeBackgroundColor : orderWithRevenue?.backgroundColor || "",
      }
    }));
  }, [orderWithRevenues, mode]);


  const GetTotalByMode = (mode: 0) => {
    let total = 0;
    orderWithRevenues.forEach((orderWithRevenue) => {
      let chart_values = orderWithRevenue?.values[mode];
      chart_values.forEach((value) => {
        total += value;
      })
    })
    return total;
  }

  return (
    <BarChart
      title={title}
      labels={labels}
      showDatasetLabels={showDatasetLabels}
      chartStyle={{ background: background, }}
      datasets={datasets}
      precision={precision}
      tooltipCallback={{
        title: function (items: TooltipItem<"bar">[]) {
          const item = items[0];
          if (!item) return "";
          if (item.dataset.label)
            return `${item.dataset.label} - ${item.label}`;
          else
            return item.label;
        },
        label: function () {
          return ""
        },
        afterLabel: function (context: TooltipItem<"bar">) {
          const labels: string[] = [];
          const dataSet = orderWithRevenues[context.datasetIndex];
          for (let index = 0; index < modes.length; index++) {
            const element = modes[index];
            const vaule = dataSet?.values[index][context.dataIndex];
            labels.push(`${element.label}: ${vaule}`);
          }
          return labels.join('\n');
        },
      }}
      context={
        <>
          <Group grow position="apart" sx={{ position: "relative" }}>
            <Box>
              <TextColor i18nKey={useTranslateContext("profit", undefined, true)} values={{ val: GetTotalByMode(0) || 0 }} />
              <TextColor i18nKey={useTranslateContext("profit_margin", undefined, true)} values={{ val: ((GetTotalByMode(0)) * 100).toPrecision(2) }} />
            </Box>
            <Group position='right'>
              <Select
                maw={150}
                data={modes}
                placeholder="Select type"
                value={mode}
                onChange={(value) => {
                  if (!value) return;
                  setMode(value)
                }}
              />
            </Group>
          </Group>
          <Group >
            <TextColor i18nKey={useTranslateContext("footer", undefined, true)} values={{ sales: GetTotalByMode(0), purchases: GetTotalByMode(0), trades: GetTotalByMode(0) }} />
          </Group>
        </>
      }
    />
  );
}


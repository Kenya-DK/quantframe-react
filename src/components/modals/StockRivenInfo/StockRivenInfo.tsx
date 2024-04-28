import { Group, TextInput, Grid, Title } from '@mantine/core';
import { StockRiven } from '@api/types';
import { useTranslateComponent, useTranslateEnums } from '@hooks/index';
import dayjs from 'dayjs';
import { PriceHistoryListItem } from '@components';


export type StockRivenInfoProps = {
  value: StockRiven;
}
export function StockRivenInfo({ value }: StockRivenInfoProps) {


  // Translate general
  const useTranslateStockRivenInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`stock_riven_info.${key}`, { ...context }, i18Key)
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateStockRivenInfo(`fields.${key}`, { ...context }, i18Key)
  // const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateStockItemInfo(`buttons.${key}`, { ...context }, i18Key)
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key)

  return (
    <Grid>
      <Grid.Col span={6}>
        <Group grow>
          <TextInput label={useTranslateFields("created_at")} value={dayjs(value.created_at).format('DD/MM/YYYY HH:mm:ss')} readOnly />
          <TextInput label={useTranslateFields("updated_at")} value={dayjs(value.updated_at).format('DD/MM/YYYY HH:mm:ss')} readOnly />
        </Group>
        <Group grow>
          <TextInput label={useTranslateFields("status")} value={useTranslateStockStatus(value.status)} readOnly />
          <TextInput label={useTranslateFields("bought")} value={value.bought} readOnly />
        </Group>
        <Group grow>
          <TextInput label={useTranslateFields("minimum_price")} value={value.minimum_price || "N/A"} readOnly />
        </Group>
      </Grid.Col>
      <Grid.Col span={6}>
        <Title order={3}>{useTranslateFields("listed")}</Title>
        {value.price_history.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()).slice(0, 5).map((price, index) => (
          <PriceHistoryListItem key={index} history={price} />
        ))}
      </Grid.Col>
    </Grid>
  );
}
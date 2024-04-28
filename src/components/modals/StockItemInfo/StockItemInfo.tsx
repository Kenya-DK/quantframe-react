import { Group, TextInput, Button, Grid, Title } from '@mantine/core';
import { StockItem } from '@api/types';
import { useTranslateComponent, useTranslateEnums } from '@hooks/index';
import dayjs from 'dayjs';
import { PriceHistoryListItem } from '@components';
import { useQuery } from '@tanstack/react-query';
import api from '@api/index';
import { useEffect, useState } from 'react';


export type StockItemInfoProps = {
  value: StockItem;
}
export function StockItemInfo({ value }: StockItemInfoProps) {

  //State
  const [wikiUrl, setWikiUrl] = useState<string | null>(null);


  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ['cache_items'],
    queryFn: () => api.cache.getTradableItems(),
  })

  useEffect(() => {
    if (!data) {
      setWikiUrl(null);
      return;
    }
    const item = data.find((x) => x.wfm_url_name === value.wfm_url);
    if (!item)
      setWikiUrl(null);
    else
      setWikiUrl(item.wiki_url);
  }, [data, value])



  // Translate general
  const useTranslateStockItemInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`stock_item_info.${key}`, { ...context }, i18Key)
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateStockItemInfo(`fields.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateStockItemInfo(`buttons.${key}`, { ...context }, i18Key)
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
          <TextInput label={useTranslateFields("owned")} value={value.owned} readOnly />
        </Group>
        <Group mt={"md"} grow>
          <Button color="blue" variant="outline" onClick={() => { window.open(`https://warframe.market/items/${value.wfm_url}`, '_blank') }}>{useTranslateButtons("wfm")}</Button>
          {wikiUrl && <Button color="blue" variant="outline" onClick={() => { window.open(wikiUrl, '_blank') }}>{useTranslateButtons("wiki")}</Button>}
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
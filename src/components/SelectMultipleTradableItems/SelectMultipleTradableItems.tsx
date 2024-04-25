import { Grid, Title } from '@mantine/core';
import { TradableItemList } from '../TradableItemList';
import { CacheTradableItem } from '@api/types';
import { useQuery } from '@tanstack/react-query';
import api from '@api/index';
import { useEffect, useState } from 'react';

export type SelectMultipleTradableItemsProps = {
  selectedItems: string[];
  leftTitle?: string;
  rightTitle?: string;
  height?: number;
  onChange(items: string[]): void;
}
export function SelectMultipleTradableItems({ onChange, selectedItems, leftTitle, rightTitle }: SelectMultipleTradableItemsProps) {
  const [leftItems, setLeftItems] = useState<CacheTradableItem[]>([]);
  const [rightItems, setRightItems] = useState<CacheTradableItem[]>([]);


  const { data } = useQuery({
    queryKey: ['cache_items'],
    queryFn: () => api.cache.getTradableItems(),
  })

  useEffect(() => {
    if (!data) return;
    // Map selected items to the actual items
    const selectedItemsData = data.filter((item) => selectedItems.includes(item.wfm_url_name));
    const dataWithoutSelected = data.filter((item) => !selectedItems.includes(item.wfm_url_name));
    setRightItems(selectedItemsData);
    setLeftItems(dataWithoutSelected);
  }, [selectedItems, data])

  const OnAddItem = (item: CacheTradableItem) => {
    const rItems = [...rightItems, item];
    setRightItems(rItems);
    setLeftItems(leftItems.filter((i) => i.wfm_url_name !== item.wfm_url_name));
    onChange(rItems.map((i) => i.wfm_url_name));
  }

  const OnAddAll = (items: CacheTradableItem[]) => {
    const rItems = [...rightItems, ...items];
    setRightItems(rItems);
    onChange(rItems.map((i) => i.wfm_url_name));
    setLeftItems(leftItems.filter((i) => !items.map((i) => i.wfm_url_name).includes(i.wfm_url_name)));
  }


  const OnRemoveItem = (item: CacheTradableItem) => {
    setLeftItems([...leftItems, item]);
    const rItems = rightItems.filter((i) => i.wfm_url_name !== item.wfm_url_name);
    setRightItems(rItems);
    onChange(rItems.map((i) => i.wfm_url_name));
  }

  const OnRemoveAll = (items: CacheTradableItem[]) => {
    setLeftItems([...leftItems, ...items]);
    const rItems = rightItems.filter((i) => !items.map((i) => i.wfm_url_name).includes(i.wfm_url_name));
    setRightItems(rItems);
    onChange(rItems.map((i) => i.wfm_url_name));
  }

  return (
    <Grid>
      <Grid.Col span={6}>
        <Title order={4}>{leftTitle}</Title>
        <TradableItemList onAddAll={OnAddAll} onAddItem={OnAddItem} availableItems={leftItems || []} />
      </Grid.Col>
      <Grid.Col span={6}>
        <Title order={4}>{rightTitle}</Title>
        <TradableItemList onAddAll={OnRemoveAll} onAddItem={OnRemoveItem} availableItems={rightItems || []} />
      </Grid.Col>
    </Grid>
  );
}
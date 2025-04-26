import { Grid, Title } from "@mantine/core";
import { TradableItemList } from "@components/TradableItemList";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect, useState } from "react";

export type SelectMultipleTradableItemsProps = {
  selectedItems: string[];
  leftTitle?: string;
  rightTitle?: string;
  height?: number;
  onChange(items: string[]): void;
};
export function SelectMultipleTradableItems({ onChange, selectedItems, leftTitle, rightTitle }: SelectMultipleTradableItemsProps) {
  const [leftItems, setLeftItems] = useState<TauriTypes.CacheTradableItem[]>([]);
  const [rightItems, setRightItems] = useState<TauriTypes.CacheTradableItem[]>([]);

  const { data } = useQuery({
    queryKey: ["cache_items"],
    queryFn: () => api.cache.getTradableItems(),
  });

  useEffect(() => {
    if (!data) return;
    // Map selected items to the actual items
    const selectedItemsData = data.filter((item) => selectedItems.includes(item.wfm_url_name));
    const dataWithoutSelected = data.filter((item) => !selectedItems.includes(item.wfm_url_name));
    setRightItems(selectedItemsData);
    setLeftItems(dataWithoutSelected);
  }, [selectedItems, data]);

  const OnAddItem = (item: TauriTypes.CacheTradableItem) => {
    const rItems = [...rightItems, item];
    setRightItems(rItems);
    setLeftItems(leftItems.filter((i) => i.wfm_url_name !== item.wfm_url_name));
    onChange(rItems.map((i) => i.wfm_url_name));
  };

  const OnAddAll = (items: TauriTypes.CacheTradableItem[]) => {
    const rItems = [...rightItems, ...items];
    setRightItems(rItems);
    onChange(rItems.map((i) => i.wfm_url_name));
    setLeftItems(leftItems.filter((i) => !items.map((i) => i.wfm_url_name).includes(i.wfm_url_name)));
  };

  const OnRemoveItem = (item: TauriTypes.CacheTradableItem) => {
    setLeftItems([...leftItems, item]);
    const rItems = rightItems.filter((i) => i.wfm_url_name !== item.wfm_url_name);
    setRightItems(rItems);
    onChange(rItems.map((i) => i.wfm_url_name));
  };

  const OnRemoveAll = (items: TauriTypes.CacheTradableItem[]) => {
    setLeftItems([...leftItems, ...items]);
    const rItems = rightItems.filter((i) => !items.map((i) => i.wfm_url_name).includes(i.wfm_url_name));
    setRightItems(rItems);
    onChange(rItems.map((i) => i.wfm_url_name));
  };

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

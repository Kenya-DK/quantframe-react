import { Grid, Title } from "@mantine/core";
import { GenericItemList, GenericItemListProps } from "@components/Forms/GenericItemList";
import { useEffect, useState } from "react";

export type SelectMultipleItemsProps<T> = {
  selectedItems: T[];
  leftTitle?: string;
  rightTitle?: string;
  leftConfig: Omit<GenericItemListProps<T>, "items"> & { items: T[] };
  rightConfig: Omit<GenericItemListProps<T>, "items"> & { items: T[] };
  onChange(items: T[]): void;
  getId: (item: T) => string | number;

  // Hooks before actions
  onBeforeAdd?: (item: T) => boolean | Promise<boolean>;
  onBeforeAddAll?: (items: T[]) => boolean | Promise<boolean>;
  onBeforeRemove?: (item: T) => boolean | Promise<boolean>;
  onBeforeRemoveAll?: (items: T[]) => boolean | Promise<boolean>;
};

export function SelectMultipleItems<T>({
  onChange,
  selectedItems,
  leftTitle,
  rightTitle,
  leftConfig,
  rightConfig,
  getId,
  onBeforeAdd,
  onBeforeAddAll,
  onBeforeRemove,
  onBeforeRemoveAll,
}: SelectMultipleItemsProps<T>) {
  const [leftItems, setLeftItems] = useState<T[]>([]);
  const [rightItems, setRightItems] = useState<T[]>([]);

  useEffect(() => {
    const allItems = leftConfig.items;
    const rightSet = new Set(selectedItems.map(getId));
    const right = allItems.filter((i) => rightSet.has(getId(i)));
    const left = allItems.filter((i) => !rightSet.has(getId(i)));

    setRightItems(right);
    setLeftItems(left);
  }, [selectedItems, leftConfig.items]);

  const updateRight = (items: T[]) => {
    setRightItems(items);
    onChange(items);
  };

  const OnAddItem = async (item: T) => {
    if (onBeforeAdd && !(await onBeforeAdd(item))) return;
    updateRight([...rightItems, item]);
    setLeftItems(leftItems.filter((i) => getId(i) !== getId(item)));
  };

  const OnAddAll = async (items: T[]) => {
    if (onBeforeAddAll && !(await onBeforeAddAll(items))) return;
    updateRight([...rightItems, ...items]);
    setLeftItems(leftItems.filter((i) => !items.some((j) => getId(j) === getId(i))));
  };

  const OnRemoveItem = async (item: T) => {
    if (onBeforeRemove && !(await onBeforeRemove(item))) return;
    updateRight(rightItems.filter((i) => getId(i) !== getId(item)));
    setLeftItems([...leftItems, item]);
  };

  const OnRemoveAll = async (items: T[]) => {
    if (onBeforeRemoveAll && !(await onBeforeRemoveAll(items))) return;
    updateRight(rightItems.filter((i) => !items.some((j) => getId(j) === getId(i))));
    setLeftItems([...leftItems, ...items]);
  };

  return (
    <Grid>
      <Grid.Col span={6}>
        {leftTitle && <Title order={4}>{leftTitle}</Title>}
        <GenericItemList {...leftConfig} items={leftItems} onAddItem={OnAddItem} onAddAll={OnAddAll} />
      </Grid.Col>
      <Grid.Col span={6}>
        {rightTitle && <Title order={4}>{rightTitle}</Title>}
        <GenericItemList {...rightConfig} items={rightItems} onAddItem={OnRemoveItem} onAddAll={OnRemoveAll} />
      </Grid.Col>
    </Grid>
  );
}

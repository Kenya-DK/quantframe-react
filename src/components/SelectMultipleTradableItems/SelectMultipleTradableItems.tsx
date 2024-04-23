import { Grid, Title } from '@mantine/core';
import { TradableItemList } from '../TradableItemList';
import { CacheTradableItem } from '@api/types';

export type SelectMultipleTradableItemsProps = {
  availableItems: CacheTradableItem[];
  onAddAll?: (items: CacheTradableItem[]) => void;
  onAddItem?: (item: CacheTradableItem) => void;
}
export function SelectMultipleTradableItems({ availableItems }: SelectMultipleTradableItemsProps) {
  return (
    <Grid>
      <Grid.Col span={6}>
        <Title order={4}>A</Title>
        <TradableItemList availableItems={availableItems || []} />
      </Grid.Col>
      <Grid.Col span={6}>
        <Title order={4}>B</Title>
        <TradableItemList availableItems={availableItems || []} />
      </Grid.Col>
    </Grid>
  );
}

import { Button, Container, Grid, Group, NumberInput } from '@mantine/core';
import { useTranslateComponent } from '@hooks/index';
import { useForm } from '@mantine/form';
import { SearchItemField } from './searchItemField';
import { DataTable } from 'mantine-datatable';
import { useState } from 'react';

interface PurchaseNewItemProps {
  onSumit: (data: any) => void;
}
const PurchaseNewItem = (props: PurchaseNewItemProps) => {
  const { onSumit } = props;
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`inventory.${key}`, { ...context })
  const roleForm = useForm({
    initialValues: {
      price: 0,
      item: "",
      rank: 0,
      type: "buy"
    },
    validate: {
      item: (val) => (val.length <= 3 ? ('name_min') : null),
    },
  });
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (d) => { onSumit(d); })}>
      <Grid>
        <Grid.Col span={12} md={6}>
          <Group grow >
            <SearchItemField value={roleForm.values.item} onChange={(value) => roleForm.setFieldValue('item', value)} />
            <NumberInput
              required
              label={useTranslateSearch('price')}
              description={useTranslateSearch('price_description')}
              value={roleForm.values.price}
              min={0}
              onChange={(value) => roleForm.setFieldValue('price', Number(value))}
              error={roleForm.errors.price && 'Invalid identifier'}
            />
            <NumberInput
              required
              label={useTranslateSearch('rank')}
              description={useTranslateSearch('rank_description')}
              value={roleForm.values.rank}
              min={0}
              onChange={(value) => roleForm.setFieldValue('rank', Number(value))}
              error={roleForm.errors.rank && 'Invalid identifier'}
            />
          </Group>
          <Group mt={5} position="center">
            <Button type="submit" onClick={() => roleForm.setFieldValue('type', "buy")} disabled={roleForm.values.item.length <= 0} radius="xl">
              {useTranslateSearch('buy')}
            </Button>
            <Button type="submit" onClick={() => roleForm.setFieldValue('type', "sell")} disabled={roleForm.values.item.length <= 0} radius="xl">
              {useTranslateSearch('sell')}
            </Button>
          </Group>
        </Grid.Col>
      </Grid>
    </form>
  );
}
interface ItemsProps {
  items: any[];
}
const Items = (props: ItemsProps) => {
  const { items } = props;
  return (
    <DataTable
      sx={{ marginTop: "20px" }}
      height={"75vh"}
      withBorder
      striped

      records={items}
      // define columns
      columns={[
        {
          accessor: 'item',
          title: ('columns.name'),
          width: 120,
        },
        {
          accessor: 'price',
          title: ('columns.price'),
          width: 64,
        },
        {
          accessor: 'listed_price',
          title: ('columns.listed_price'),
          width: 64,
        },
        {
          accessor: 'owned',
          title: ('columns.owned'),
          width: 64,
        },
        {
          accessor: 'actions',
          width: 100,
          title: ('components.dataTable.columns.actions'),
          render: ({ }) =>
            <Group>
              <Group mr={10} position="center">
                <NumberInput
                  required
                  size='sm'
                  min={0}
                  rightSection={<Button color="blue" radius="sx">
                    Sell
                  </Button>}
                />
              </Group>
            </Group>
        },
      ]}
    />
  );
}

export const Inventory = () => {
  const [items, setItems] = useState<any[]>([]);
  return (
    <Container >
      <PurchaseNewItem onSumit={(item) => {
        setItems([...items, item]);
      }} />
      <Items items={items} />
    </Container>
  );
}
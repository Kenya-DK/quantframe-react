import { Button, Group, NumberInput, Stack, Tooltip } from "@mantine/core";
import { useForm } from "@mantine/form";
import { SearchItemField } from "../../../../components/searchItemField";
import { CreateStockItemEntryDto, Wfm } from "../../../../types";
import { useTranslatePage } from "../../../../hooks";
import { useState } from "react";

interface PurchaseNewItemProps {
  loading: boolean;
  onSumit: (data: CreateStockItemEntryDto) => void;
}

export const PurchaseNewItem = (props: PurchaseNewItemProps) => {
  const useTranslateItemPanel = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`live_trading.tabs.item.${key}`, { ...context }, i18Key)
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateItemPanel(`fields.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateItemPanel(`buttons.${key}`, { ...context }, i18Key)
  const { onSumit, loading } = props;
  const [, setSelectedItem] = useState<Wfm.ItemDto | undefined>(undefined);
  const roleForm = useForm({
    initialValues: {
      price: 0,
      item: "",
      quantity: 1,
      rank: 0,
      report: true,
    },
  });
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (d) => {
      onSumit({
        item_id: d.item,
        report: d.report,
        price: d.price,
        quantity: d.quantity,
        rank: d.rank
      });
    })}>
      <Stack justify='center' spacing="md">
        <Group grow >
          <SearchItemField value={roleForm.values.item} onChange={(value) => {
            setSelectedItem(value);
            roleForm.setFieldValue('item', value.url_name);
            roleForm.setFieldValue('rank', value.mod_max_rank || 0);
          }} />
          <NumberInput
            required
            label={useTranslateFields('quantity.label')}
            description={useTranslateFields('quantity.description')}
            value={roleForm.values.quantity}
            min={1}
            onChange={(value) => roleForm.setFieldValue('quantity', Number(value))}
            error={roleForm.errors.quantity && 'Invalid identifier'}
          />
          {/* {((selectedItem?.mod_max_rank || 0 > 0) &&
            <NumberInput
              required
              label={useTranslateFields('rank.label')}
              description={useTranslateFields('rank.description')}
              value={roleForm.values.rank}
              min={0}
              max={selectedItem?.mod_max_rank || 0}
              onChange={(value) => roleForm.setFieldValue('rank', Number(value))}
              error={roleForm.errors.rank && 'Invalid identifier'}
            />
          )} */}
          <NumberInput
            required
            label={useTranslateFields('price.label')}
            description={useTranslateFields('price.description')}
            value={roleForm.values.price}
            min={0}
            onChange={(value) => roleForm.setFieldValue('price', Number(value))}
            error={roleForm.errors.price && 'Invalid identifier'}
          />
        </Group>
        <Group mt={5} position="center">
          <Tooltip label={useTranslateButtons('resell.description')} position="top" withArrow>
            <Button loading={loading} type="submit" onClick={() => {
              roleForm.setFieldValue('report', true)
            }} disabled={roleForm.values.item.length <= 0} radius="xl">
              {useTranslateButtons('resell.label')}
            </Button>
          </Tooltip>
          <Tooltip label={useTranslateButtons('resell_without_report.description')} position="top" withArrow>
            <Button loading={loading} type="submit" onClick={() => {
              roleForm.setFieldValue('report', false)
            }} disabled={roleForm.values.item.length <= 0} radius="xl">
              {useTranslateButtons('resell_without_report.label')}
            </Button>
          </Tooltip>
        </Group>
      </Stack>
    </form>
  );
}
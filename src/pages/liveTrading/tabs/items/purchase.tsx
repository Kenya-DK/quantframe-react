import { ActionIcon, Group, NumberInput, Tooltip } from "@mantine/core";
import { useForm } from "@mantine/form";
import { SearchItemField } from "@components/searchItemField";
import { CreateStockItemEntryDto, Wfm } from "$types/index";
import { useTranslatePage } from "@hooks/index";
import { useState } from "react";
import { faShoppingCart } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useAppContext } from "@contexts/index";

interface PurchaseNewItemProps {
  loading: boolean;
  onSubmit: (data: CreateStockItemEntryDto) => void;
}

export const PurchaseNewItem = (props: PurchaseNewItemProps) => {
  const useTranslateItemPanel = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`live_trading.tabs.item.${key}`, { ...context }, i18Key)
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateItemPanel(`fields.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateItemPanel(`buttons.${key}`, { ...context }, i18Key)
  const { onSubmit } = props;
  const [, setSelectedItem] = useState<Wfm.ItemDto | undefined>(undefined);
  const { settings } = useAppContext();
  const roleForm = useForm({
    initialValues: {
      price: 0,
      item: "",
      quantity: 1,
      rank: 0,
    },
  });
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (d) => {
      onSubmit({
        item_id: d.item,
        price: d.price,
        quantity: d.quantity,
        rank: d.rank
      });
    })}>
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
          rightSection={
            <Tooltip label={useTranslateButtons(`resell.${settings?.live_scraper.stock_item.report_to_wfm ? "description_with_report" : "description_without_report"}`)}>
              <ActionIcon type="submit" variant="filled" color="green" disabled={roleForm.values.item.length <= 0} >
                <FontAwesomeIcon icon={faShoppingCart} />
              </ActionIcon>
            </Tooltip>
          }
          rightSectionWidth={40}
        />
      </Group>
    </form>
  );
}
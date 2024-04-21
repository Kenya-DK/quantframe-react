import { Group, NumberInput, Select } from '@mantine/core';
import { useQuery } from '@tanstack/react-query';
import api, { } from "@api/index";
import { CacheTradableItem, CacheTradableItemSubType, SubType } from "@api/types";
import { useEffect, useState } from 'react';
import { useTranslateComponent } from '@hooks/index';
import { upperFirst } from '@mantine/hooks';

export type SelectTradableItemProps = {
  value: string;
  onChange(item: SelectTradableItemItem): void;
}

interface SelectTradableItemItem extends Omit<CacheTradableItem, "sub_type"> {
  label: string;
  value: string;
  available_sub_types?: CacheTradableItemSubType;
  sub_type?: SubType;
}
export function SelectTradableItem({ value, onChange }: SelectTradableItemProps) {
  // State
  const [items, setItems] = useState<SelectTradableItemItem[]>([]);
  const [filteredItems, setFilteredItems] = useState(items);
  const [lastKeyPressed, setLastKeyPressed] = useState<string | null>(null);
  const [selectedItem, setSelectedItem] = useState<SelectTradableItemItem | null>(null);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`select_tradable_item.${key}`, { ...context }, i18Key)
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`fields.${key}`, { ...context }, i18Key)

  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ['cache_items'],
    queryFn: () => api.cache.getTradableItems(),
  })

  useEffect(() => {
    if (!data) return;
    setItems(data.map((item) => ({
      ...item,
      label: item.name,
      value: item.wfm_url_name,
      available_sub_types: item.sub_type,
      sub_type: undefined,
    })))

  }, [data])


  const handleSelect = (item: SelectTradableItemItem) => {
    const new_item = { ...item };
    if (item.available_sub_types) {
      const sub_type = item.available_sub_types;
      if (sub_type.variants)
        new_item.sub_type = { variant: sub_type.variants[0] };
      if (sub_type.max_rank)
        new_item.sub_type = { rank: sub_type.max_rank };
      if (sub_type.amber_stars || sub_type.cyan_stars)
        new_item.sub_type = { cyan_stars: sub_type.cyan_stars, amber_stars: sub_type.amber_stars };
    }
    onChange(new_item);
    setSelectedItem(new_item);
  }


  const handleSubTypeUpdate = (sub_type: SubType) => {
    if (!selectedItem) return;
    setSelectedItem({ ...selectedItem, sub_type });
    onChange({ ...selectedItem, sub_type });
  }

  return (
    <Group>
      <Select
        w={300}
        label={useTranslateFormFields('item.label')}
        placeholder={useTranslateFormFields('item.placeholder')}
        data={items}
        searchable
        limit={10}
        required
        maxDropdownHeight={400}
        value={value}
        onKeyDown={(event) => {
          setLastKeyPressed(event.key);
        }}
        onSearchChange={(searchValue) => {
          setFilteredItems(
            items.filter(item => item.label.toLowerCase().includes(searchValue.toLowerCase()))
          );
        }}
        onBlur={() => {
          if (lastKeyPressed === 'Tab' && filteredItems.length > 0) {
            const firstItem = filteredItems[0];
            handleSelect(firstItem);
          }
          setLastKeyPressed(null);
        }}
        onChange={async (item) => {
          if (!item) return;
          const tItem = items.find(i => i.wfm_url_name === item);
          if (!tItem) return;
          handleSelect(tItem);
        }}
      />
      {(selectedItem && selectedItem.available_sub_types) && (
        <Group>
          {selectedItem.available_sub_types.variants && (
            <Select
              label={useTranslateFormFields('variant.label')}
              placeholder={useTranslateFormFields('variant.placeholder')}
              data={selectedItem.available_sub_types.variants.map((variant) => ({ label: upperFirst(variant), value: variant }))}
              required
              value={selectedItem.sub_type?.variant || selectedItem.available_sub_types.variants[0] || ""}
              onChange={(variant) => {
                if (!selectedItem || !variant) return;
                handleSubTypeUpdate({ variant })
              }} />
          )}
          {selectedItem.available_sub_types.max_rank && (
            <NumberInput
              w={100}
              required
              label={useTranslateFormFields('rank.label')}
              placeholder={useTranslateFormFields('rank.placeholder')}
              value={selectedItem.sub_type?.rank || 0}
              min={0}
              max={selectedItem.available_sub_types.max_rank}
              onChange={(event) => handleSubTypeUpdate({ rank: Number(event) })}
            />
          )}
          {selectedItem.available_sub_types.cyan_stars && (
            <NumberInput
              w={100}
              required
              label={useTranslateFormFields('cyan_stars.label')}
              placeholder={useTranslateFormFields('cyan_stars.placeholder')}
              value={selectedItem.sub_type?.cyan_stars || 0}
              min={0}
              max={selectedItem.available_sub_types.cyan_stars}
              onChange={(event) => handleSubTypeUpdate({ cyan_stars: Number(event), amber_stars: selectedItem.available_sub_types?.amber_stars ? selectedItem.sub_type?.amber_stars : undefined })}
            />
          )}
          {selectedItem.available_sub_types.amber_stars && (
            <NumberInput
              w={100}
              required
              label={useTranslateFormFields('amber_stars.label')}
              placeholder={useTranslateFormFields('amber_stars.placeholder')}
              value={selectedItem.sub_type?.amber_stars || 0}
              min={0}
              max={selectedItem.available_sub_types.amber_stars}
              onChange={(event) => handleSubTypeUpdate({ amber_stars: Number(event), cyan_stars: selectedItem.available_sub_types?.cyan_stars ? selectedItem.sub_type?.cyan_stars : undefined })}
            />
          )}
        </Group>
      )}
    </Group>
  );
}
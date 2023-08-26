
import { Select } from '@mantine/core';
import { useTranslateComponent } from '@hooks/index';
import { useEffect, useState } from 'react';
import { Wfm } from '../types';
import { useTauriContext } from '../contexts';
interface SearchItemFieldProps {
  value: string;
  onChange: (item: Wfm.ItemDto) => void;
}

export const SearchItemField = (props: SearchItemFieldProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`searchItemField.${key}`, { ...context })
  const { tradable_items } = useTauriContext();
  const [items, setItems] = useState<Array<Wfm.ItemDto & { label: string, value: string }>>([]);
  const { value, onChange } = props;
  useEffect(() => {
    setItems(tradable_items.map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name })) || []);
  }, [tradable_items]);
  return (
    <Select
      w={300}
      label={useTranslateSearch('title')}
      placeholder={useTranslateSearch('placeholder')}
      description={useTranslateSearch('description')}
      data={items}
      searchable
      limit={10}
      required
      maxDropdownHeight={400}
      nothingFound={useTranslateSearch('no_results')}
      value={value}
      onChange={async (value) => {
        if (!value) return;
        const item = items.find(item => item.url_name === value);
        if (!item) return;
        onChange(item)
      }}
      filter={(value, item) =>
        item.label?.toLowerCase().includes(value.toLowerCase().trim()) ||
        item.value.toLowerCase().includes(value.toLowerCase().trim())
      }
    />
  );
}
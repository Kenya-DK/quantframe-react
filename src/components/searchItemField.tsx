
import { Select } from '@mantine/core';
import { useTranslateComponent } from '@hooks/index';
import { useEffect, useState } from 'react';
import { Wfm } from '../types';
import { useCacheContext } from '../contexts';
interface SearchItemFieldProps {
  value: string;
  onChange: (item: Wfm.ItemDto) => void;
}

export const SearchItemField = (props: SearchItemFieldProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`searchItemField.${key}`, { ...context })
  const { items: wfItems } = useCacheContext();
  const [items, setItems] = useState<Array<Wfm.ItemDto & { label: string, value: string }>>([]);
  const { value, onChange } = props;
  const [filteredItems, setFilteredItems] = useState(items);
  const [lastKeyPressed, setLastKeyPressed] = useState<string | null>(null);

  useEffect(() => {
    setItems(wfItems.map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name })) || []);
  }, [wfItems]);
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
          onChange(firstItem);
        }
        setLastKeyPressed(null);
      }}
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
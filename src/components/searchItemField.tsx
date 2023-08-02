
import { Select } from '@mantine/core';
import { useTranslateComponent } from '@hooks/index';
import api from "@api/index";
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { Wfm } from '../types';
interface SearchItemFieldProps {
  value: string;
  onChange: (item: Wfm.ItemDto) => void;
}

export const SearchItemField = (props: SearchItemFieldProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`searchItemField.${key}`, { ...context })
  const { value, onChange } = props;
  const [items, setItems] = useState<Array<Wfm.ItemDto & { label: string, value: string }>>([]);

  useQuery({
    queryKey: ['warframes'],
    queryFn: () => api.items.list(),
    onSuccess: (data) => {
      setItems(data.map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name })) || []);
    }
  })
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
        const item = await api.items.findById(value);
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
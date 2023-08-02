
import { Select } from '@mantine/core';
import { useTranslateComponent } from '@hooks/index';
import api from "@api/index";
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
interface SearchItemFieldProps {
  value: string;
  onChange: (id: string) => void;
}

export const SearchItemField = (props: SearchItemFieldProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`searchItemField.${key}`, { ...context })
  const { value, onChange } = props;
  const [items, setItems] = useState<any[]>([]);

  useQuery({
    queryKey: ['warframes'],
    queryFn: () => api.items.list(),
    onSuccess: (data) => {
      setItems(data.map((warframe) => ({ label: warframe.item_name, value: warframe.url_name })) || []);
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
      onChange={(value) => onChange(value ?? "")}
      filter={(value, item) =>
        item.label?.toLowerCase().includes(value.toLowerCase().trim()) ||
        item.value.toLowerCase().includes(value.toLowerCase().trim())
      }
    />
  );
}
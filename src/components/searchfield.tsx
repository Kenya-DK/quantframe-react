
import { ActionIcon, Collapse, Divider, Group, TextInput, Tooltip } from '@mantine/core';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faAdd, faFilter, faSearch } from '@fortawesome/free-solid-svg-icons';
import { useToggle } from '@mantine/hooks';
import React, { useEffect, useState } from 'react';
import { useTranslateComponent } from '@hooks/index';

interface SearchfieldProps {
  value: string;
  onChange: (text: string) => void;
  onSearch?: (text: string) => void;
  onCreate?: () => void;
  rightSection?: React.ReactNode;
  rightSectionWidth?: number;
  filter?: React.ReactNode;
}

export const SearchField = (props: SearchfieldProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`searchfield.${key}`, { ...context })
  const { value, filter, onSearch, onCreate, onChange, rightSection, rightSectionWidth } = props;
  const [openFilter, setOpenFilter] = useToggle();
  const [sectionWidth, setSectionWidth] = useState(115);

  useEffect(() => {
    const buttonWidth = 44;
    let width = 0;
    if (filter)
      width += buttonWidth;
    if (onSearch)
      width += buttonWidth;
    if (onCreate)
      width += buttonWidth;
    setSectionWidth(width);
  }, [onChange, onSearch, onCreate, filter]);


  return (
    <>
      <TextInput value={value} onKeyDown={(e: any) => {
        if (e.key === 'Enter' && onSearch)
          onSearch(e.currentTarget.value);
      }}
        onChange={(e) => { onChange(e.currentTarget.value); }} label={useTranslateSearch('title')} placeholder={useTranslateSearch('placeholder') || "Search..."}
        rightSectionWidth={rightSectionWidth ?? sectionWidth}
        rightSection={
          <Group spacing={"5px"}>
            <Divider orientation="vertical" />
            {rightSection}
            {filter &&
              <Tooltip label={useTranslateSearch('buttons.filter')}>
                <ActionIcon color={openFilter ? "blue.7" : "dark.4"} variant="filled" onClick={async () => setOpenFilter()} >
                  <FontAwesomeIcon icon={faFilter} />
                </ActionIcon>
              </Tooltip>

            }
            {onSearch &&
              <Tooltip label={useTranslateSearch('buttons.search')}>
                <ActionIcon color="blue.7" variant="filled" onClick={async () => {
                  if (onSearch)
                    onSearch(value);
                }} >
                  <FontAwesomeIcon icon={faSearch} />
                </ActionIcon>
              </Tooltip>
            }
            {onCreate &&
              <Tooltip label={useTranslateSearch('buttons.create')}>
                <ActionIcon variant="filled" color="green" onClick={async () => onCreate()} >
                  <FontAwesomeIcon icon={faAdd} />
                </ActionIcon>
              </Tooltip>
            }
          </Group>
        } />
      {openFilter &&
        <Collapse in={openFilter}>
          {filter}
        </Collapse>}
    </>

  );
}
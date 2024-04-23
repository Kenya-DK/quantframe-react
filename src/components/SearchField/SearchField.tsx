import { Group, Box, TextInput, Collapse, Divider, Tooltip, ActionIcon } from '@mantine/core';
import { useTranslateComponent } from '@hooks/index';
import { faAdd, faFilter, faSearch } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useToggle } from '@mantine/hooks';
import { useEffect, useState } from 'react';

export type SearchFieldProps = {
  value: string;
  onChange: (text: string) => void;
  description?: string;
  onSearch?: (text: string) => void;
  onCreate?: () => void;
  rightSection?: React.ReactNode;
  rightSectionWidth?: number;
  filter?: React.ReactNode;
}
export function SearchField({ value, filter, description, onSearch, onCreate, onChange, rightSection, rightSectionWidth }: SearchFieldProps) {
  // States
  const [openFilter, setOpenFilter] = useToggle();
  const [sectionWidth, setSectionWidth] = useState(115);

  // Translate general
  const useTranslateSearchField = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`searchfield.${key}`, { ...context }, i18Key)
  const useTranslateSearchFieldButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`searchfield.buttons.${key}`, { ...context }, i18Key)

  // On change
  useEffect(() => {
    const buttonWidth = 45;
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
    <Box>
      <TextInput
        value={value}
        onKeyDown={(event) => {
          if (event.key === "Enter" && onSearch)
            onSearch(value);
        }}
        onChange={(event) => onChange(event.currentTarget.value)}
        label={useTranslateSearchField("label")}
        placeholder={useTranslateSearchField("placeholder")}
        description={description}
        rightSectionWidth={rightSectionWidth ?? sectionWidth}
        rightSection={
          <Group p={0} m={0} gap={5}>
            <Divider orientation="vertical" />
            {rightSection}
            {filter && (
              <Tooltip label={useTranslateSearchFieldButtons('filter.tooltip')}>
                <ActionIcon color={openFilter ? "blue.7" : "dark.4"} variant="filled" onClick={async () => setOpenFilter()} >
                  <FontAwesomeIcon icon={faFilter} />
                </ActionIcon>
              </Tooltip>
            )}
            {onSearch &&
              <Tooltip label={useTranslateSearchFieldButtons('search.tooltip')}>
                <ActionIcon color="blue.7" variant="filled" onClick={async () => {
                  if (onSearch)
                    onSearch(value);
                }} >
                  <FontAwesomeIcon icon={faSearch} />
                </ActionIcon>
              </Tooltip>
            }
            {onCreate &&
              <Tooltip label={useTranslateSearchFieldButtons('create.tooltip')}>
                <ActionIcon variant="filled" color="green" onClick={async () => onCreate()} >
                  <FontAwesomeIcon icon={faAdd} />
                </ActionIcon>
              </Tooltip>
            }
          </Group>
        }

      />
      {openFilter && (
        <Collapse in={openFilter}>
          {filter}
        </Collapse>
      )}
    </Box>
  );
}
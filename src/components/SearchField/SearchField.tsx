import { Group, Box, TextInput, Collapse, Divider } from "@mantine/core";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { faFilter, faSearch } from "@fortawesome/free-solid-svg-icons";
import { useToggle } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { ActionWithTooltip } from "@components/ActionWithTooltip";

export type SearchFieldProps = {
  value: string;
  onChange: (text: string) => void;
  onSearch?: (text: string) => void;
  searchDisabled?: boolean;
  description?: string;
  onCreate?: () => void;
  rightSection?: React.ReactNode;
  rightSectionWidth?: number;
  filter?: React.ReactNode;
  onFilterToggle?: (open: boolean) => void;
};
export function SearchField({
  value,
  filter,
  description,
  onSearch,
  searchDisabled,
  onCreate,
  onChange,
  rightSection,
  onFilterToggle,
  rightSectionWidth,
}: SearchFieldProps) {
  // States
  const [openFilter, setOpenFilter] = useToggle();
  const [sectionWidth, setSectionWidth] = useState(115);

  // Translate general
  const useTranslateSearchField = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`searchfield.${key}`, { ...context }, i18Key);
  const useTranslateSearchFieldButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`searchfield.buttons.${key}`, { ...context }, i18Key);

  // On change
  useEffect(() => {
    const buttonWidth = 35;
    let width = 0;
    if (filter) width += buttonWidth;
    if (onSearch) width += buttonWidth;
    if (onCreate) width += buttonWidth;
    setSectionWidth(width);
  }, [onChange, onSearch, onCreate, filter]);

  useEffect(() => {
    if (onFilterToggle) onFilterToggle(openFilter);
  }, [openFilter]);

  return (
    <Box>
      <TextInput
        value={value}
        onKeyDown={(event) => {
          if (event.key === "Enter" && onSearch) onSearch(value);
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
              <ActionWithTooltip
                tooltip={useTranslateSearchFieldButtons("filter.tooltip")}
                icon={faFilter}
                color={openFilter ? "blue.7" : "dark.4"}
                actionProps={{ size: "sm" }}
                iconProps={{ size: "xs" }}
                onClick={async () => setOpenFilter()}
              />
            )}
            {onSearch && (
              <ActionWithTooltip
                tooltip={useTranslateSearchFieldButtons("search.tooltip")}
                icon={faSearch}
                color={"blue.7"}
                actionProps={{ size: "sm", disabled: searchDisabled ?? false }}
                iconProps={{ size: "xs" }}
                onClick={async () => {
                  if (onSearch) onSearch(value);
                }}
              />
            )}
            {onCreate && (
              <ActionWithTooltip
                tooltip={useTranslateSearchFieldButtons("create.tooltip")}
                icon={faFilter}
                color={"green"}
                onClick={async () => onCreate()}
              />
            )}
          </Group>
        }
      />
      <Collapse in={openFilter}>{filter}</Collapse>
    </Box>
  );
}

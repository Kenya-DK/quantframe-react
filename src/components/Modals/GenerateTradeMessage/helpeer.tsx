import { Text, Image, Group, SelectProps } from "@mantine/core";

export const renderSelectOption: SelectProps["renderOption"] = ({ option, checked }) => (
  <Group gap="xs" style={{ fontWeight: checked ? 700 : 400 }} justify="flex-start">
    <span>
      <Image src={(option as any).img} fit="contain" width={20} height={20} />
    </span>
    <Text>{option.label}</Text>
  </Group>
);

import { Group, MantineSize, NumberFormatter } from "@mantine/core";
import { memo } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPlat } from "@icons";

export type DisplayPlatinumProps = {
  value: number;
  iconColor?: string;
  size?: MantineSize | (string & {});
};

export const DisplayPlatinum = memo(function DisplayPlatinum({ value, iconColor, size }: DisplayPlatinumProps) {
  return (
    <Group gap={2} style={size ? { fontSize: `var(--mantine-font-size-${size})` } : undefined}>
      <NumberFormatter value={value} thousandsGroupStyle="thousand" thousandSeparator="," /> <FontAwesomeIcon icon={faPlat} color={iconColor} />
    </Group>
  );
});


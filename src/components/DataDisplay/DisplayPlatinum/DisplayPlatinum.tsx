import { Group, NumberFormatter } from "@mantine/core";
import { memo } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPlat } from "@icons";

export type DisplayPlatinumProps = {
  value: number;
};

export const DisplayPlatinum = memo(function DisplayPlatinum({ value }: DisplayPlatinumProps) {
  return (
    <Group gap={2}>
      <NumberFormatter value={value} thousandsGroupStyle="thousand" thousandSeparator="," /> <FontAwesomeIcon icon={faPlat} />
    </Group>
  );
});

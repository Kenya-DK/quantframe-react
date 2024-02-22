
import { Group, MantineNumberSize, NumberInput, Sx } from '@mantine/core';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faLeftRight } from '@fortawesome/free-solid-svg-icons';
import { useState } from 'react';

interface MinMaxFieldProps {
  min: number;
  minAllowed?: number;
  max: number | ""
  maxAllowed?: number;
  width?: number;
  label: string;
  onChange: (min: number, max: number | "") => void;
  size?: MantineNumberSize;
  sx?: Sx | (Sx | undefined)[];
}

export const MinMaxField = ({ size, width, maxAllowed, minAllowed, onChange, label, min, max }: MinMaxFieldProps) => {

  const [] = useState<number | "">("");
  const handleValueChange = (value: number | "", type: "min" | "max") => {
    if (type === "min") {
      onChange(Number(value), max);
    } else {
      // Check if max is less than min
      if (value != "" && Number(value) < min) {
        onChange(min, min);
        return;
      }
      onChange(min, value);
    }
  }

  return (
    <Group spacing={"5px"}>
      <NumberInput
        value={min}
        w={width}
        size={size ? size.toString() : "sm"}
        min={minAllowed ?? 0}
        max={maxAllowed ?? 999999999}
        formatter={(value) =>
          !Number.isNaN(parseFloat(value))
            ? `${value}`.replace(/\B(?<!\.\d*)(?=(\d{3})+(?!\d))/g, ',')
            : ''
        }
        onChange={(value) => handleValueChange(Number(value), "min")}
        label={label}
        placeholder='0'
      />
      <FontAwesomeIcon icon={faLeftRight} style={{ marginTop: 23 }} />
      <NumberInput
        w={width}
        value={max}
        size={size ? size.toString() : "sm"}
        min={minAllowed ?? 0}
        max={maxAllowed ?? 999999999}
        formatter={(value) =>
          !Number.isNaN(parseFloat(value))
            ? `${value}`.replace(/\B(?<!\.\d*)(?=(\d{3})+(?!\d))/g, ',')
            : ''
        }
        mt={23}
        onChange={(value) => handleValueChange(value, "max")}
        placeholder='∞'
      />
    </Group>

  );
}
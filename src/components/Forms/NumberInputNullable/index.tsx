import { NumberInput, NumberInputProps } from "@mantine/core";

export type NumberInputNullableProps = Omit<NumberInputProps, "onChange"> & {
  onChange?: (value: number | undefined) => void;
};

export function NumberInputNullable(props: NumberInputNullableProps) {
  return (
    <NumberInput
      {...props}
      onChange={(value) => {
        let numValue = Number(value) || 0;
        if (props.onChange) {
          if (value === null || value === undefined || numValue <= 0) props.onChange(undefined);
          else props.onChange(numValue);
        }
      }}
    />
  );
}

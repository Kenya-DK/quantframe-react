import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { Select, SelectProps } from "@mantine/core";

export type DateRangeValue = "7" | "30" | "90" | "365" | "all";

export type DateRangeOption = {
  value: string;
  label: string;
};

export type DateRangeSelectProps = Omit<SelectProps, "value" | "onChange" | "data"> & {
  value: string;
  onChange: (value: string) => void;
  options?: DateRangeOption[];
};

export const DATE_RANGE_VALUES: DateRangeValue[] = ["7", "30", "90", "365", "all"];

export function DateRangeSelect({ value, onChange, options, label, ...props }: DateRangeSelectProps) {
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateCommon(`date_range.${key}`, { ...context }, i18Key);

  const data = options ?? DATE_RANGE_VALUES.map((option) => ({ value: option, label: useTranslate(`options.${option}`) }));

  return (
    <Select
      allowDeselect={false}
      label={label ?? useTranslate("label")}
      data={data}
      value={value}
      onChange={(value) => value && onChange(value)}
      w={200}
      radius="md"
      {...props}
    />
  );
}

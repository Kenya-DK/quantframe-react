import { Group, NumberInput } from "@mantine/core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faLeftRight } from "@fortawesome/free-solid-svg-icons";
import { MinMaxDto } from "$types";

export type MinMaxProps = {
  value: MinMaxDto | undefined;
  minAllowed?: number;
  maxAllowed?: number;
  label: string;
  onChange: (value: MinMaxDto | undefined) => void;
};

export function MinMax({ value, label, minAllowed, maxAllowed, onChange }: MinMaxProps) {
  // Translate general
  // const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`riven_filter.${key}`, { ...context }, i18Key)
  // const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)
  // const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)

  return (
    <Group gap={"sm"}>
      <NumberInput
        value={value?.min ?? 0}
        w={"100px"}
        min={minAllowed ?? 0}
        max={maxAllowed ?? 999999999}
        onChange={(n) => {
          const min = Number(n);
          const max = value?.max ?? 0;
          if (min > max) return onChange({ min: min, max: min });

          if (min == 0 && max == 0) onChange(undefined);
          else onChange({ min, max });
        }}
        label={label}
        placeholder="0"
      />
      <FontAwesomeIcon icon={faLeftRight} style={{ marginTop: 23 }} />
      <NumberInput
        mt={23}
        w={"100px"}
        value={value?.max ?? 0}
        min={minAllowed ?? 0}
        max={maxAllowed ?? 999999999}
        onChange={(n) => {
          const min = value?.min ?? 0;
          const max = Number(n);
          if (max == 0 && min == 0) onChange(undefined);
          else if (max == 0) onChange({ min, max: undefined });
          else onChange({ min, max });
        }}
        placeholder="∞"
      />
    </Group>
  );
}

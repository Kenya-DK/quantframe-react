import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Group, NumberInput, Select } from "@mantine/core";
import { upperFirst } from "@mantine/hooks";

export type SelectSubTypeProps = {
  value: TauriTypes.SubType | undefined;
  availableSubTypes?: TauriTypes.CacheTradableItemSubType;
  showLabel?: boolean;
  onChange(item: TauriTypes.SubType): void;
};
export function SelectSubType({ value, availableSubTypes, showLabel = true, onChange }: SelectSubTypeProps) {
  if (!availableSubTypes) return null;

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`select_sub_type.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);

  return (
    <Group>
      <Group>
        {availableSubTypes.variants && (
          <Select
            label={showLabel ? useTranslateFormFields("variant.label") : undefined}
            placeholder={useTranslateFormFields("variant.placeholder")}
            data={availableSubTypes.variants.map((variant) => ({ label: upperFirst(variant), value: variant }))}
            required
            value={value?.variant || availableSubTypes.variants[0] || ""}
            onChange={(variant) => {
              if (!value || !variant) return;
              onChange({ ...value, variant });
            }}
          />
        )}
        {availableSubTypes.max_rank && (
          <NumberInput
            w={150}
            required
            label={showLabel ? useTranslateFormFields("rank.label") : undefined}
            placeholder={showLabel ? useTranslateFormFields("rank.placeholder") : undefined}
            value={value?.rank || 0}
            min={0}
            max={availableSubTypes.max_rank}
            onChange={(event) => {
              if (!value) return;
              onChange({ ...value, rank: Number(event) });
            }}
          />
        )}
        {availableSubTypes.cyan_stars && (
          <NumberInput
            w={150}
            required
            label={showLabel ? useTranslateFormFields("cyan_stars.label") : undefined}
            placeholder={useTranslateFormFields("cyan_stars.placeholder")}
            value={value?.cyan_stars || 0}
            min={0}
            max={availableSubTypes.cyan_stars}
            onChange={(event) => {
              if (!value) return;
              onChange({ ...value, cyan_stars: Number(event) });
            }}
          />
        )}
        {availableSubTypes.amber_stars && (
          <NumberInput
            w={150}
            required
            label={showLabel ? useTranslateFormFields("amber_stars.label") : undefined}
            placeholder={useTranslateFormFields("amber_stars.placeholder")}
            value={value?.amber_stars || 0}
            min={0}
            max={availableSubTypes.amber_stars}
            onChange={(event) => {
              if (!value) return;
              onChange({ ...value, amber_stars: Number(event) });
            }}
          />
        )}
      </Group>
    </Group>
  );
}

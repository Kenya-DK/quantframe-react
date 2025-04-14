import { Flex, NumberInput, Select } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { RivenAttribute, CacheRivenAttribute } from "@api/types";
import { useEffect, useState } from "react";
import { ActionWithTooltip } from "../../ActionWithTooltip";
import { faClose } from "@fortawesome/free-solid-svg-icons";

export type CreateRivenAttributeProps = {
  availableAttributes: CacheRivenAttribute[];
  value: RivenAttribute;
  positiveNumberOnly?: boolean;
  negativeNumberOnly?: boolean;
  canRemove?: boolean;
  onChange?: (values: RivenAttribute) => void;
  onRemove?: (index: number) => void;
};
export function CreateRivenAttribute({
  positiveNumberOnly,
  negativeNumberOnly,
  availableAttributes,
  onChange,
  canRemove,
  onRemove,
  value,
}: CreateRivenAttributeProps) {
  const [currentValue, setCurrentValue] = useState<CacheRivenAttribute | undefined>(undefined);

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`create_riven_attribute.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateFormButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  // User form
  const form = useForm({
    initialValues: {
      ...value,
    },
    validate: {
      value: (value) => {
        console.log("Value", value);
        if (positiveNumberOnly && value < 0) return useTranslateFormFields("value.error.positive");
        if (negativeNumberOnly && value > 0) return useTranslateFormFields("value.error.negative");
        if (currentValue?.positiveOnly && value < 0) return useTranslateFormFields("value.error.positive");
        if (currentValue?.negativeOnly && value > 0) return useTranslateFormFields("value.error.negative");
        return null;
      },
    },
    onValuesChange: (values) => {
      onChange && onChange(values);
    },
  });

  useEffect(() => {
    if (value.url_name) {
      const attr = availableAttributes.find((item) => item.url_name == value.url_name);
      setCurrentValue(attr);
    }
  }, [value.url_name]);

  // Helper functions
  const getAvailableAttributes = () => {
    return availableAttributes.map((item) => ({ label: item.effect, value: item.url_name }));
  };

  const GetUnitSymbol = () => {
    if (currentValue?.unit == "multiply") return "+";
    if (currentValue?.unit == "percent") return "%";
    if (currentValue?.unit == "seconds") return "sec";
    return undefined;
  };

  const GetMaxValue = () => {
    if ((positiveNumberOnly && form.values.value < 0) || (negativeNumberOnly && form.values.value > 0)) return undefined;
    if (positiveNumberOnly && currentValue?.unit != "multiply") return 400;
    if (positiveNumberOnly && currentValue?.unit == "multiply") return 4;

    if (negativeNumberOnly && currentValue?.unit == "multiply") return 1;
    if (negativeNumberOnly && currentValue?.unit != "multiply") return 0;
    return undefined;
  };

  const GetMinValue = () => {
    if ((positiveNumberOnly && form.values.value < 0) || (negativeNumberOnly && form.values.value > 0)) return undefined;
    if (positiveNumberOnly) return 0;
    if (negativeNumberOnly && currentValue?.unit != "multiply") return -400;
    if (negativeNumberOnly && currentValue?.unit == "multiply") return 0;
    return undefined;
  };

  const ValidateValue = () => {
    console.log("ValidateValue", form.values.value);
    if ((positiveNumberOnly && form.values.value < 0) || (negativeNumberOnly && form.values.value > 0))
      form.setFieldValue("value", -form.values.value);
  };

  return (
    <Flex gap={"xs"} align="center">
      <Select
        searchable
        clearable
        w={"100%"}
        limit={5}
        value={form.values.url_name || ""}
        onChange={(event) => {
          form.setFieldValue("url_name", event || "");
          form.setFieldValue("value", 0);
        }}
        data={getAvailableAttributes()}
      />
      <NumberInput
        w={150}
        disabled={form.values.url_name == "N/A" || form.values.url_name == ""}
        step={currentValue?.unit == "multiply" ? 0.1 : 1}
        decimalScale={1}
        max={GetMaxValue()}
        min={GetMinValue()}
        onBlur={() => ValidateValue()}
        value={form.values.value || 0}
        rightSection={currentValue?.unit == "multiply" ? undefined : GetUnitSymbol()}
        leftSection={currentValue?.unit == "multiply" ? GetUnitSymbol() : undefined}
        onChange={(event) => form.setFieldValue("value", Number(event))}
        error={form.errors.value && useTranslateFormFields("value.error")}
        radius="md"
      />
      {onRemove && canRemove && (
        <ActionWithTooltip
          icon={faClose}
          tooltip={useTranslateFormButtons("remove")}
          color="red"
          onClick={() => {
            onRemove && onRemove(0);
          }}
        />
      )}
    </Flex>
  );
}

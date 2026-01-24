import { Flex, NumberInput } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { RivenAttribute, TauriTypes } from "$types";
import { useEffect, useState } from "react";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faClose } from "@fortawesome/free-solid-svg-icons";
import { TokenSearchSelect } from "@components/Forms/TokenSearchSelect";

export type CreateRivenAttributeProps = {
  availableAttributes: TauriTypes.CacheRivenAttribute[];
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
  const [currentValue, setCurrentValue] = useState<TauriTypes.CacheRivenAttribute | undefined>(undefined);

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`create_riven_attribute.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateFormButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  const getExpectedSign = (isPositiveAttribute?: boolean) => {
    if (!currentValue) return undefined;
    if (currentValue.unit == "multiply") return undefined;
    const isPositive =
      typeof isPositiveAttribute == "boolean"
        ? isPositiveAttribute
        : positiveNumberOnly === true
          ? true
          : negativeNumberOnly === true
            ? false
            : undefined;
    if (typeof isPositive != "boolean") return undefined;
    const positiveIsNegative = currentValue.positiveIsNegative === true;
    if (isPositive) return positiveIsNegative ? "negative" : "positive";
    return positiveIsNegative ? "positive" : "negative";
  };

  const normalizeValue = (value: number, expectedSign: "positive" | "negative" | undefined) => {
    if (!expectedSign || value == 0) return value;
    if (expectedSign == "positive" && value < 0) return Math.abs(value);
    if (expectedSign == "negative" && value > 0) return -Math.abs(value);
    return value;
  };

  const getSignError = (value: number, expectedSign: "positive" | "negative" | undefined) => {
    if (!expectedSign) return null;
    if (expectedSign == "positive" && value < 0) return useTranslateFormFields("value.error.positive");
    if (expectedSign == "negative" && value > 0) return useTranslateFormFields("value.error.negative");
    return null;
  };

  const isSignMismatched = (value: number, expectedSign: "positive" | "negative" | undefined) => {
    if (!expectedSign) return false;
    return expectedSign == "positive" ? value < 0 : value > 0;
  };

  // User form
  const form = useForm({
    initialValues: {
      ...value,
    },
    validate: {
      value: (value: number, values: RivenAttribute) => {
        if (currentValue?.unit == "multiply" && value < 0) return useTranslateFormFields("value.error.positive");
        const expectedSign = getExpectedSign(values.positive);
        const signError = getSignError(value, expectedSign);
        if (signError) return signError;
        if (currentValue?.positiveOnly) {
          const positiveOnlyError = getSignError(value, getExpectedSign(true));
          if (positiveOnlyError) return positiveOnlyError;
        }
        if (currentValue?.negativeOnly) {
          const negativeOnlyError = getSignError(value, getExpectedSign(false));
          if (negativeOnlyError) return negativeOnlyError;
        }
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
    return availableAttributes.map((item) => ({ label: item.name, value: item.url_name }));
  };

  const GetUnitSymbol = () => {
    if (currentValue?.unit == "multiply") return form.values.positive ? "+" : "-";
    if (currentValue?.unit == "percent") return "%";
    if (currentValue?.unit == "seconds") return "sec";
    return undefined;
  };

  const GetMaxValue = () => {
    if (!currentValue) return undefined;
    if (currentValue.unit == "multiply") return form.values.positive ? 4 : 1;
    const expectedSign = getExpectedSign(form.values.positive);
    if (isSignMismatched(form.values.value, expectedSign)) return undefined;
    if (expectedSign == "positive") return 400;
    if (expectedSign == "negative") return 0;
    return undefined;
  };

  const GetMinValue = () => {
    if (!currentValue) return undefined;
    if (currentValue.unit == "multiply") return 0;
    const expectedSign = getExpectedSign(form.values.positive);
    if (isSignMismatched(form.values.value, expectedSign)) return undefined;
    if (expectedSign == "positive") return 0;
    if (expectedSign == "negative") return -400;
    return undefined;
  };

  const ValidateValue = () => {
    const expectedSign = getExpectedSign(form.values.positive);
    const normalizedValue = normalizeValue(form.values.value, expectedSign);
    if (normalizedValue != form.values.value) form.setFieldValue("value", normalizedValue);
  };

  return (
    <Flex gap={"xs"} align="center">
      <TokenSearchSelect
        searchable
        clearable
        autoSelectOnBlur
        selectFirstOptionOnChange
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
        decimalScale={currentValue?.unit == "multiply" ? 2 : 1}
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

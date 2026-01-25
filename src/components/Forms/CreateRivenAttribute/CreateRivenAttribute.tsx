import { Flex, NumberInput } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { RivenAttribute, TauriTypes } from "$types";
import { useEffect, useState } from "react";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faClose } from "@fortawesome/free-solid-svg-icons";
import { TokenSearchSelect } from "@components/Forms/TokenSearchSelect";

// warframe.market riven form limits (change if needed, or control from outside)
const RIVEN_PERCENT_ABS_MAX = 699;
const RIVEN_MULTIPLY_MAX_POSITIVE = 2.99;
const RIVEN_MULTIPLY_MAX_NEGATIVE = 0.99;

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

  type ExpectedSign = "positive" | "negative" | undefined;

  const resolveSlotSign = (isPositiveAttribute?: boolean): ExpectedSign => {
    if (typeof isPositiveAttribute == "boolean") return isPositiveAttribute ? "positive" : "negative";
    if (positiveNumberOnly === true) return "positive";
    if (negativeNumberOnly === true) return "negative";
    return undefined;
  };

  const invertSign = (sign: ExpectedSign): ExpectedSign =>
    sign == "positive" ? "negative" : sign == "negative" ? "positive" : undefined;

  const getExpectedSign = (isPositiveAttribute?: boolean): ExpectedSign => {
    if (!currentValue || currentValue.unit == "multiply") return undefined;
    const slotSign = resolveSlotSign(isPositiveAttribute);
    if (!slotSign) return undefined;
    return currentValue.positiveIsNegative ? invertSign(slotSign) : slotSign;
  };

  const isSignMismatched = (value: number, expectedSign: ExpectedSign) => {
    if (!expectedSign) return false;
    return expectedSign == "positive" ? value < 0 : value > 0;
  };

  const getSignError = (value: number, expectedSign: ExpectedSign) => {
    if (!expectedSign || !isSignMismatched(value, expectedSign)) return null;
    return expectedSign == "positive" ? useTranslateFormFields("value.error.positive") : useTranslateFormFields("value.error.negative");
  };

  const normalizeValue = (value: number, expectedSign: ExpectedSign) => {
    if (!expectedSign || value == 0) return value;
    const absValue = Math.abs(value);
    return expectedSign == "positive" ? absValue : -absValue;
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

  const expectedSign = getExpectedSign(form.values.positive);

  const GetUnitSymbol = () => {
    if (currentValue?.unit == "multiply") return form.values.positive ? "+" : "-";
    if (currentValue?.unit == "percent") return "%";
    if (currentValue?.unit == "seconds") return "sec";
    return undefined;
  };

  const getRangeForValue = (value: number) => {
    if (!currentValue) return { min: undefined, max: undefined };
    if (currentValue.unit == "multiply")
      return {
        min: 0,
        max: form.values.positive ? RIVEN_MULTIPLY_MAX_POSITIVE : RIVEN_MULTIPLY_MAX_NEGATIVE,
      };
    if (isSignMismatched(value, expectedSign)) return { min: undefined, max: undefined };
    if (expectedSign == "positive") return { min: 0, max: RIVEN_PERCENT_ABS_MAX };
    if (expectedSign == "negative") return { min: -RIVEN_PERCENT_ABS_MAX, max: 0 };
    return { min: undefined, max: undefined };
  };

  const range = getRangeForValue(form.values.value);

  const ValidateValue = () => {
    let nextValue = normalizeValue(form.values.value, expectedSign);
    const { min, max } = getRangeForValue(nextValue);
    if (typeof min == "number" && nextValue < min) nextValue = min;
    if (typeof max == "number" && nextValue > max) nextValue = max;
    if (nextValue != form.values.value) form.setFieldValue("value", nextValue);
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
        max={range.max}
        min={range.min}
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

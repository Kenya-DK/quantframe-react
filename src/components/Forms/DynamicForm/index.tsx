import {
  Box,
  Button,
  Checkbox,
  FileInput,
  Group,
  MultiSelect,
  NumberInput,
  PasswordInput,
  Radio,
  Select,
  Slider,
  Switch,
  TagsInput,
  Text,
  Textarea,
  TextInput,
} from "@mantine/core";
import { DateInput } from "@mantine/dates";
import { useForm, UseFormReturnType } from "@mantine/form";
import type { ComponentProps, ReactNode } from "react";
import { useTranslation } from "react-i18next";

type UnionKeys<T> = T extends unknown ? keyof T : never;

export type DynamicFormOption = {
  label: string;
  value: string;
};

type DynamicFormFieldBase<TField extends Record<string, any>, TCustom extends Record<string, any> = {}> = {
  field: UnionKeys<TField> & string;
  label?: string;
  description?: string;
  placeholder?: string;
  required?: boolean;
  requiredMessage?: string;
  options?: DynamicFormOption[];
  autofocus?: boolean;
  validate?: (value: any, values: Record<string, any>) => string | null | undefined;
} & TCustom;

export type DynamicFormField<TField extends Record<string, any>, TCustom extends Record<string, any> = {}> =
  | (DynamicFormFieldBase<TField, TCustom> & { type: "text"; props?: ComponentProps<typeof TextInput> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "textarea"; props?: ComponentProps<typeof Textarea> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "number"; props?: ComponentProps<typeof NumberInput> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "checkbox"; props?: ComponentProps<typeof Checkbox> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "switch"; props?: ComponentProps<typeof Switch> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "slider"; props?: ComponentProps<typeof Slider> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "datepicker"; props?: ComponentProps<typeof DateInput> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "file"; props?: ComponentProps<typeof FileInput> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "radio"; props?: ComponentProps<typeof Radio.Group> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "select"; props?: ComponentProps<typeof Select> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "multiselect"; props?: ComponentProps<typeof MultiSelect> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "password"; props?: ComponentProps<typeof PasswordInput> })
  | (DynamicFormFieldBase<TField, TCustom> & { type: "tagsinput"; props?: ComponentProps<typeof TagsInput> })
  | (DynamicFormFieldBase<TField, TCustom> & {
      type: "custom";
      render: (ctx: { value: any; onChange: (value: any) => void; error?: string; form: UseFormReturnType<TField> }) => ReactNode;
    });

export type DynamicFormGroup<TField extends Record<string, any>, TCustom extends Record<string, any> = {}> = {
  type: "group";
  props?: ComponentProps<typeof Group>;
  fields: DynamicFormField<TField, TCustom>[];
} & TCustom;

export type DynamicFormItem<TField extends Record<string, any>, TCustom extends Record<string, any> = {}> =
  | DynamicFormField<TField, TCustom>
  | DynamicFormGroup<TField, TCustom>;

export type DynamicFormProps<T extends Record<string, any>, TField extends Record<string, any> = T, TCustom extends Record<string, any> = {}> = {
  value: T | undefined;
  items: DynamicFormItem<TField, TCustom>[];
  i18n_key?: string;
  onChange?: (values: T) => void;
  onSubmit?: (values: T) => void;
  onCancel?: () => void;
  cancelLabel?: string;
  confirmLabel?: string;
  loading?: boolean;
  disabled?: boolean;
};

export function DynamicForm<TField extends Record<string, any>, TCustom extends Record<string, any> = {}>({
  value,
  i18n_key,
  items,
  onChange,
  onSubmit,
  onCancel,
  cancelLabel = "Cancel",
  confirmLabel = "Submit",
  loading,
  disabled,
}: DynamicFormProps<TField, TField, TCustom>) {
  const { t: translate } = useTranslation();

  const t = i18n_key ? (key: string) => translate(`${i18n_key}.${key}`) : (key: string) => translate(key);

  const fields = items.flatMap((item) => (item.type === "group" ? item.fields : [item]));

  const label = (field: DynamicFormField<TField, TCustom>) => field.label ?? (i18n_key ? t(`${field.field}.label`) : undefined);
  const placeholder = (field: DynamicFormField<TField, TCustom>) => field.placeholder ?? (i18n_key ? t(`${field.field}.placeholder`) : undefined);
  const description = (field: DynamicFormField<TField, TCustom>) => field.description ?? (i18n_key ? t(`${field.field}.description`) : undefined);

  const initialValues: Record<string, any> = {
    ...(value ?? {}),
    ...Object.fromEntries(
      fields.map((field) => [
        field.field,
        (value as Record<string, any> | undefined)?.[field.field] ?? (field.type === "checkbox" || field.type === "switch" ? false : ""),
      ]),
    ),
  };

  const form = useForm<TField>({
    initialValues: initialValues as TField,

    validate: (values) => {
      const errors: Record<string, string> = {};

      for (const field of fields) {
        if (field.required) {
          const fieldValue = values[field.field];

          if (fieldValue === undefined || fieldValue === null || fieldValue === "") {
            const key = `${field.field}.required`;
            const resolved = t(key);
            errors[field.field] = field.requiredMessage ?? (resolved !== key ? resolved : (label(field) ?? field.field));
          }
        }

        if (field.validate) {
          const customError = field.validate(values[field.field], values);
          if (customError) {
            errors[field.field] = customError;
          }
        }
      }

      return errors;
    },

    onValuesChange: (values) => {
      const validation = form.validate();

      if (!validation.hasErrors) onChange?.(values);
    },
  });

  const handleSubmit = form.onSubmit((values) => {
    onSubmit?.(values);
  });

  const renderField = (field: DynamicFormField<TField, TCustom>) => {
    const key = field.field;
    const value = form.values[key];

    switch (field.type) {
      case "text":
        return (
          <TextInput
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            required={field.required}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
            value={String(value ?? "")}
          />
        );

      case "textarea":
        return (
          <Textarea
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            required={field.required}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
          />
        );

      case "number":
        return (
          <NumberInput
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            required={field.required}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
            value={typeof value === "number" ? value : ""}
          />
        );

      case "checkbox":
        return (
          <Checkbox
            key={key}
            label={label(field)}
            description={description(field)}
            {...field.props}
            {...form.getInputProps(key, { type: "checkbox" })}
            checked={Boolean(value)}
          />
        );

      case "switch":
        return (
          <Switch
            key={key}
            label={label(field)}
            description={description(field)}
            {...field.props}
            {...form.getInputProps(key, { type: "checkbox" })}
            checked={Boolean(value)}
          />
        );

      case "slider":
        return (
          <Box key={key}>
            <Text size="sm" mb={8}>
              {label(field)}
            </Text>
            <Slider {...field.props} {...form.getInputProps(key)} value={typeof value === "number" ? value : 0} />
          </Box>
        );

      case "datepicker":
        return (
          <DateInput
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            required={field.required}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
          />
        );

      case "file":
        return (
          <FileInput
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            required={field.required}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
          />
        );

      case "radio":
        return (
          <Radio.Group key={key} label={label(field)} description={description(field)} {...field.props} {...form.getInputProps(key)}>
            {(field.options ?? []).map((option) => (
              <Radio key={option.value} value={option.value} label={option.label} />
            ))}
          </Radio.Group>
        );

      case "select":
        return (
          <Select
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            required={field.required}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
            value={String(value ?? "")}
            data={field.options ?? field.props?.data ?? []}
          />
        );

      case "multiselect":
        return (
          <MultiSelect
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
            value={Array.isArray(value) ? value.map(String) : []}
            data={field.options ?? field.props?.data ?? []}
          />
        );

      case "password":
        return (
          <PasswordInput
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            required={field.required}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
            value={String(value ?? "")}
          />
        );

      case "tagsinput":
        return (
          <TagsInput
            key={key}
            label={label(field)}
            placeholder={placeholder(field)}
            description={description(field)}
            required={field.required}
            data-autofocus={field.autofocus || undefined}
            {...field.props}
            {...form.getInputProps(key)}
            value={Array.isArray(value) ? value.map(String) : []}
          />
        );

      case "custom":
        return (
          <Box key={key}>
            {field.render({
              value,
              onChange: (val: any) => form.setFieldValue(key, val),
              error: form.errors[key] as string | undefined,
              form,
            })}
          </Box>
        );

      default:
        return null;
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <Box px="md">
        {items.map((item, index) => {
          if (item.type === "group") {
            return (
              <Group key={index} {...item.props}>
                {item.fields.map(renderField)}
              </Group>
            );
          }

          return renderField(item);
        })}

        {(onCancel || onSubmit) && (
          <Group justify="flex-end" mt="xl">
            {onCancel && (
              <Button color="red" onClick={onCancel} radius="xl" disabled={disabled}>
                {cancelLabel}
              </Button>
            )}

            {onSubmit && (
              <Button type="submit" color="green" radius="xl" loading={loading} disabled={disabled}>
                {confirmLabel}
              </Button>
            )}
          </Group>
        )}
      </Box>
    </form>
  );
}

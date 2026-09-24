import { DynamicForm, DynamicFormItem } from "@components/Forms/DynamicForm";
import { Container, FocusTrap } from "@mantine/core";
import { ContextModalProps } from "@mantine/modals";
import i18next from "i18next";
import { useMemo } from "react";

export type PromptField = {
  type:
    | "text"
    | "number"
    | "select"
    | "textarea"
    | "datepicker"
    | "checkbox"
    | "radio"
    | "switch"
    | "slider"
    | "range"
    | "file"
    | "multiselect"
    | "group";
  name: string;
  label: string;
  value?: any;
  attributes?: any;
  required?: boolean;
  autoFocus?: boolean;
  description?: string;
  placeholder?: string;
  options?: PromptFieldOption[];
};

export type PromptFieldOption = {
  label: string;
  value: string;
};

export type PromptModalProps = {
  confirmLabel?: string;
  cancelLabel?: string;
  height?: string;
  fields: PromptField[];
  onConfirm: (data: any) => void;
  onCancel: (id: string) => void;
};

const BuildFormValues = (fields: PromptField[]): { [key: string]: any } => {
  const formValues: { [key: string]: any } = {};

  for (let index = 0; index < fields.length; index++) {
    const field = fields[index];
    switch (field.type) {
      case "text":
      case "textarea":
        formValues[field.name] = field.value || "";
        break;
      case "number":
      case "range":
      case "slider":
        formValues[field.name] = field.value || 0;
        break;
      case "select":
        formValues[field.name] = field.options ? field.value || field.options[0].value : "";
        break;
      case "checkbox":
      case "switch":
        formValues[field.name] = field.value || false;
        break;
      case "radio":
        formValues[field.name] = field.options ? field.options[0].value : "";
        break;
      case "multiselect":
        formValues[field.name] = field.options ? [field.options[0].value] : [];
        break;
      case "file":
        formValues[field.name] = field.value || null;
        break;
      default:
        break;
    }
  }

  return formValues;
};

const FormatField = (field: PromptField): DynamicFormItem<Record<string, any>> | null => {
  if (field.type === "group") return null;

  return {
    type: field.type,
    field: field.name,
    label: field.label,
    description: field.description,
    placeholder: field.placeholder,
    options: field.options,
    required: field.required,
    props: {
      ...field.attributes,
      ...(field.autoFocus ? { autoFocus: true } : {}),
    },
  } as DynamicFormItem<Record<string, any>>;
};

export function PromptModal({ context, id, innerProps }: ContextModalProps<PromptModalProps>) {
  const { height, confirmLabel, cancelLabel, fields, onConfirm, onCancel } = innerProps;

  const formValues = useMemo<{ [key: string]: any }>(() => BuildFormValues(fields), [fields]);
  const items = useMemo(() => fields.map(FormatField).filter((item): item is DynamicFormItem<Record<string, any>> => item !== null), [fields]);

  return (
    <Container size="auto" h={height} pt={25}>
      <FocusTrap active={true}>
        <DynamicForm
          value={formValues}
          items={items}
          onSubmit={(data) => {
            context.closeModal(id);
            onConfirm(data);
          }}
          onCancel={() => {
            context.closeModal(id);
            onCancel(id);
          }}
          cancelLabel={cancelLabel || i18next.t("components.modals.base.buttons.cancel")}
          confirmLabel={confirmLabel || i18next.t("components.modals.base.buttons.confirm")}
        />
      </FocusTrap>
    </Container>
  );
}

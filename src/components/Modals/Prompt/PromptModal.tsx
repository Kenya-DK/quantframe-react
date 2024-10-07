import { Button, Group, TextInput, Container, Textarea, Select, NumberInput, Checkbox } from '@mantine/core';
import { useForm } from '@mantine/form';
import { ContextModalProps } from '@mantine/modals';
import i18next from 'i18next';


export type PromptField = {
  type: 'text' | 'number' | 'select' | 'textarea' | 'checkbox' | 'radio' | 'switch' | 'slider' | 'range' | 'file' | 'multiselect' | 'group';
  name: string;
  label: string;
  value?: any;
  attributes?: any;
  required?: boolean;
  options?: PromptFieldOption[];
}

export type PromptFieldOption = {
  label: string,
  value: string;
}

export type PromptModalProps = {
  confirmLabel?: string;
  cancelLabel?: string;
  height?: string;
  fields: PromptField[];
  onConfirm: (data: any) => void;
  onCancel: (id: string) => void;
}


export function PromptModal({ context, id, innerProps }: ContextModalProps<PromptModalProps>) {

  const { height, confirmLabel, cancelLabel, fields, onConfirm, onCancel } = innerProps;

  const formValues: { [key: string]: any; } = {};

  for (let index = 0; index < fields.length; index++) {
    const field = fields[index];
    switch (field.type) {
      case 'text':
      case 'textarea':
        formValues[field.name] = field.value || '';
        break;
      case 'number':
      case 'range':
      case 'slider':
        formValues[field.name] = field.value || 0;
        break;
      case 'select':
        formValues[field.name] = field.options ? field.value || field.options[0].value : '';
        break;
      case 'checkbox':
      case 'switch':
        formValues[field.name] = field.value || false;
        break;
      case 'radio':
        formValues[field.name] = field.options ? field.options[0].value : '';
        break;
      case 'multiselect':
        formValues[field.name] = field.options ? [field.options[0].value] : [];
        break;
      default:
        break;
    }
  }

  const form = useForm({
    initialValues: formValues,
    validate: {

      // password: (val) => (val.length <= 6 ? 'Password should include at least 6 characters' : null),
    },
  });
  return (
    <form method="post" onSubmit={form.onSubmit(async (data) => {
      context.closeModal(id)
      onConfirm(data);
    })}>
      <Container size="auto" h={height} pt={25}>
        {fields.map((field, index) => {
          switch (field.type) {
            case 'text':
              return (
                <Group grow key={index}>
                  <TextInput
                    {...form.getInputProps(field.name)}
                    required={field.required}
                    label={field.label}
                    value={form.values[field.name]}
                    onChange={(event) => form.setFieldValue(field.name, event.currentTarget.value)}
                  />
                </Group>
              );
            case 'textarea':
              return (
                <Group grow key={index}>
                  <Textarea
                    {...form.getInputProps(field.name)}
                    required={field.required}
                    label={field.label}
                    value={form.values[field.name]}
                    onChange={(event) => form.setFieldValue(field.name, event.currentTarget.value)}
                  />
                </Group>
              );
            case 'select':
              return (
                <Group grow key={index}>
                  <Select
                    {...form.getInputProps(field.name)}
                    required={field.required}
                    label={field.label}
                    value={form.values[field.name]}
                    data={field.options || []}
                  />
                </Group>
              )
            case 'number':
              return (
                <Group grow key={index}>
                  <NumberInput
                    {...form.getInputProps(field.name)}
                    {...field.attributes}
                    required={field.required}
                    label={field.label}

                    value={form.values[field.name]}
                    onChange={(value) => form.setFieldValue(field.name, Number(value))}
                  />
                </Group>
              )
            case 'checkbox':
              return (
                <Group grow key={index}>
                  <Checkbox
                    {...form.getInputProps(field.name)}
                    label={field.label}
                    value={form.values[field.name]}
                    checked={form.values[field.name]}
                    onChange={(event) => form.setFieldValue(field.name, event.currentTarget.checked)}
                  />
                </Group>
              )
            default:
              return (<></>);
          }
        })}
      </Container>
      <Group justify="flex-end" mt="xl">
        <Button color='red' onClick={() => {
          context.closeModal(id);
          onCancel(id);
        }} radius="xl">
          {cancelLabel || i18next.t('components.modals.base.buttons.cancel')}
        </Button>
        <Button type="submit" color='green' radius="xl" >
          {confirmLabel || i18next.t('components.modals.base.buttons.confirm')}
        </Button>
      </Group>
    </form>
  );
}
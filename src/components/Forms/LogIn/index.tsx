import { Anchor, PaperProps, Divider, Group, Paper, Text } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { DynamicForm } from "../DynamicForm";

export type LogInFormProps = {
  onSubmit: (values: { email: string; password: string }) => void;
  is_loading?: boolean;
  hide_submit?: boolean;
  paperProps?: PaperProps;
  footerContent?: React.ReactNode;
};

export function LogInForm(props: LogInFormProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`log_in.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  return (
    <Paper radius="md" p="xl" withBorder {...props.paperProps}>
      <Text size="lg" fw={500}>
        {useTranslateForm("title")}
      </Text>

      <Divider my="lg" />

      <DynamicForm<{ email: string; password: string }>
        value={{ email: "", password: "" }}
        i18n_key="components.forms.log_in.fields"
        items={[
          {
            field: "email",
            type: "text",
            required: true,
            validate: (value) => (/^\S+@\S+$/.test(value) ? null : useTranslateFormFields("email.error")),
          },
          { field: "password", type: "password", required: true, props: { mt: "md" } },
        ]}
        onSubmit={(values) => props.onSubmit({ email: values.email, password: values.password })}
        confirmLabel={useTranslateButtons("submit")}
        loading={props.is_loading}
        disabled={props.hide_submit}
      />

      <Anchor component="button" type="button" c="dimmed" size="xs" mt="md" display="block">
        {useTranslateForm("register")}
      </Anchor>

      {props.footerContent && (
        <Group mt={15} grow>
          {props.footerContent}
        </Group>
      )}
    </Paper>
  );
}

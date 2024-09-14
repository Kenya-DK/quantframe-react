import { Anchor, PaperProps, Button, Divider, Group, Paper, PasswordInput, Stack, TextInput, Text } from '@mantine/core';
import { useForm } from '@mantine/form';
import { useTranslateForms } from '@hooks/useTranslate.hook';

export type LogInFormProps = {
  onSubmit: (values: { email: string; password: string }) => void;
  is_loading?: boolean;
  hide_submit?: boolean;
  paperProps?: PaperProps;
  footerContent?: React.ReactNode;
}

export function LogInForm(props: LogInFormProps) {


  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`log_in.${key}`, { ...context }, i18Key)
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)

  // User form
  const form = useForm({
    initialValues: {
      email: '',
      name: '',
      password: '',
      terms: true,
    },
    validate: {
      email: (val) => (/^\S+@\S+$/.test(val) ? null : 'Invalid email'),
    },
  });
  return (
    <Paper radius="md" p="xl" withBorder {...props.paperProps}>

      <Text size="lg" fw={500}>
        {useTranslateForm('title')}
      </Text>

      <Divider my="lg" />

      <form onSubmit={form.onSubmit(() => {
        props.onSubmit({ email: form.values.email, password: form.values.password })
      })}>
        <Stack>
          <TextInput
            required
            label={useTranslateFormFields('email.label')}
            placeholder={useTranslateFormFields('email.placeholder')}
            value={form.values.email}
            onChange={(event) => form.setFieldValue('email', event.currentTarget.value)}
            error={form.errors.email && useTranslateFormFields('email.error')}
            radius="md"
          />

          <PasswordInput
            required
            label={useTranslateFormFields('password.label')}
            placeholder={useTranslateFormFields('password.placeholder')}
            value={form.values.password}
            onChange={(event) => form.setFieldValue('password', event.currentTarget.value)}
            error={form.errors.password && useTranslateFormFields('password.error')}
            radius="md"
          />

        </Stack>

        <Group justify="space-between" mt="xl">
          <Anchor component="button" type="button" c="dimmed" size="xs">
            {useTranslateForm('register')}
          </Anchor>
          <Button disabled={props.hide_submit} loading={props.is_loading} type="submit" radius="xl">
            {useTranslateButtons('submit')}
          </Button>
        </Group>
      </form>
      {props.footerContent && (
        <Group mt={15} grow>
          {props.footerContent}
        </Group>
      )}
    </Paper>
  );
}
import { Container } from "@mantine/core";
import { SettingsNotifications } from "@api/types";
import { useForm } from "@mantine/form";

export type NotificationPanelProps = {
  value: SettingsNotifications;
  onSubmit: (value: SettingsNotifications) => void;
}
export const NotificationPanel = ({ onSubmit, value }: NotificationPanelProps) => {

  // Translate general
  // const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`log_in.${key}`, { ...context }, i18Key)
  // const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)
  // const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)

  // Form
  const form = useForm({
    initialValues: value,
    validate: {},
  });

  return (
    <Container>
      <form onSubmit={form.onSubmit(() => { onSubmit(form.values) })}>
      </form>
    </Container>
  );
};
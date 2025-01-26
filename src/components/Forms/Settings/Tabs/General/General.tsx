import { Box, Button, Group, TextInput } from "@mantine/core";
import { Settings } from "@api/types";
import { useForm } from "@mantine/form";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { TooltipIcon } from "@components/TooltipIcon";

export type GeneralPanelProps = {
  value: Settings;
  onSubmit?: (value: Settings) => void;
};
export const GeneralPanel = ({ value, onSubmit }: GeneralPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.general.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  // User form
  const form = useForm({
    initialValues: value,
    validate: {},
  });
  return (
    <Box p={"md"}>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          if (onSubmit) onSubmit(form.values);
        }}
      >
        <Group gap={"md"} mt={25}>
          <TextInput
            w={350}
            label={useTranslateFormFields("wf_log_path.label")}
            placeholder={useTranslateFormFields("wf_log_path.placeholder")}
            rightSection={<TooltipIcon label={useTranslateFormFields("wf_log_path.tooltip")} />}
            value={form.values.wf_log_path}
            onChange={(event) => form.setFieldValue("wf_log_path", event.currentTarget.value)}
            radius="md"
          />
        </Group>
        <Group
          justify="flex-end"
          style={{
            position: "absolute",
            bottom: 25,
            right: 25,
          }}
        >
          <Button type="submit" variant="light" color="blue">
            {useTranslateButtons("save.label")}
          </Button>
        </Group>
      </form>
    </Box>
  );
};

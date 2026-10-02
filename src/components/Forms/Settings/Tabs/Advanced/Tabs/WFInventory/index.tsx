import { TauriTypes } from "$types";
import api from "@api/index";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Button, Group, Select, Stack, TextInput } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { notifications } from "@mantine/notifications";
export type WFInventoryPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};
const getFieldPath = (field: string) => `wf_inventory.${field}`;
export const WFInventoryPanel = ({ form }: WFInventoryPanelProps) => {
  const updateMutation = api.wf_inventory.update();
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.advanced.wf_inventory.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateFormButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  const source = form.values.wf_inventory.source;
  const sourceType: string = typeof source === "string" ? source : Object.keys(source)[0];

  const setSource = (value: string | null) => {
    if (!value) return;
    if (value === "None") {
      form.setFieldValue(getFieldPath("source"), "None");
    } else if (value === "Profile") {
      const prev = source !== "None" && "Profile" in source ? source.Profile : undefined;
      form.setFieldValue(getFieldPath("source"), { Profile: { id: prev?.id || "" } });
    } else if (value === "Alecaframe") {
      const prev = source !== "None" && "Alecaframe" in source ? source.Alecaframe : undefined;
      form.setFieldValue(getFieldPath("source"), { Alecaframe: { path: prev?.path || "" } });
    } else if (value === "File") {
      const prev = source !== "None" && "File" in source ? source.File : undefined;
      form.setFieldValue(getFieldPath("source"), { File: { path: prev?.path || "" } });
    }
  };

  return (
    <Box h="100%" p={"md"}>
      <Stack>
        <Select
          allowDeselect={false}
          label={useTranslateFormFields("source.label")}
          description={useTranslateFormFields("source.description")}
          value={sourceType}
          onChange={setSource}
          data={[
            { value: "None", label: useTranslateFormFields("source.options.none") },
            { value: "Profile", label: useTranslateFormFields("source.options.profile") },
            { value: "Alecaframe", label: useTranslateFormFields("source.options.alecaframe") },
            { value: "File", label: useTranslateFormFields("source.options.file") },
          ]}
          radius="md"
        />
        {sourceType === "Profile" && (
          <TextInput
            label={useTranslateFormFields("player_id.label")}
            placeholder={useTranslateFormFields("player_id.placeholder")}
            rightSection={<TooltipIcon label={useTranslateFormFields("player_id.tooltip")} />}
            radius="md"
            value={source !== "None" && "Profile" in source ? source.Profile.id : ""}
            onChange={(event) =>
              form.setFieldValue(getFieldPath("source"), {
                Profile: { id: event.currentTarget.value },
              })
            }
          />
        )}
        {sourceType === "Alecaframe" && (
          <TextInput
            label={useTranslateFormFields("path.label")}
            placeholder={useTranslateFormFields("path.placeholder")}
            rightSection={<TooltipIcon label={useTranslateFormFields("path.tooltip")} />}
            radius="md"
            value={source !== "None" && "Alecaframe" in source ? source.Alecaframe.path : ""}
            onChange={(event) =>
              form.setFieldValue(getFieldPath("source"), {
                Alecaframe: { path: event.currentTarget.value },
              })
            }
          />
        )}
        {sourceType === "File" && (
          <TextInput
            label={useTranslateFormFields("file_path.label")}
            placeholder={useTranslateFormFields("file_path.placeholder")}
            rightSection={<TooltipIcon label={useTranslateFormFields("file_path.tooltip")} />}
            radius="md"
            value={source !== "None" && "File" in source ? source.File.path : ""}
            onChange={(event) =>
              form.setFieldValue(getFieldPath("source"), {
                File: { path: event.currentTarget.value },
              })
            }
          />
        )}
        <Group>
          <Button
            mt="md"
            onClick={() =>
              updateMutation.mutate(undefined, {
                onSuccess: () => notifications.show({ color: "green", message: useTranslateFormButtons("update_success") }),
                onError: () => notifications.show({ color: "red", message: useTranslateFormButtons("update_error") }),
              })
            }
            color="blue"
            loading={updateMutation.isPending}
            disabled={sourceType === "None"}
          >
            {useTranslateFormButtons("update")}
          </Button>
        </Group>
      </Stack>
    </Box>
  );
};

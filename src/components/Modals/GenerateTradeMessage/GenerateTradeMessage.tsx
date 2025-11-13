import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { Box, Checkbox, Group, Stack, TextInput, Text } from "@mantine/core";
import { useForm } from "@mantine/form";
import { ActionWithTooltip } from "../../Shared/ActionWithTooltip";
import { faCopy } from "@fortawesome/free-solid-svg-icons";
import { notifications } from "@mantine/notifications";
import { TauriTypes } from "$types";
const MAX_LENGTH = 180;

export interface GenerateTradeMessageModalProps {
  prefix: string;
  suffix: string;
  items: TauriTypes.ChatLink[];
}

export function GenerateTradeMessageModal({ prefix, suffix, items }: GenerateTradeMessageModalProps) {
  const form = useForm({
    initialValues: {
      prefix: prefix || "",
      suffix: suffix || "",
      spacing: true,
    },
  });

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`generate_trade_message.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);

  const GenerateMessage = () => {
    const { prefix, suffix } = form.values;
    let message = prefix;
    items.forEach((item) => {
      let candidate = `${message}`;
      if (item.prefix) candidate += `${item.prefix}`;
      candidate += `[${item.link}]`;
      if (item.suffix) candidate += `${item.suffix} `;
      if (form.values.spacing) candidate = candidate.replace(/<SP>/g, " ");
      else candidate = candidate.replace(/<SP>/g, "");

      if (candidate.length > MAX_LENGTH) return;
      message = candidate;
    });
    // Add flair if space allows
    if (message.length <= MAX_LENGTH - suffix.length) message += suffix;
    return message || "";
  };
  const CopyToClipboard = () => {
    const message = GenerateMessage();
    navigator.clipboard.writeText(message);
    notifications.show({
      title: useTranslateCommon("notifications.copy_to_clipboard.title"),
      message: useTranslateCommon("notifications.copy_to_clipboard.message", { message }),
      color: "green.7",
    });
  };
  return (
    <Box>
      <Stack>
        <Group>
          <Group>
            <TextInput
              label={useTranslateFields("prefix.label")}
              description={useTranslateFields("prefix.description")}
              {...form.getInputProps("prefix")}
            />
            <TextInput
              label={useTranslateFields("suffix.label")}
              description={useTranslateFields("suffix.description")}
              {...form.getInputProps("suffix")}
            />
            <Checkbox
              label={useTranslateFields("spacing.label")}
              description={useTranslateFields("spacing.description")}
              {...form.getInputProps("spacing", { type: "checkbox" })}
            />
          </Group>
        </Group>
        <TextInput
          readOnly
          maxLength={MAX_LENGTH}
          label={useTranslateFields("message.label")}
          description={useTranslateFields("message.description")}
          value={GenerateMessage()}
          rightSection={
            <ActionWithTooltip
              tooltip={useTranslate("button_copy_tooltip")}
              icon={faCopy}
              onClick={() => {
                CopyToClipboard();
              }}
            />
          }
        />
        <Text fw={"xs"}>{useTranslateFields("message.length_info", { length: GenerateMessage().length, max: MAX_LENGTH })}</Text>
      </Stack>
    </Box>
  );
}

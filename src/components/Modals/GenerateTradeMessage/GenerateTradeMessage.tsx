import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { Box, Group, Stack, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { ActionWithTooltip } from "../../Shared/ActionWithTooltip";
import { faCopy } from "@fortawesome/free-solid-svg-icons";
import { notifications } from "@mantine/notifications";

export interface TradeMessage {
  prefix?: string;
  suffix?: string;
  name: string;
  price: number;
}
export interface GenerateTradeMessageModalProps {
  prefix: string;
  suffix: string;
  items: TradeMessage[];
}

export function GenerateTradeMessageModal({ prefix, suffix, items }: GenerateTradeMessageModalProps) {
  const form = useForm({
    initialValues: {
      prefix: prefix,
      suffix: suffix,
    },
  });

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`generate_trade_message.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);

  const GenerateMessage = () => {
    const MAX_LENGTH = 180;
    const { prefix, suffix } = form.values;
    let message = prefix;
    items.forEach((item) => {
      let candidate = `${message}`;
      if (item.prefix) candidate += `${item.prefix}`;
      candidate += `[${item.name}]`;
      if (item.suffix) candidate += `${item.suffix} `;
      candidate += `${item.price}p`;

      if (candidate.length > MAX_LENGTH) return;
      message = candidate;
    });
    // Add flair if space allows
    if (message.length <= MAX_LENGTH - suffix.length) message += suffix;
    return message;
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
          </Group>
        </Group>
        <TextInput
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
      </Stack>
    </Box>
  );
}

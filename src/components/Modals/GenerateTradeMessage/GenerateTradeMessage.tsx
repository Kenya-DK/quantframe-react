import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { Box, TextInput, Text, Grid, Group, Stack, Divider, Title, Button, Select, Image, Autocomplete } from "@mantine/core";
import { useForm } from "@mantine/form";
import { ActionWithTooltip } from "../../Shared/ActionWithTooltip";
import { faCopy, faPlus, faTrash } from "@fortawesome/free-solid-svg-icons";
import { notifications } from "@mantine/notifications";
import { DisplaySettings, GetChatLinkNameMultiple, ApplyTemplate, GroupByKey } from "@utils/helper";
import { useEffect, useState } from "react";
import { ItemWithMeta } from "$types";
import { DataTable } from "mantine-datatable";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { renderSelectOption } from "./helpeer";
import { upperFirst } from "@mantine/hooks";
const MAX_LENGTH = 180;

export interface GenerateTradeMessageModalProps {
  prefix: string;
  suffix: string;
  template: string;
  displaySettings?: Record<string, DisplaySettings>;
  items: ItemWithMeta[];
  onSave?: (data: GenerateTradeMessageModalProps) => void;
}

export function GenerateTradeMessageModal({ prefix, template, suffix, displaySettings, items, onSave }: GenerateTradeMessageModalProps) {
  const form = useForm({
    initialValues: {
      prefix: prefix || "",
      suffix: suffix || "",
      template: template || "",
      groupByKey: "",
      displaySettings: displaySettings || ({} as Record<string, DisplaySettings>),
    },
  });

  // Fetch data from rust side
  const { data: chatIcons } = useQuery({
    queryKey: ["cache_chat_icons"],
    queryFn: () => api.cache.getChatIcons(),
  });

  const RenderMessageWithIcons = (message: string) => {
    if (!chatIcons) return <Text size="sm">{message}</Text>;

    const iconMap = chatIcons.reduce((acc, icon) => {
      acc[icon.code] = { url: icon.url, code: icon.code };
      return acc;
    }, {} as Record<string, { url: string; code: string }>);

    // Split message by icon patterns (:iconname:)
    const parts: (string | { type: "icon"; url: string; code: string })[] = [];
    let lastIndex = 0;
    const iconRegex = /:([^:]+):/g;
    let match;

    while ((match = iconRegex.exec(message)) !== null) {
      // Add text before the icon
      if (match.index > lastIndex) {
        parts.push(message.substring(lastIndex, match.index));
      }

      // Add the icon
      const iconCode = `:${match[1]}:`;
      const iconData = iconMap[iconCode];
      if (iconData) {
        parts.push({ type: "icon", url: iconData.url, code: iconData.code });
      } else {
        parts.push(iconCode); // Keep original if icon not found
      }

      lastIndex = match.index + match[0].length;
    }

    // Add remaining text
    if (lastIndex < message.length) {
      parts.push(message.substring(lastIndex));
    }

    return (
      <Text size="sm" component="div" style={{ display: "flex", alignItems: "center", flexWrap: "wrap", gap: "2px" }}>
        {parts.map((part, index) => {
          if (typeof part === "string") {
            return (
              <span key={index} style={{ whiteSpace: "pre-wrap" }}>
                {part}
              </span>
            );
          } else {
            return (
              <img
                key={index}
                src={part.url}
                alt={part.code}
                style={{
                  width: "18px",
                  height: "18px",
                  display: "inline-block",
                  verticalAlign: "middle",
                  margin: "0 1px",
                }}
              />
            );
          }
        })}
      </Text>
    );
  };

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`generate_trade_message.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);
  const useTranslateTitle = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`titles.${key}`, { ...context }, i18Key);

  // State
  const [messagePreview, setMessagePreview] = useState<string>("");
  const [displayData, setDisplayData] = useState<DisplaySettings & { key: string }>({ key: "", prefix: "", suffix: "", value: "" });
  const [availableKeys, setAvailableKeys] = useState<string[]>([]);
  const [chatIcon, setChatIcon] = useState<string>(":alliance:");

  useEffect(() => {
    const generateMessages = async () => {
      let template = form.values.template;
      const displaySettings = form.values.displaySettings;
      const newItems = await GetChatLinkNameMultiple(items, displaySettings);
      let keys: string[] = [];
      for (let val of newItems) keys = [...keys, ...Object.keys(val)];
      setAvailableKeys(Array.from(new Set(keys)));

      let message = form.values.prefix;
      if (form.values.groupByKey) {
        let groupByKey = form.values.groupByKey;
        let groupedItems = GroupByKey(`${groupByKey}.value`, newItems);
        for (let [, items] of Object.entries(groupedItems)) {
          for (let i = 0; i < items.length; i++) {
            if (i != items.length - 1) delete items[i][groupByKey];
            let candidate = ApplyTemplate(template, items[i]);
            if (candidate.length > MAX_LENGTH) return;
            message += candidate;
          }
        }
      } else
        newItems.forEach((data) => {
          let candidate = ApplyTemplate(template, data);
          if (candidate.length > MAX_LENGTH) return;
          message += candidate;
        });

      // Add flair if space allows
      if (message.length <= MAX_LENGTH - form.values.suffix.length) message += form.values.suffix;
      setMessagePreview(message.trim());
    };
    generateMessages();
  }, [items, form.values.template, form.values.displaySettings]);

  const CopyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    notifications.show({
      title: useTranslateCommon("notifications.copy_to_clipboard.title"),
      message: useTranslateCommon("notifications.copy_to_clipboard.message", { message: text }),
      color: "green.7",
    });
  };

  return (
    <Box h={"75%"}>
      <Stack>
        <Grid>
          <Grid.Col span={6}>
            <Title order={3} mt="md">
              {useTranslateTitle("message_settings")}
            </Title>
            <Group grow>
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
            <TextInput
              label={useTranslateFields("template.label")}
              description={useTranslateFields("template.description")}
              placeholder={useTranslateFields("template.placeholder")}
              value={form.values.template}
              onChange={(e) => form.setFieldValue("template", e.currentTarget.value)}
            />
            <Group grow>
              <Select
                searchable
                label={useTranslateFields("group_by.label")}
                description={useTranslateFields("group_by.description")}
                value={form.values.groupByKey}
                onChange={(value) => form.setFieldValue("groupByKey", value || "")}
                data={["none", ...availableKeys].map((key) => ({ label: upperFirst(key), value: key == "none" ? "" : key }))}
              />
              <Select
                searchable
                allowDeselect={false}
                label={useTranslateFields("chat_icon.label")}
                description={useTranslateFields("chat_icon.description")}
                value={chatIcon}
                onChange={(value) => setChatIcon(value || "")}
                data={chatIcons ? chatIcons.map((icon) => ({ label: icon.name, value: icon.code, img: icon.url })) : []}
                renderOption={renderSelectOption}
                leftSection={<Image src={chatIcons?.find((icon) => icon.code === chatIcon)?.url || ""} fit="contain" width={20} height={20} />}
                rightSectionPointerEvents="inherit"
                rightSection={
                  <ActionWithTooltip tooltip={useTranslate("button_copy_tooltip")} icon={faCopy} onClick={() => CopyToClipboard(chatIcon)} />
                }
              />
            </Group>
          </Grid.Col>
          <Grid.Col span={6}>
            <DataTable
              mt={"md"}
              striped
              withColumnBorders
              withTableBorder
              withRowBorders
              height={"30vh"}
              records={Object.entries(form.values.displaySettings).map(([key, value]) => ({ key, ...value }))}
              columns={[
                {
                  accessor: "key",
                  title: useTranslate("datatable.columns.key"),
                  footer: (
                    <Autocomplete
                      size="xs"
                      placeholder={useTranslateFields("key.placeholder")}
                      data={availableKeys}
                      value={displayData.key}
                      onChange={(value) => setDisplayData({ ...displayData, key: value || "" })}
                    />
                  ),
                },
                {
                  accessor: "prefix",
                  title: useTranslate("datatable.columns.prefix"),
                  render: (row) => (
                    <TextInput
                      size="xs"
                      value={row.prefix}
                      onChange={(e) => {
                        form.setFieldValue("displaySettings", {
                          ...form.values.displaySettings,
                          [row.key]: {
                            ...form.values.displaySettings[row.key],
                            prefix: e.currentTarget.value,
                          },
                        });
                      }}
                    />
                  ),
                  footer: (
                    <TextInput
                      size="xs"
                      placeholder={useTranslateFields("prefix.placeholder")}
                      value={displayData.prefix}
                      onChange={(e) => setDisplayData({ ...displayData, prefix: e.currentTarget.value })}
                    />
                  ),
                },
                {
                  accessor: "suffix",
                  title: useTranslate("datatable.columns.suffix"),
                  render: (row) => (
                    <TextInput
                      size="xs"
                      placeholder={useTranslateFields("suffix.placeholder")}
                      value={row.suffix}
                      onChange={(e) => {
                        form.setFieldValue("displaySettings", {
                          ...form.values.displaySettings,
                          [row.key]: {
                            ...form.values.displaySettings[row.key],
                            suffix: e.currentTarget.value,
                          },
                        });
                      }}
                    />
                  ),
                  footer: (
                    <TextInput
                      size="xs"
                      placeholder={useTranslateFields("suffix.placeholder")}
                      value={displayData.suffix}
                      onChange={(e) => setDisplayData({ ...displayData, suffix: e.currentTarget.value })}
                    />
                  ),
                },
                {
                  accessor: "actions",
                  title: useTranslateCommon("datatable_columns.actions.title"),
                  width: 75,
                  render: (row) => (
                    <Group gap={3} justify="center">
                      <ActionWithTooltip
                        tooltip={useTranslateCommon("datatable_columns.actions.buttons.delete_tooltip")}
                        icon={faTrash}
                        color="red"
                        iconProps={{ size: "xs" }}
                        actionProps={{ size: "sm" }}
                        onClick={async (e) => {
                          e.stopPropagation();
                          let currentSettings = { ...form.values.displaySettings };
                          delete currentSettings[row.key];
                          form.setFieldValue("displaySettings", currentSettings);
                        }}
                      />
                    </Group>
                  ),
                  footer: (
                    <Group gap={3} justify="center">
                      <ActionWithTooltip
                        tooltip={useTranslate("button_add_key_tooltip")}
                        icon={faPlus}
                        color="green.7"
                        actionProps={{ size: "sm" }}
                        iconProps={{ size: "xs" }}
                        onClick={() => {
                          if (displayData.key.trim() === "") return;
                          form.setFieldValue("displaySettings", {
                            ...form.values.displaySettings,
                            [displayData.key]: { prefix: displayData.prefix, suffix: displayData.suffix, value: "" },
                          });
                          setDisplayData({ key: "", prefix: "", suffix: "", value: "" });
                        }}
                      />
                    </Group>
                  ),
                },
              ]}
            />
          </Grid.Col>
        </Grid>
        <Divider />
        <Stack gap="xs">
          <TextInput
            readOnly
            maxLength={MAX_LENGTH}
            label={useTranslateFields("message.label")}
            description={useTranslateFields("message.description")}
            value={messagePreview}
            rightSection={
              <ActionWithTooltip tooltip={useTranslate("button_copy_tooltip")} icon={faCopy} onClick={() => CopyToClipboard(messagePreview)} />
            }
          />
          <Text fw={"xs"}>{useTranslateFields("message.length_info", { current: messagePreview.length, max: MAX_LENGTH })}</Text>
          <Box>
            <Title order={5}>{useTranslateTitle("preview_with_icons")}</Title>
            {RenderMessageWithIcons(messagePreview)}
          </Box>
        </Stack>
        <Group justify="space-between">
          {onSave && (
            <Button
              onClick={() => {
                onSave({
                  prefix: form.values.prefix,
                  suffix: form.values.suffix,
                  template: form.values.template,
                  displaySettings: form.values.displaySettings,
                  items: [],
                });
                notifications.show({
                  title: useTranslateCommon("notifications.update_settings.success.title"),
                  message: useTranslateCommon("notifications.update_settings.success.message"),
                  color: "green.7",
                });
              }}
            >
              {useTranslateCommon("buttons.save.label")}
            </Button>
          )}
        </Group>
      </Stack>
    </Box>
  );
}

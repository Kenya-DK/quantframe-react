import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { Box, TextInput, Text, Grid, Group, Stack, Divider, Title, Select, Image, Autocomplete, Tabs } from "@mantine/core";
import { useForm } from "@mantine/form";
import { ActionWithTooltip } from "../../Shared/ActionWithTooltip";
import { faBook, faCopy, faPlus, faSave, faTrash } from "@fortawesome/free-solid-svg-icons";
import { notifications } from "@mantine/notifications";
import { DisplaySettings, GetChatLinkNameMultiple, ApplyTemplate, GroupByKey } from "@utils/helper";
import { useEffect, useState } from "react";
import { ItemWithMeta, TauriTypes } from "$types";
import { DataTable } from "mantine-datatable";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { RenderMessageWithIcons, renderSelectOption } from "./helpeer";
import { upperFirst } from "@mantine/hooks";
import { useMutations } from "./mutations";
import { useModals } from "./modals";
const MAX_LENGTH = 180;

export interface GenerateTradeMessageModalProps {
  prefix: string;
  suffix: string;
  template: string;
  displaySettings?: Record<string, DisplaySettings>;
  items: ItemWithMeta[];
  settings?: TauriTypes.GenerateTradeMessageSettings;
  onSave?: (data: GenerateTradeMessageModalProps) => void;
}

export function GenerateTradeMessageModal({ prefix, template, suffix, displaySettings, items }: GenerateTradeMessageModalProps) {
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

  // Fetch settings from rust side
  const { data: settings, refetch } = api.app.get_settings();

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
  const [selectedTemplate, setSelectedTemplate] = useState<TauriTypes.SaveTemplateSetting | null>(null);

  const ValidateLength = (candidate: string, message: string) => {
    let totalLength = candidate.length + message.length;
    return totalLength <= MAX_LENGTH;
  };

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
            if (!ValidateLength(candidate, message)) return;
            message += candidate;
          }
        }
      } else
        newItems.forEach((data) => {
          let candidate = ApplyTemplate(template, data);
          if (!ValidateLength(candidate, message)) return;
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
  // Mutations
  const { createMutation, deleteMutation } = useMutations({ refetchQueries: () => refetch(), setLoadingRows: () => {} });
  // Modals
  const { OpenSaveModal, OpenDeleteModal } = useModals({ createMutation, deleteMutation });
  // const OpenDeleteModal = () => {
  //   modals.openConfirmModal({
  //     title: useTranslateCommon("prompts.delete_item.title"),
  //     children: <Text size="sm">{useTranslateCommon("prompts.delete_item.message", { count: 1 })}</Text>,
  //     labels: { confirm: useTranslateCommon("prompts.delete_item.confirm"), cancel: useTranslateCommon("prompts.delete_item.cancel") },
  //     onConfirm: async () => {
  //       if (!selectedTemplate || !settings) return;
  //       const filteredTemplates = settings?.generate_trade_message.templates.filter((t) => t.name !== selectedTemplate.name) || [];
  //       api.app.updateSettings({
  //         ...settings,
  //         generate_trade_message: {
  //           templates: filteredTemplates,
  //         },
  //       });
  //     },
  //   });
  // };
  return (
    <Box h={"75%"}>
      <Stack>
        <Grid>
          <Grid.Col span={8}>
            <Tabs defaultValue="message_settings">
              <Tabs.List>
                <Tabs.Tab value="message_settings">{useTranslateTitle("message_settings")}</Tabs.Tab>
                <Tabs.Tab value="display_settings">{useTranslateTitle("display_settings")}</Tabs.Tab>
              </Tabs.List>
              <Tabs.Panel value="message_settings">
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
                  <Select
                    searchable
                    label={useTranslateFields("group_by.label")}
                    description={useTranslateFields("group_by.description")}
                    value={form.values.groupByKey}
                    onChange={(value) => form.setFieldValue("groupByKey", value || "")}
                    data={["none", ...availableKeys].map((key) => ({ label: upperFirst(key), value: key == "none" ? "" : key }))}
                  />
                </Group>
                <TextInput
                  label={useTranslateFields("template.label")}
                  description={useTranslateFields("template.description")}
                  placeholder={useTranslateFields("template.placeholder")}
                  value={form.values.template}
                  onChange={(e) => form.setFieldValue("template", e.currentTarget.value)}
                />
              </Tabs.Panel>
              <Tabs.Panel value="display_settings">
                <DataTable
                  mt={"md"}
                  striped
                  withColumnBorders
                  withTableBorder
                  withRowBorders
                  idAccessor={"key"}
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
              </Tabs.Panel>
            </Tabs>
          </Grid.Col>
          <Grid.Col span={4}>
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
            <Select
              searchable
              label={useTranslateFields("load_template.label")}
              description={useTranslateFields("load_template.description")}
              value={selectedTemplate?.name || ""}
              onChange={(value) => {
                const template = settings?.generate_trade_message.templates.find((t) => t.name === value) || null;
                setSelectedTemplate(template);
              }}
              data={settings?.generate_trade_message.templates.map((template) => ({ label: template.name, value: template.name })) || []}
              rightSectionPointerEvents="inherit"
              rightSectionWidth={35 * 3}
              rightSection={
                <Group gap={3}>
                  <ActionWithTooltip
                    tooltip={useTranslate("button_delete_template_tooltip")}
                    icon={faTrash}
                    color="red"
                    actionProps={{ disabled: selectedTemplate?.name === "Default" || selectedTemplate == null, size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={() => {
                      if (!selectedTemplate || !settings) return;
                      OpenDeleteModal(settings, selectedTemplate.name);
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslate("button_load_template_tooltip")}
                    icon={faBook}
                    actionProps={{ disabled: selectedTemplate == null, size: "sm" }}
                    iconProps={{ size: "xs" }}
                    onClick={() => {
                      if (!selectedTemplate) return;
                      console.log("Loading template:", selectedTemplate);
                      form.setValues({ ...form.values, ...selectedTemplate });
                    }}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslate("button_save_template_tooltip")}
                    icon={faSave}
                    actionProps={{ size: "sm" }}
                    onClick={() => {
                      if (!settings) return;
                      OpenSaveModal({ settings, template: form.values });
                    }}
                  />
                </Group>
              }
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
            {RenderMessageWithIcons(messagePreview, chatIcons || [])}
          </Box>
        </Stack>
      </Stack>
    </Box>
  );
}

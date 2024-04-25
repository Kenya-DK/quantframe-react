import { PaperProps, Card, Group, Text, Tooltip, ActionIcon, Divider, TextInput, Collapse } from '@mantine/core';
import { SettingsNotification } from '@api/types';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useTranslateForms } from '@hooks/index';
import { faBell } from '@fortawesome/free-solid-svg-icons';
import { faDiscord } from '@fortawesome/free-brands-svg-icons';
import { TooltipIcon } from '../../TooltipIcon';

export type NotificationFormProps = {
  title: string;
  value: SettingsNotification;
  onChanges: (values: SettingsNotification) => void;
  paperProps?: PaperProps;
}


export function NotificationForm({ onChanges, title, value }: NotificationFormProps) {


  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`notification.${key}`, { ...context }, i18Key)
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)

  return (
    <Card shadow="xs">
      <Group justify="space-between" mt="md" mb="xs">
        <Text fw={500}>{title}</Text>
        <Group>
          <Tooltip position="top" label={useTranslateButtons('system.tooltip')}>
            <ActionIcon color={value.system_notify ? "green.7" : "blue.7"} variant="filled" onClick={() => {
              onChanges({ ...value, system_notify: !value.system_notify })
            }} >
              <FontAwesomeIcon icon={faBell} />
            </ActionIcon>
          </Tooltip>
          <Tooltip position="top" label={useTranslateButtons('discord.tooltip')}>
            <ActionIcon color={value.discord_notify ? "green.7" : "blue.7"} variant="filled" onClick={() => onChanges({ ...value, discord_notify: !value.discord_notify })} >
              <FontAwesomeIcon icon={faDiscord} />
            </ActionIcon>
          </Tooltip>
        </Group>
      </Group>
      <Divider />
      <Collapse in={value.discord_notify || value.system_notify}>
        <TextInput
          required
          label={useTranslateFormFields('title.label')}
          placeholder={useTranslateFormFields('title.placeholder')}
          value={value.title}
          onChange={(event) => onChanges({ ...value, title: event.currentTarget.value })}
          radius="md"
        />
        <TextInput
          required
          label={useTranslateFormFields('content.label')}
          placeholder={useTranslateFormFields('content.placeholder')}
          value={value.content}
          onChange={(event) => onChanges({ ...value, content: event.currentTarget.value })}
          radius="md"
        />
        {value.discord_notify && (
          <Group grow>
            <TextInput
              required
              label={useTranslateFormFields('webhook.label')}
              placeholder={useTranslateFormFields('webhook.placeholder')}
              value={value.webhook}
              rightSection={<TooltipIcon label={useTranslateFormFields('webhook.description')} />}
              onChange={(event) => onChanges({ ...value, webhook: event.currentTarget.value })}
              radius="md"
            />
            <TextInput
              required
              label={useTranslateFormFields('user_ids.label')}
              placeholder={useTranslateFormFields('user_ids.placeholder')}
              value={value.user_ids}
              rightSection={<TooltipIcon label={useTranslateFormFields('user_ids.description')} />}
              onChange={(event) => onChanges({ ...value, user_ids: event.currentTarget.value.split(',').map((v) => v.trim()) })}
              radius="md"
            />
          </Group>
        )}
      </Collapse>
    </Card>
  );
}
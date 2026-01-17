import { useTranslateForms } from "@hooks/useTranslate.hook";

type TranslateFn = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => string;

const createTranslator = (prefix: string): TranslateFn => {
  return (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`${prefix}.${key}`, { ...context }, i18Key);
};

export const createEditNotificationSettingTranslations = () => {
  return {
    form: createTranslator("edit_notification_setting"),
    systemFields: createTranslator("edit_notification_setting.system.fields"),
    discordFields: createTranslator("edit_notification_setting.discord.fields"),
    webhookFields: createTranslator("edit_notification_setting.webhook.fields"),
    manageSounds: createTranslator("edit_notification_setting.manage_sounds"),
  };
};

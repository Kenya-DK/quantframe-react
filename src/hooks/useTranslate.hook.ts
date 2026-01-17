import i18next from "i18next"

export const useTranslateGeneral = (key: string, context?: { [key: string]: any }, i18Key?: boolean): string => i18Key ? `general.${key}` : i18next.t(`general.${key}`, { ...context }) as string
export const useTranslateComponent = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `components.${key}` : i18next.t(`components.${key}`, { ...context }) as string
export const useTranslateEnums = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `enums.${key}` : i18next.t(`enums.${key}`, { ...context }) as string

export const useTranslateForms = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`forms.${key}`, { ...context }, i18Key)
export const useTranslateDataGrid = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`dataTable.${key}`, { ...context }, i18Key) as string

export const useTranslateLayouts = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `layout.${key}` : i18next.t(`layout.${key}`, { ...context }) as string

export const useTranslatePages = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `pages.${key}` : i18next.t(`pages.${key}`, { ...context }) as string

export const useTranslateModals = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`modals.${key}`, { ...context }, i18Key) as string

export const useTranslateContexts = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `context.${key}` : i18next.t(`context.${key}`, { ...context }) as string

export const useTranslateSockets = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `sockets.${key}` : i18next.t(`sockets.${key}`, { ...context }) as string

export const useTranslateNotifications = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `notifications.${key}` : i18next.t(`notifications.${key}`, { ...context }) as string

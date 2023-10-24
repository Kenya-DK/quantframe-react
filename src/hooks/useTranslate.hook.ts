import i18next from "i18next"

export const useTranslateGeneral = (key: string, context?: { [key: string]: any }, i18Key?: boolean): string => i18Key ? `general.${key}` : i18next.t(`general.${key}`, { ...context }) as string
export const useTranslateComponent = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `components.${key}` : i18next.t(`components.${key}`, { ...context }) as string

export const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`forms.${key}`, { ...context }, i18Key)
export const useTranslateDataGrid = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`dataTable.${key}`, { ...context }, i18Key) as string

export const useTranslateLayout = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `layout.${key}` : i18next.t(`layout.${key}`, { ...context }) as string

export const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `pages.${key}` : i18next.t(`pages.${key}`, { ...context }) as string

export const useTranslateModal = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`modals.${key}`, { ...context }, i18Key) as string

export const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `success.${key}` : i18next.t(`success.${key}`, { ...context }) as string
export const useTranslateError = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `error.${key}` : i18next.t(`error.${key}`, { ...context }) as string

export const useTranslateContext = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => i18Key ? `context.${key}` : i18next.t(`context.${key}`, { ...context }) as string
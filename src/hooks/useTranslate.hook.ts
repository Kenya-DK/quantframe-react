import i18next from "i18next"

export const useTranslateGeneral = (key: string, context?: { [key: string]: any }): string => i18next.t(`general.${key}`, { ...context }) as string
export const useTranslateComponent = (key: string, context?: { [key: string]: any }) => i18next.t(`components.${key}`, { ...context }) as string

export const useTranslateForm = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`forms.${key}`, { ...context })
export const useTranslateDataGrid = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`dataTable.${key}`, { ...context }) as string

export const useTranslateLayout = (key: string, context?: { [key: string]: any }) => i18next.t(`layout.${key}`, { ...context }) as string

export const useTranslatePage = (key: string, context?: { [key: string]: any }) => i18next.t(`pages.${key}`, { ...context }) as string

export const useTranslateModal = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`modals.${key}`, { ...context }) as string

export const useTranslateSuccess = (key: string, context?: { [key: string]: any }) => i18next.t(`success.${key}`, { ...context }) as string
export const useTranslateError = (key: string, context?: { [key: string]: any }) => i18next.t(`error.${key}`, { ...context }) as string
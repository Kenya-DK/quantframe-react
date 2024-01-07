export interface ProgressReport {
  id: string,
  title: string,
  i18n_key: string,
  values: { [key: string]: any },
  isCompleted: boolean,
}
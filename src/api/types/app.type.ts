export interface AppInfo {
  authors: string;
  description: string;
  name: string;
  version: string;
  is_pre_release: boolean;
  is_development: boolean;
}
export interface OnToggleControlPayload {
  id: string;
  state: boolean;
}

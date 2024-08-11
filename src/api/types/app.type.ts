
export interface AppInfo {
  authors: string;
  description: string;
  name: string;
  version: string;
  is_pre_release: boolean;
}
export interface OnToggleControlPayload {
  id: string;
  state: boolean;
}
export interface User {
  anonymous: boolean;
  auctions_limit: number;
  avatar?: string;
  check_code: string;
  id: string;
  ingame_name: string;
  locale: string;
  order_limit: number;
  platform: string;
  qf_access_token: string;
  qf_banned: boolean;
  qf_banned_reason?: string;
  qf_banned_until?: string;
  region: string;
  role: Role;
  status: UserStatus;
  unread_messages: number;
  verification: boolean;
  wfm_access_token: string;
  wfm_banned: boolean;
  wfm_banned_reason?: string;
  wfm_banned_until?: string;
}
export interface Role {
  can_managed: boolean;
  created_at: Date;
  id: string;
  name: string;
  permissions: string;
  updated_at: Date;
}
export enum UserStatus {
  Online = "online",
  Invisible = "invisible",
  Ingame = "ingame",
}

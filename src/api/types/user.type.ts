export interface User {
  auctions_limit: number;
  avatar: string;
  check_code: string;
  id: string;
  ingame_name: string;
  locale: string;
  order_limit: number;
  platform: string;
  qf_access_token: string;
  qf_banned: boolean;
  region: string;
  role: Role;
  status: UserStatus;
  wfm_access_token: string;
  wfm_banned: boolean;
  authorized: boolean;
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
  Online = 'online',
  Invisible = 'invisible',
  Ingame = 'ingame',
}

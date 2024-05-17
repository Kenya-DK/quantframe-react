export interface User {
  access_token: null;
  anonymous: boolean;
  auctions_limit: number;
  avatar: null;
  check_code: string;
  id: string;
  ingame_name: string;
  locale: string;
  order_limit: number;
  platform: string;
  qf_access_token: null;
  qf_banned: boolean;
  region: string;
  role: string;
  verification: boolean;
  wfm_access_token: null;
  wfm_banned: boolean;
  status: UserStatus;
}

export enum UserStatus {
  Online = 'online',
  Invisible = 'invisible',
  Ingame = 'ingame',
}

export interface TopLevel {
  anonymous: boolean;
  auctions_limit: number;
  avatar: null;
  check_code: string;
  id: string;
  ingame_name: string;
  locale: string;
  order_limit: number;
  platform: string;
  qf_access_token: null;
  qf_banned: boolean;
  region: string;
  role: string;
  status: string;
  verification: boolean;
  wfm_access_token: null;
  wfm_banned: boolean;
}

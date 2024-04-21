export interface User {
  access_token: null;
  auctions_limit: number;
  avatar: string;
  banned: boolean;
  id: string;
  ingame_name: string;
  locale: string;
  order_limit: number;
  platform: string;
  region: string;
  role: string;
  status: UserStatus;
}

export enum UserStatus {
  Online = 'online',
  Invisible = 'invisible',
  Ingame = 'ingame',
}
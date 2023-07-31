export namespace Wfm {

  export interface User {
    // unread_messages: number
    // has_mail: number
    // check_code: string,
    // written_reviews: string,
    // verification: boolean,
    ingame_name: string,
    avatar: string,
    // anonymous: boolean,
    platform: 'pc',
    // reputation: number,
    // linked_accounts: {}
    id: string,
    region: 'en' | (string & {}),
    locale: 'en' | (string & {}),
    // background: string|null,
    role: 'user',
    // avatar: string,
    banned: boolean
  }

  export interface LinkedAccounts {
    steam_profile: boolean;
    patreon_profile: boolean;
    xbox_profile: boolean;
  }

  export interface PatreonProfile {
    patreon_founder: boolean;
    subscription: boolean;
    patreon_badge: string;
  }

}
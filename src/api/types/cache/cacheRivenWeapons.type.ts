export interface CacheRivenWeapon {
  disposition: number;
  godRoll: RivenGodRoll;
  name: string;
  riven_type: string;
  uniqueName: string;
  upgrade_type: string;
  wfm_group: string;
  wfm_icon: string;
  wfm_icon_format: string;
  wfm_id: string;
  wfm_thumb: string;
  wfm_url_name: string;
}

export interface RivenGodRoll {
  good_rolls: RivenGoodRoll[];
  negative_attributes: string[];
  weapon_url_name: string;
}

export interface RivenGoodRoll {
  optional: string[];
  required: string[];
}

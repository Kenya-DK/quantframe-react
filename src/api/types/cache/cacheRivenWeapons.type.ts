
export interface CacheRivenWeapon {
  wfm_id: string;
  wfm_url_name: string;
  wfm_group: string;
  riven_type: string;
  wfm_icon: string;
  wfm_icon_format: string;
  wfm_thumb: string;
  uniqueName: string;
  i18n: Record<string, I18N>;
}

export interface I18N {
  name: string;
}

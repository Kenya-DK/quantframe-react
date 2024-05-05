export interface CacheRivenDataByRivenInternalID {
  rivenInternalID: string;
  veiledName: string;
  baseDrain: number;
  fusionLimit: number;
  challenges: Record<string, CacheRivenChallenges>;
  rivenStats: Record<string, CacheRivenRivenStat>;
}

export interface CacheRivenChallenges {
  challengeUID: string;
  description: string;
  complications: Record<string, CacheRivenComplication>;
}

export interface CacheRivenComplication {
  complicationID: string;
  description: string;
}

export interface CacheRivenRivenStat {
  wfm_id: string;
  modifierTag: string;
  prefixTag: string;
  suffixTag: string;
  baseValue: number;
  localizationString: string;
  shortString: string;
}

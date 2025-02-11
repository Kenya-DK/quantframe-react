import { SubType } from "./subType.type";

export interface SyndicatesPrice {
  location: string;
  standing: number;
  name: string;
  wfm_id: string;
  wfm_url_name: string;
  min_price: number;
  max_price: number;
  avg_price: number;
  volume: number;
  datetime: Date;
  sub_type: SubType;
}
export enum Syndicate {
  ArbitersOfHexis = "arbiters_of_hexis",
  NecraLoid = "necraloid",
  NewLoka = "new_loka",
  TheHex = "the_hex",
  KahlsGarrison = "kahl's_garrison",
  OperationalSupply = "operational_supply",
  Cavia = "cavia",
  SolarisUnited = "solaris_united",
  TheQuills = "the_quills",
  CephalonSimaris = "cephalon_simaris",
  Entrati = "entrati",
  CephalonSuda = "cephalon_suda",
  ThePerrinSequence = "the_perrin_sequence",
  TheHoldfasts = "the_holdfasts",
  Ventkids = "ventkids",
  SteelMeridian = "steel_meridian",
  RedVeil = "red_veil",
  Ostron = "ostron",
  VoxSolaris = "vox_solaris",
  Conclave = "conclave",
}

export interface ItemPrice {
  datetime: Date;
  volume: number;
  min_price: number;
  max_price: number;
  avg_price: number;
  wa_price: number;
  median: number;
  order_type: string;
  moving_avg: number;
  id: string;
  trading_tax: number;
  mod_rank: number;
}

export interface ItemPriceChart {
  labels: string[];
  volume_chart: number[];
  min_price_chart: number[];
  max_price_chart: number[];
  avg_price: number[];
  median_price_chart: number[];
  moving_avg_chart: number[];
}

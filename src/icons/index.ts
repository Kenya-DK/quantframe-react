export * from "./faAmberStar";
export * from "./facWarframeMarket";
export * from "./faCyanStar";
export * from "./faInfinity";
export * from "./faMoneyBillTrendDown";
export * from "./faPlat";
export * from "./faTradingAnalytics";
export * from "./faWebHook";
export * from "./polarity";

import { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import {
  faPolarityMadurai,
  faPolarityNaramon,
  faPolarityVazarin,
  faPolarityZenuri,
  faPolarityUnairu,
  faPolarityUmbra,
  faPolarityPenjaga,
  faPolarityAura,
} from "./polarity";
import { faCross } from "@fortawesome/free-solid-svg-icons";

export const POLARITY_ICON_BY_TYPE: Record<string, IconDefinition> = {
  madurai: faPolarityMadurai,
  naramon: faPolarityNaramon,
  vazarin: faPolarityVazarin,
  zenuri: faPolarityZenuri,
  unairu: faPolarityUnairu,
  umbra: faPolarityUmbra,
  penjaga: faPolarityPenjaga,
  aura: faPolarityAura,
};

export function getPolarityIcon(polarity: string): IconDefinition {
  return POLARITY_ICON_BY_TYPE[polarity] ?? faCross;
}

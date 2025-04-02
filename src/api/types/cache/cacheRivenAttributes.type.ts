export interface CacheRivenAttribute {
  id: string;
  gameRef: string;
  group: string;
  prefix: string;
  suffix: string;
  effect: string;
  url_name: string;
  unit?: string;
  exclusiveTo?: string[];
  positiveIsNegative?: boolean;
  positiveOnly?: boolean;
  negativeOnly?: boolean;
}

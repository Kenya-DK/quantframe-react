export type IOperatorType = "eq" | "neq" | "gt" | "gteq" | "lt" | "lteq" | "in" | "nin" | "like" | "nlike" | "isnull" | "isnotnull" | "between" | "nbetween" | "empty" | "nempty";
export type IPropertyType = "string" | "number" | "boolean" | "date" | "array" | "object" | "any";

export interface ISearchKeyParameter {
  [key: string]: ISearchParameter;
}

export interface ISearchParameter {
  type?: IPropertyType;
  filters: ISearchFilter[];
  orFilters?: ISearchOrParameter[];
}

export interface ISearchFilter {
  operator: IOperatorType;
  value: any;
  isCaseSensitive?: boolean;
}

export interface ISearchOrParameter extends Omit<ISearchParameter, 'orFilters'> {
  code: string;
}
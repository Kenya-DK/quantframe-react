// Filter Types
export enum OperatorType {
  STRING = "string",
  NUMBER = "number",
  BOOLEAN = "boolean",
  DATE = "date",
  ARRAY = "array",
  OBJECT = "object",
  ANY = "any",
}
export enum Operator {
  EQUALS = "equals", // Equals
  NOT_EQUALS = "not_equals", // Not equals
  GREATER_THAN = "greater_than", // Greater than
  GREATER_OR_EQUAL = "greater_or_equal", // Greater than or equals
  LESS_THAN = "less_than", // Less than
  LESS_OR_EQUAL = "less_or_equal", // Less than or equals
  IN_ARRAY = "in_array", // In array
  NOT_IN_ARRAY = "not_in_array", // Not in array
  MATCHES = "matches", // Like pattern
  NOT_MATCHES = "not_matches", // Not like pattern
  IS_NULL = "is_null", // Is null
  IS_NOT_NULL = "is_not_null", // Is not null
  BETWEEN_VALUES = "between_values", // Between two values
  NOT_BETWEEN_VALUES = "not_between_values", // Not between two values
  IS_EMPTY = "is_empty", // Is empty
  IS_NOT_EMPTY = "is_not_empty", // Is not empty
  CONTAINS_VALUE = "contains_value", // Contains
  DOES_NOT_CONTAIN = "does_not_contain", // Not contains
  STARTS_WITH = "starts_with", // Starts with
  DOES_NOT_START_WITH = "does_not_start_with", // Not starts with
  ENDS_WITH = "ends_with", // Ends with
  DOES_NOT_END_WITH = "does_not_end_with", // Not ends with
  IS_STRICTLY_EMPTY = "is_strictly_empty", // Is strictly empty
  IS_STRICTLY_NOT_EMPTY = "is_strictly_not_empty", // Is strictly not empty
  OUTSIDE_RANGE = "outside_range", // Not between two values (alternative to NOT_BETWEEN)
}

// A mapping of each Operator to a corresponding FilterCriteria or supported data types
export type OperatorConditionMap = Partial<{
  [key in Operator]: string | number | boolean | Date | Array<string | number | boolean | Date>;
}> & {
  type?: OperatorType; // Specifies the type of the field (e.g., string, number)
  isCaseSensitive?: boolean; // Optional case-sensitivity
  combineFields?: string[]; // Optional fields to combine for Operator.COMBINE
  combineWith?: string; // Optional field to combine with for Operator.COMBINE
};

// Represents a dynamic structure for filter queries using keys and their conditions
export interface FieldFilter {
  [fieldName: string]: OperatorConditionMap;
}

// Defines the structure for complex filter conditions
export type ComplexFilter = {
  AND?: FieldFilter[]; // Nested group of AND conditions
  OR?: FieldFilter[]; // Nested group of OR conditions
};

const CombineFields = (item: any, fields: string[], combineWith: string): string => {
  return fields.map((field) => GetNestedValue(item, field) ?? "").join(combineWith);
};
// Filter Helper Functions
const ConvertToType = (value: any, type: OperatorType | undefined): any => {
  if (value === undefined || value === null) return value;
  switch (type) {
    case OperatorType.STRING:
      return value.toString();
    case OperatorType.NUMBER:
      return Number(value);
    case OperatorType.BOOLEAN:
      return Boolean(value);
    case OperatorType.DATE:
      return new Date(value);
    default:
      return value;
  }
};

const CompareValue = (value: any, filterValue: any, operation: Operator): boolean => {
  switch (operation) {
    case Operator.EQUALS:
      return value == filterValue;
    case Operator.GREATER_THAN:
      return value > filterValue;
    case Operator.GREATER_OR_EQUAL:
      return value >= filterValue;
    case Operator.LESS_THAN:
      return value < filterValue;
    case Operator.LESS_OR_EQUAL:
      return value <= filterValue;
    case Operator.IN_ARRAY:
      return value.includes(filterValue);
    case Operator.NOT_IN_ARRAY:
      return !value.includes(filterValue);
    case Operator.MATCHES:
      return new RegExp(filterValue, "i").test(value);
    case Operator.NOT_MATCHES:
      return new RegExp(filterValue, "i").test(value);
    case Operator.IS_NULL:
      return value == null;
    case Operator.IS_NOT_NULL:
      return value != null;
    case Operator.BETWEEN_VALUES:
      return value >= filterValue[0] && value <= filterValue[1];
    case Operator.NOT_BETWEEN_VALUES:
      return value < filterValue[0] || value > filterValue[1];
    case Operator.IS_EMPTY:
      return value == "";
    case Operator.IS_NOT_EMPTY:
      return value != "";
    case Operator.CONTAINS_VALUE:
      return value.includes(filterValue);
    case Operator.DOES_NOT_CONTAIN:
      return !value.includes(filterValue);
    case Operator.STARTS_WITH:
      return value.startsWith(filterValue);
    case Operator.DOES_NOT_START_WITH:
      return !value.startsWith(filterValue);
    case Operator.ENDS_WITH:
      return value.endsWith(filterValue);
    case Operator.DOES_NOT_END_WITH:
      return !value.endsWith(filterValue);
    case Operator.IS_STRICTLY_EMPTY:
      return value == null || value == undefined;
    case Operator.IS_STRICTLY_NOT_EMPTY:
      return value != null && value != undefined;
    case Operator.OUTSIDE_RANGE:
      return value < filterValue[0] || value > filterValue[1];
    default:
      return false;
  }
};

const CompareValues = (value: any, filterValues: OperatorConditionMap): boolean => {
  const excludeProperties = ["type", "isCaseSensitive", "combineFields", "combineWith"];
  const operators = Object.keys(filterValues).filter((operator) => !excludeProperties.includes(operator));
  return operators.every((operator) => {
    let fValue = filterValues[operator as keyof OperatorConditionMap] as any;

    // Handle case sensitivity for string values
    if (!filterValues.isCaseSensitive) {
      if (typeof fValue === "string") fValue = fValue.toString().toLowerCase();
      if (Array.isArray(fValue)) fValue = fValue.map((v) => v.toString().toLowerCase());
      if (typeof value === "string") value = value.toString().toLowerCase();
      if (Array.isArray(value)) value = value.map((v) => v.toString().toLowerCase());
    }
    return CompareValue(value, fValue, operator as Operator);
  });
};

const GetNestedValue = (item: any, propertyName: string): any => {
  if (!propertyName.includes(".")) return item[propertyName];

  const properties = propertyName.split(".");
  let value = item;
  for (const property of properties) {
    value = value[property];
    if (!value) break;
    if (Array.isArray(value)) {
      return value.map((v) => GetNestedValue(v, properties.slice(1).join(".")));
    } else if (typeof value === "object") {
      return GetNestedValue(value, properties.slice(1).join("."));
    }
  }
  return value;
};
export const ApplyFilter = <T>(items: T[], filter: ComplexFilter): T[] => {
  return items.filter((item) => {
    if (!filter) return true;
    const andFlag = ProcessANDFilter(item, filter);
    const orFlag = ProcessORFilter(item, filter);
    return andFlag && orFlag;
  });
};

const ProcessANDFilter = <T>(item: T, filter: ComplexFilter): boolean => {
  if (!filter.AND || filter.AND.length === 0) return true;
  return filter.AND.every((filterValue) => {
    return ProcessFilterConditions(item, filterValue);
  });
};

const ProcessORFilter = <T>(item: T, filter: ComplexFilter): boolean => {
  if (!filter.OR || filter.OR.length === 0) return true;
  return filter.OR.some((filterValue) => {
    return ProcessFilterConditions(item, filterValue);
  });
};

const ProcessFilterConditions = <T>(item: T, filterValue: FieldFilter): boolean => {
  return Object.entries(filterValue).every(([filterName, filterValue]) => {
    let propertyValue = GetNestedValue(item, filterName);
    if (filterValue.combineFields) propertyValue = CombineFields(item, filterValue.combineFields, filterValue.combineWith || "");
    if (!propertyValue) return false;
    propertyValue = ConvertToType(propertyValue, filterValue.type);

    return CompareValues(propertyValue, filterValue);
  });
};

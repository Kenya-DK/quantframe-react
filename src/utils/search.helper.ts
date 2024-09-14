import { IOperatorType, IPropertyType, ISearchFilter, ISearchOrParameter, ISearchParameter, ISearchKeyParameter } from "../types";

export const validateSearchParameter = (searchParams: ISearchKeyParameter): string | null => {
  for (const key in searchParams) {
    if (Object.prototype.hasOwnProperty.call(searchParams, key)) {
      const searchParam: ISearchParameter = searchParams[key];
      // Validate type
      const validTypes: IPropertyType[] = ["string", "number", "boolean", "date", "array", "object", "any"];
      if (searchParam.type && !validTypes.includes(searchParam.type))
        return `Type ${searchParam.type} is invalid for key ${key}`;

      // Validate filters
      if (!Array.isArray(searchParam.filters) || searchParam.filters.length === 0)
        return `Filters must be an array with at least one filter for key ${key}`;
      for (const filter of searchParam.filters) {
        const errorMessage = validateSearchFilter(filter);
        if (errorMessage)
          return errorMessage;
      }

      // Validate orFilters
      if (searchParam.orFilters && !Array.isArray(searchParam.orFilters))
        return `OrFilters must be an array for key ${key}`;
      if (searchParam.orFilters) {
        for (const orParam of searchParam.orFilters) {
          const errorMessage = validateSearchOrParameter(orParam);
          if (errorMessage)
            return errorMessage;
        }
      }
    }
  }
  return null;
}
export const validateSearchOrParameter = (searchParam: ISearchOrParameter): string | null => {
  // Validate type
  const validTypes: IPropertyType[] = ["string", "number", "boolean", "date", "array", "object", "any"];
  if (searchParam.type && !validTypes.includes(searchParam.type))
    return `Type ${searchParam.type} is invalid for key ${searchParam.code}`;

  // Validate filters
  if (!Array.isArray(searchParam.filters) || searchParam.filters.length === 0)
    return `Filters must be an array with at least one filter for key ${searchParam.code}`;
  for (const filter of searchParam.filters) {
    const errorMessage = validateSearchFilter(filter);
    if (errorMessage)
      return errorMessage;
  }
  return null;
}
export const validateSearchFilter = (searchFilter: ISearchFilter): string | null => {
  // Validate operator
  const validOperators: IOperatorType[] = ["eq", "neq", "gt", "gteq", "lt", "lteq", "in", "nin", "like", "nlike", "isnull", "isnotnull", "between", "nbetween", "empty", "nempty"];
  if (!validOperators.includes(searchFilter.operator))
    return `Operator ${searchFilter.operator} is invalid`;
  else
    return null;
}

export const searchByProperty = <T>(data: T[], propertyName: string, query: ISearchParameter): T[] => {
  return data.filter((item: T) => {
    let found = false;
    for (let filter of query.filters) {
      let property = (item as any)[propertyName];
      if (propertyName.includes(".")) {
        const properties = propertyName.split(".");
        property = (item as any)[properties[0]];
        for (let i = 1; i < properties.length; i++) {
          if (Array.isArray(property) || property.length === 0)
            property = property.map((item: any) => item[properties[i]]);
          else
            property = property[properties[i]];
          if (property === undefined) return false;
        }
      }

      found = compareValue(property, query.type, filter);
      if (query.orFilters && query.orFilters.length > 0 && !found) {

        return query.orFilters.some((orFilter) => {
          const orFound = searchByProperty([item], orFilter.code, { type: orFilter.type, filters: orFilter.filters }).length > 0;
          return orFound;
        });
      } else if (!found) break;
    }
    return found;
  });
}
export const searchByProperties = <T>(data: T[], queys: ISearchKeyParameter): T[] => {
  const keys = Object.keys(queys);
  const queryValues = Object.values(queys);
  return data.filter((item: T) => {
    let result = true;
    for (let index = 0; index < queryValues.length; index++) {
      const property = keys[index];
      const propertyValue = queryValues[index];
      if (searchByProperty([item], property, propertyValue).length <= 0) { result = false; break; }
    }
    return result;
  });
}
export const compareValue = (property: any, propertyType: IPropertyType | undefined, filter: ISearchFilter): boolean => {
  if (property === undefined) return false;
  switch (propertyType) {
    case "date":
      property = new Date(property);
      filter.value = new Date(filter.value);
      // Validate date
      if (property.toString() === "Invalid Date")
        throw new Error(`Invalid date for property ${property}`);

      // Validate date
      if (filter.value.toString() === "Invalid Date")
        throw new Error(`Invalid date for filter ${filter.value}`);
      break;
    case "number":
      property = Number(property);
      break;
    case "boolean":
      property = Boolean(property);
      break;
    case "array":
      property = Array(property);
      break;
    case "object":
      property = Object(property);
      break;
    case "string":
      property = String(property);
      if (!filter.isCaseSensitive) property = property.toLowerCase();
      break;
  }

  if (!filter.isCaseSensitive) {
    // Convert value to lower case
    if (typeof filter.value === "string")
      filter.value = filter.value.toLowerCase();
    if (Array.isArray(filter.value))
      filter.value = filter.value.map((item: any) => {
        if (typeof item === "string")
          return item.toLowerCase()
        if (typeof item === "number")
          return Number(item);
        if (typeof item === "boolean")
          return Boolean(item);
        return item;
      });
    // Convert property to lower case
    if (typeof property === "string")
      property = property.toLowerCase();
    if (Array.isArray(property))
      property = property.map((item: any) => {
        if (typeof item === "string")
          return item.toLowerCase()
        if (typeof item === "number")
          return Number(item);
        if (typeof item === "boolean")
          return Boolean(item);
        return item;
      });

  }

  switch (filter.operator) {
    // Equal to
    case "eq":
      // Equal
      if (property === filter.value) return true;
      break;
    // Not equal to
    case "neq":
      if (property !== filter.value) return true;
      break;
    // Greater than
    case "gt":
      if (property > filter.value) return true;
      break;
    // Greater than or equal to
    case "gteq":
      if (property >= filter.value) return true;
      break;
    // Less than
    case "lt":
      if (property <= filter.value) return true;
      break;
    // Less than or equal to
    case "lteq":
      if (property < filter.value) return true;
      break;
    // In array
    case "in":
      if (property === null) return false;
      if (Array.isArray(filter.value) && Array.isArray(property))
        return property.some((item) => filter.value.includes(item));
      else if (property.includes(filter.value)) return true;
      break;
    // Not in array
    case "nin":
      if (property === null) return false;
      if (Array.isArray(filter.value) && Array.isArray(property))
        return !property.some((item) => filter.value.includes(item));
      else if (!property.includes(filter.value)) return true;
      break;
    // Contains
    case "like":
      if (property === null) return false;
      if (Array.isArray(filter.value) && Array.isArray(property))
        return property.some((item) => filter.value.some((fItem: any) => item.includes(fItem)));
      else if (Array.isArray(filter.value) && !Array.isArray(property))
        return filter.value.some((fItem: any) => property.includes(fItem));
      else if (Array.isArray(property) && !Array.isArray(filter.value))
        return property.some((fItem: any) => fItem.includes(filter.value));
      else if (property.includes(filter.value)) return true;
      break;
    // Not contains
    case "nlike":
      if (property === null) return false;
      if (Array.isArray(filter.value) && Array.isArray(property))
        return !property.some((item) => filter.value.some((fItem: any) => item.includes(fItem)));
      else if (Array.isArray(filter.value) && !Array.isArray(property))
        return !filter.value.some((fItem: any) => property.includes(fItem));
      else if (Array.isArray(property) && !Array.isArray(filter.value))
        return !property.some((fItem: any) => fItem.includes(filter.value));
      else if (!property.includes(filter.value)) return true;
      break;
    // Is null
    case "isnull":
      if (property === null) return true;
      break;
    // Not null
    case "isnotnull":
      if (property !== null) return true;
      break;
    // Between two values
    case "between":
      if (filter.value.length !== 2) return false;
      if (property > filter.value[0] && property < filter.value[1]) return true;
      break;
    // Not between two values
    case "nbetween":
      if (filter.value.length !== 2) return false;
      if (property < filter.value[0] || property > filter.value[1]) return true;
      break;
    // Empty array
    case "empty":
      if (property.length === 0) return true;
      break;
    // Non-empty array
    case "nempty":
      if (property.length > 0) return true;
      break;
    default:
      break;
  }
  return false;
};
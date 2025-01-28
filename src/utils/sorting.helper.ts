export enum SortDirection {
  ASC = "asc",
  DESC = "desc",
}

export type Sort = {
  field: string;
  direction: SortDirection | "asc" | "desc";
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

const SortValue = (valueA: any, valueB: any, direction: "asc" | "desc"): number => {
  if (valueA == undefined && valueB == undefined) return -1;

  if (Array.isArray(valueA) && Array.isArray(valueB)) {
    if (valueA.length === 0 && valueB.length === 0) return 0;
    if (valueA.length === 0) return -1;
    if (valueB.length === 0) return 1;
    return SortValue(valueA[0], valueB[0], direction);
  }

  if (valueA === valueB) return 0;
  if (direction === "asc") return valueA < valueB ? -1 : 1;
  return valueA < valueB ? 1 : -1;
};

export const SortItems = <T>(items: T[], sort: Sort): T[] => {
  if (!sort) return items;
  return items.sort((a, b) => {
    let valueA = GetNestedValue(a, sort.field);
    let valueB = GetNestedValue(b, sort.field);
    return SortValue(valueA, valueB, sort.direction);
  });
};

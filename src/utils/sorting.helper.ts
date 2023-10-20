import { SortingField } from "$types/index";

export const validateSortParameter = (searchParams: SortingField[]): string | null => {
  if (!Array.isArray(searchParams))
    return "Sorting is not an array";

  for (let index = 0; index < searchParams.length; index++) {
    const element = searchParams[index];
    if (!element.field)
      return `Sorting[${index}].field is not defined`;
    if (element.direction && element.direction !== 'asc' && element.direction !== 'desc')
      return `Sorting[${index}].direction is not valid (asc or desc)`;
  }
  return null;
}

export const convertSortingToParams = (params: URLSearchParams, sorting: SortingField[]): URLSearchParams => {
  if (sorting) {
    for (let index = 0; index < sorting.length; index++) {
      const element = sorting[index];
      params.append(`sort[${index}][field]`, element.field);
      params.append(`sort[${index}][direction]`, element.direction || "asc");
    }
  }
  return params;
};

export const sortArray = <T extends any[]>(sortings: Array<SortingField>, array: T): T => {
  if (!Array.isArray(sortings))
    throw new Error("Sorting is not an array.");

  if (sortings.length === 0)
    return array;
  const sortedArray = array.sort((a: any, b: any) => {
    let result = 0;
    for (let i = 0; i < sortings.length; i++) {
      const sorting = sortings[i];
      let propertyA = a[sorting.field];
      let propertyB = b[sorting.field];
      if (sorting.field.includes(".")) {
        const propertys = sorting.field.split(".");
        propertyA = a[propertys[0]];
        propertyB = b[propertys[0]];
        for (let i = 1; i < propertys.length; i++) {
          if (Array.isArray(propertyA) || propertyA.length === 0) {
            propertyA = propertyA.map((item: any) => item[propertys[i]]);
            propertyB = propertyB.map((item: any) => item[propertys[i]]);
          }
          else {
            propertyA = propertyA[propertys[i]];
            propertyB = propertyB[propertys[i]];
          }
          if (propertyA === undefined || propertyB === undefined) return 0;
        }
      }
      if (propertyA == undefined || propertyB == undefined)
        return -1;

      if (Array.isArray(propertyA) && Array.isArray(propertyB)) {
        propertyA = propertyA.length;
        propertyB = propertyB.length;
      }
      if (sorting.direction === 'asc' ? propertyA > propertyB : propertyA < propertyB) {
        result = 1;
        break;
      }
      if (sorting.direction === 'asc' ? propertyA < propertyB : propertyA > propertyB) {
        result = -1;
        break;
      }
    }
    return result;
  });
  return sortedArray;
}